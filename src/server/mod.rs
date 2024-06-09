use std::{sync::Arc, thread::sleep, time::Duration};

use crate::forwardconfig::store::ForwardConfigStore;

pub mod teleproxy {
    tonic::include_proto!("teleproxy");
}

pub fn run(port: u16, _config: Arc<ForwardConfigStore>) {
    loop {
        println!("server running on {}", port);
        sleep(Duration::from_millis(1000));
    }
}
