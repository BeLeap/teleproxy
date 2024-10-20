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

enum LogFormat {
    JSON,
    PRETTY,
}

fn main() {
    let format = match env::var("RUST_LOG_FORMAT") {
        Ok(format) => match format.as_str() {
            "json" => LogFormat::JSON,
            f => {
                eprintln!("unsupported log format {} fallback to pretty", f);
                LogFormat::PRETTY
            }
        },
        Err(_) => LogFormat::PRETTY,
    };

    match format {
        LogFormat::JSON => {
            tracing_subscriber::fmt().json().init();
        }
        LogFormat::PRETTY => {
            tracing_subscriber::fmt().pretty().init();
        }
    }

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::Client(args) => cli::client::handler(args),
        cli::Command::Server(args) => cli::server::handler(args),
    }
}
