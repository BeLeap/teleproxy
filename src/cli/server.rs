use clap::Args;
use pingora::prelude::*;

#[derive(Args)]
pub struct ServerArgs {
    #[arg(short, long)]
    target: String,
}

pub fn handler(_args: &ServerArgs) {
    let mut proxy = Server::new(None).unwrap();
    proxy.bootstrap();
    proxy.run_forever();
}
