use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Error, QueueDeclareOptions, Result};

use crate::thread_pool::ThreadPool;

pub struct PaymentQueue<'a> {
    url: &'a str,
    queue_name: &'a str,
    thread_pool: ThreadPool<String>,
}

impl<'a> PaymentQueue<'a> {
    pub fn new(
        url: &'a str,
        queue_name: &'a str,
        thread_pool: ThreadPool<String>,
    ) -> Self {
        Self {
            url,
            queue_name,
            thread_pool,
        }
    }

    pub fn lister<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce(String) + Send + 'static + Copy,
    {
        let mut conn = Connection::insecure_open(self.url)?;
        let channel = conn.open_channel(None)?;
        let queue = channel.queue_declare(self.queue_name, QueueDeclareOptions::default())?;
        let consumer = queue.consume(ConsumerOptions::default())?;
        for message in consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    self.thread_pool
                        .execute(String::from_utf8(delivery.body).unwrap(), f);
                },
                _ => {
                    println!("Consumer ended");
                    break;
                }
            }
        }
        drop(&self.thread_pool);
        conn.close()?;
        Ok(())
    }
}
