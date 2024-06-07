mod cli;

use clap::Parser;
use pingora::prelude::*;

fn main() {
    let cli = cli::Cli::parse();

    let mut proxy = Server::new(None).unwrap();
    proxy.bootstrap();
    proxy.run_forever();
}
