use std::{
    marker::PhantomData,
    sync::{mpsc, Arc, Mutex},
};

use self::worker::Worker;

mod worker;
type Job<T> = Box<dyn FnOnce(T) + Send + 'static>;

pub enum Message<T> {
    NewJob(T, Job<T>),
    Terminate,
}

pub struct ThreadPool<T: Send + 'static> {
    workers: Vec<Worker<T>>,
    sender: mpsc::Sender<Message<T>>,
    _marker: PhantomData<T>,
}

impl<T> ThreadPool<T>
where
    T: Send + 'static,
{
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender,
            _marker: PhantomData::default(),
        }
    }

    pub fn execute<F>(&self, message: T, f: F)
    where
        F: FnOnce(T) + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(error) = self.sender.send(Message::NewJob(message, job)) {
            eprintln!("{}", error)
        }
    }
}

impl<T> Drop for ThreadPool<T>
where
    T: Send,
{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutdown all workers");

        for worker in &mut self.workers {
            println!("Sending terminate message to all workers.");
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
