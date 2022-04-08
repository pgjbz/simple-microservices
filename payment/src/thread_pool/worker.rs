use std::{thread, sync::{Arc, Mutex, mpsc}, marker::PhantomData};

use super::Message;

pub struct Worker<T: Send + 'static> {
    _id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
    _marker: PhantomData<T>
}

impl<T> Worker<T> 
where T: Send {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<T>>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap(); //while let (and if let and match) does not drop temporary values until the end of the associated block.

            match message {
                Message::NewJob(message, job) => {
                    println!("Worker {} got a job; executing.", id);
                    job(message);
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });
        Self {
            _id: id,
            thread: Some(thread),
            _marker: PhantomData::default()
        }
    }
}