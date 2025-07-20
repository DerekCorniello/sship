mod service;
use service::{send, receive, discover, cli};
use clap::Parser;

pub fn main() {
    let args = cli::Cli::parse();
    match args.cmd {
        cli::Command::Discover(_) => {
            discover::run();
        }
        cli::Command::Receive(_) => {
            receive::run(args);
        }
        cli::Command::Send(_) => {
            send::run(args);
        }
    }
}
