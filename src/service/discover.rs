use crate::cli::{Cli, Command};
use log::{error, debug};
pub fn run(args: Cli) {
    match args.cmd {
        Command::Discover(args) => {

        }
        _ => {
            error!("Fatal error, should not be getting anything other than the discover command here.");
        }
    }
}
