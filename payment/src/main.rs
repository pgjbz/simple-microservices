use std::env;

use payment::{
    models::order::{Order, Status},
    queues::queue::Queue,
    thread_pool::ThreadPool,
};
use rand::Rng;

struct RabbitEnv<'a> {
    host: &'a str,
    username: &'a str,
    password: &'a str,
    port: &'a str,
}

impl<'a> RabbitEnv<'a> {
    fn new(
        host: &'a str,
        username: &'a str,
        password: &'a str,
        port: &'a str,
    ) -> Self {
        Self {
            host,
            username,
            password,
            port,
        }
    }
}

fn main() {
    let rabbitmq_queue_payment =
        env::var("RABBITMQ_QUEUE_PAYMENT").unwrap_or("payment_queue".to_string());

    let env_props = RabbitEnv::new("RABBITMQ_HOST", "RABBITMQ_USERNAME", "RABBITMQ_PASSWORD", "RABBITMQ_PORT");
    let dns = build_dns(env_props);

    let thread_pool = ThreadPool::new(5);
    let mut payment_queue = Queue::new(&dns, Some(thread_pool));
    payment_queue.open_con().unwrap();
    payment_queue.lister(&rabbitmq_queue_payment, listen_payment).unwrap();
}

fn listen_payment(message: String) {
    println!("listen: {}", message);
    let mut order: Order = match  serde_json::from_str(&message) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{:?}", error);
            return
        }   
    };
    let random_value = rand::thread_rng().gen_range(1..=3);
    order.status = Status::from(random_value);
    {
        
        let env_props = RabbitEnv::new("RABBITMQ_HOST", "RABBITMQ_USERNAME", "RABBITMQ_PASSWORD", "RABBITMQ_PORT");
        let dns = build_dns(env_props);
        let rabbitmq_queue_order =
            env::var("RABBITMQ_QUEUE_ORDER").unwrap_or("order_queue".to_string());
        let mut queue = Queue::new(&dns, None);
        let message = serde_json::to_string(&order).unwrap();
        println!("sending message: {}", message);
        queue.open_con().unwrap();
        queue.notify(&rabbitmq_queue_order, &message).unwrap();
    }
}

fn build_dns(env_props: RabbitEnv) -> String {
    let rabbitmq_host = env::var(env_props.host).unwrap_or("localhost".to_string());
    let rabbitmq_username = env::var(env_props.username).unwrap_or("guest".to_string());
    let rabbitmq_password = env::var(env_props.password).unwrap_or("guest".to_string());
    let rabbitmq_port: usize = env::var(env_props.port)
        .map(|val| val.parse().unwrap_or(5672))
        .unwrap_or(5672);

    format!("amqp://{rabbitmq_username}:{rabbitmq_password}@{rabbitmq_host}:{rabbitmq_port}")
}
