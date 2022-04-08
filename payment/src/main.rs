use std::env;

use payment::{queues::payment_queue::PaymentQueue, thread_pool::ThreadPool, models::order::Order};

fn main() {
    let rabbitmq_host = env::var("RABBITMQ_HOST").unwrap_or("localhost".to_string());
    let rabbitmq_username = env::var("RABBITMQ_USERNAME").unwrap_or("guest".to_string());
    let rabbitmq_password = env::var("RABBITMQ_PASSWORD").unwrap_or("guest".to_string());
    let rabbitmq_queue_payment =
        env::var("RABBITMQ_QUEUE_PAYMENT").unwrap_or("payment_queue".to_string());
    let rabbitmq_port: usize = env::var("RABBITMQ_PORT")
        .map(|val| val.parse().unwrap_or(5672))
        .unwrap_or(5672);

    let dns =
        format!("amqp://{rabbitmq_username}:{rabbitmq_password}@{rabbitmq_host}:{rabbitmq_port}");

    let thread_pool = ThreadPool::new(5);
    let payment_queue = PaymentQueue::new(&dns, &rabbitmq_queue_payment, thread_pool);
    payment_queue
        .lister(listen_payment)
        .unwrap();
}

fn listen_payment(message: String) {
    let order: Order = match serde_json::from_str(&message) {
        Ok(value) => value,
        _ => return,
    };

    println!("{:?}", order);
}