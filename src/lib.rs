extern crate log;

use log::{info,warn};
use seda_bus::{MessageBus};
use std::collections::HashMap;
use ra_common::models::Envelope;


trait Service {
    fn operate(&mut self, operation: u8, env: Envelope);
}

pub struct LogService {

}

impl LogService {
    pub fn new() -> LogService {
        LogService {}
    }
    fn log(&mut self, env: Envelope) {
        info!("{}", String::from_utf8(env.msg).unwrap())
    }
}

impl Service for LogService {
    fn operate(&mut self, operation: u8, env: Envelope) {
        match operation {
            1 => self.log(env),
            _ => warn!("Unsupported op {}",operation)
        }
    }
}

pub struct ServiceBus {
    m_bus: MessageBus,
    services: HashMap<u8, Box<dyn Service>>
}

impl ServiceBus {
    pub fn new() -> ServiceBus {
        ServiceBus {
            m_bus: MessageBus::new(),
            services: HashMap::new()
        }
    }
    pub fn register(&mut self, service: Box<dyn Service>) -> u8 {
        let id :u8 = self.m_bus.register();
        self.services.insert(id, service);
        id
    }
    pub fn send(&mut self, mut env: Envelope) -> bool {
        match env.slip.end_route() {
            Some(route) => {
                match self.services.get(&route.service) {
                    Some(mut service) => {
                        service.operate(route.op, env);
                        true
                    },
                    None => false
                }
            },
            None => {
                warn!("{}", "No current route.");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
