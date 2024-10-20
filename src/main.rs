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
                eprintln!("unsupported log format '{}' fallback to pretty", f);
                LogFormat::PRETTY
            }
        },
        Err(_) => LogFormat::PRETTY,
    };
    let filter = tracing_subscriber::EnvFilter::from_default_env();

    match format {
        LogFormat::JSON => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .json()
                .init();
        }
        LogFormat::PRETTY => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .pretty()
                .init();
        }
    }

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::Client(args) => cli::client::handler(args),
        cli::Command::Server(args) => cli::server::handler(args),
    }
}
