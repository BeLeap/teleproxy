mod cli;
mod client;
mod dto;
mod forwardconfig;
mod forwardhandler;
mod proto;
mod proxy;
mod server;

use clap::Parser;
use env_logger::Env;

fn main() {
    let env = Env::default()
        .filter_or("RUST_LOG", "trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::Client(args) => cli::client::handler(args),
        cli::Command::Server(args) => cli::server::handler(args),
    }
}
