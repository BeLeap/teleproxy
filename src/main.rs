use pingora::prelude::*;

fn main() {
    let mut proxy = Server::new(None).unwrap();
    proxy.bootstrap();
    proxy.run_forever();
}
