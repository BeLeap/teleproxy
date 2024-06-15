mod cli;
mod dto;
mod forwardconfig;
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
        cli::Command::Client(_args) => {}
        cli::Command::Server(args) => cli::server::handler(args),
    }
}
