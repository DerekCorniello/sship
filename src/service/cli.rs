use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(name = "sship", about = "Secure P2P file transfers over SSH")]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Send(SendArgs),
    Receive(ReceiveArgs),
    Discover(DiscoverArgs),
}

#[derive(Args, Debug)]
pub struct SendArgs {
    pub path: String,
}

#[derive(Args, Debug)]
pub struct ReceiveArgs {
    pub code: String,

    #[arg(short, long)]
    pub rename: Option<String>,
}

#[derive(Args, Debug)]
pub struct DiscoverArgs {}
