use crate::cli::{Cli, Command};
pub fn run(args: Cli) {
    match args.cmd {
        Command::Discover(args) => {

        }
        _ => {
            println!("Fatal error, should not be getting anything other than the discover command here.");
        }
    }
}
