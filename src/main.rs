mod dto;
mod forwardconfig;
mod proxy;
mod server;
mod cli;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::Client(_args) => {},
        cli::Command::Server(args) => cli::server::handler(args),
    }
}