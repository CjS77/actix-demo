extern crate actix;
extern crate actix_demo;

use actix::prelude::*;
use actix_demo::messages;
use std::time::Duration;

fn main() {
    System::run(|| {
        let mut oracle = messages::RandomOracle::new(Duration::from_secs(1));
        let alice = messages::OracleListener { name: "Alice".to_string(), secret: 15 }.start();
        let bob = messages::OracleListener { name: "Bob".to_string(), secret: 11 }.start();
        let charlie = messages::OracleListener { name: "Charlie".to_string(), secret: 3 }.start();
        oracle.add_listener(alice.recipient());
        oracle.add_listener(bob.recipient());
        oracle.add_listener(charlie.recipient());
        oracle.start();
    });
}
