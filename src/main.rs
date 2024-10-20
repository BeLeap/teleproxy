mod cli;
mod client;
mod config;
mod dto;
mod forwardconfig;
mod forwardhandler;
mod proto;
mod proxy;
mod server;

use std::env;

use clap::Parser;

fn main() {
    match env::var("RUST_LOG_FORMAT") {
        Ok(format) => match format.as_str() {
            "json" => {
                tracing_subscriber::fmt().json().init();
            }
            _ => {
                tracing_subscriber::fmt().compact().init();
            }
        },
        Err(_) => {
            tracing_subscriber::fmt().compact().init();
        }
    };

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::Client(args) => cli::client::handler(args),
        cli::Command::Server(args) => cli::server::handler(args),
    }
}
