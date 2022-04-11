use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Error, QueueDeclareOptions, Result, Exchange, Publish};

use crate::thread_pool::ThreadPool;

pub struct Queue<'a> {
    url: &'a str,
    thread_pool: Option<ThreadPool<String>>,
    connection: Option<Connection>,
}

impl<'a> Queue<'a> {
    pub fn new(url: &'a str, thread_pool: Option<ThreadPool<String>>) -> Self {
        Self {
            url,
            thread_pool,
            connection: None,
        }
    }

    pub fn open_con(&mut self) -> Result<(), Error> {
        let conn = Connection::insecure_open(self.url)?;
        self.connection = Some(conn);
        Ok(())
    }

    pub fn lister<F>(&mut self, queue_name: &'a str, f: F) -> Result<(), Error>
    where
        F: FnOnce(String) + Send + 'static + Copy,
    {
        let conn = if let Some(ref mut connection) = self.connection {
            connection
        } else {
            return Err(Error::ClientException);
        };
        let channel = conn.open_channel(None)?;
        let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;
        let consumer = queue.consume(ConsumerOptions::default())?;
        for message in consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    if let Some(ref thread_pool) = self.thread_pool {
                        thread_pool
                            .execute(String::from_utf8(delivery.body).unwrap(), f);
                    } else {
                        f(String::from_utf8(delivery.body).unwrap());
                    }
                }
                _ => {
                    println!("Consumer ended");
                    break;
                }
            }
        }
        drop(&self.thread_pool);
        Ok(())
    }

    pub fn notify(&mut self, routing_key: &str, message: &str) -> Result<(), Error>{
        let conn = if let Some(ref mut connection) = self.connection {
            connection
        } else {
            return Err(Error::ClientException);
        };
        let channel = conn.open_channel(None)?;
        let exchange = Exchange::direct(&channel);
        exchange
                .publish(Publish::new(message.as_bytes(), routing_key))?;
        Ok(())
    }
}
