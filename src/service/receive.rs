use crate::cli::{Cli, Command};
use log::{error, info};
pub fn run(args: Cli) {
    match args.cmd {
        Command::Receive(args) => {

        }
        _ => {
            error!("Fatal error, should not be getting anything other than the receive command here.");
        }
    }
}
