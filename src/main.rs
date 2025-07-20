mod service;
mod util;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use service::{cli, discover, receive, send};

pub fn main() {
    let args = cli::Cli::parse();
    Builder::new()
        .filter_level(if args.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        })
        .init();
    match args.cmd {
        cli::Command::Discover(_) => {
            discover::run(args);
        }
        cli::Command::Receive(_) => {
            receive::run(args);
        }
        cli::Command::Send(_) => {
            send::run(args);
        }
    }
}
