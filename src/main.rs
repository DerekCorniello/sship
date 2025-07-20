mod service;
use service::{send, receive, discover, cli};
use clap::Parser;

pub fn main() {
    let args = cli::Args::parse();
    match args.cmd {
        cli::Commands::Discover => {
            discover::run();
        }
        cli::Commands::Receive => {
            receive::run(args);
        }
        cli::Commands::Send => {
            send::run(args);
        }
    }
}
