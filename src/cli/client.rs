use clap::Args;

#[derive(Args)]
pub struct ClientArgs {
    #[arg(short, long)]
    proxy_address: String,
    #[arg(short, long)]
    target_address: String,
}

pub fn handler(_args: &ClientArgs) {}
