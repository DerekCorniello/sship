use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,

    pub path: String,

    #[arg(short, long)]
    pub rename: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Send,
    Receive,
    Discover,
}
