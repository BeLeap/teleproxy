use clap::Args;

#[derive(Args)]
pub struct ServerArgs {
    #[arg(short, long)]
    target: String,
}
