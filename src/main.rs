extern crate log;
extern crate simple_logger;

use log::{trace};
use service_bus::{LogService,ServiceBus};
use ra_common::models::{Envelope, Route};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting RA Service Bus...");
    let from: u8 = 11;

    let mut bus = ServiceBus::new(String::from("ra"));
    let log_service = LogService::new();
    let service = Box::new(log_service);
    let service_id = bus.register(service);
    trace!("service registered (id: {})", service_id);
    for n in 1..10 {
        let mut env = Envelope::new(from, service_id, format!("Hello World {}: {}", service_id, n).into_bytes());
        env.slip.add_route(Route::new(service_id, 1));
        bus.send(env);
    }
    trace!("unregister of service {} {}", service_id, if bus.unregister(service_id) { "successful"} else { "unsuccessful"});
    trace!("Service Bus Stopped.");
}
