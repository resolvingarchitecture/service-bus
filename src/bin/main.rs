extern crate log;
extern crate simple_logger;

use log::{trace,info};
use std::thread;
use std::time::Duration;
use service_bus::{LogService,ServiceBus};
use ra_common::models::Envelope;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Service Bus...");
    let from: u8 = 11;

    let mut bus = ServiceBus::new();
    let mut log_service = LogService::new();
    let mut service = Box::new(log_service);
    let service_id = bus.register(service);

    for n in 1..10 {
        let env = Envelope::new(from, service_id, format!("Hello World {}: {}", service_id, n).into_bytes());
        bus.send(env);
    }

    thread::spawn( move || {
        loop {
            match bus.poll(addr) {
                Some(env) => info!("env to={} msg={}", env.to, env.msg),
                None => info!("x")
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    trace!("Service Bus Stopped.");
}
