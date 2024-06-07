use clap::{Parser, Subcommand};
mod client;
mod server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Client(client::ClientArgs),
    Server(server::ServerArgs),
}
