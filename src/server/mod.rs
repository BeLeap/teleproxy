use std::{thread::sleep, time::Duration};

pub mod teleproxy {
    tonic::include_proto!("teleproxy");
}

pub fn run(port: u16) {
    loop {
        println!("server running on {}", port);
        sleep(Duration::from_millis(1000));
    }
}
