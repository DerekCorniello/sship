use crate::cli::{Cli, Command};
use std::fs;

pub fn run(args: Cli) {
    match args.cmd {
        Command::Send(args) => {
            let path = args.path;
            let is_dir = match fs::metadata(&path) {
                Ok(meta) if meta.is_file() => false,
                Ok(meta) if meta.is_dir() => true,
                Ok(meta) => {
                    eprintln!("Unsupported file type: '{:?}'", meta.file_type());
                    return;
                }
                Err(e) => {
                    eprintln!("Error accessing '{}': {}", path, e);
                    return;
                }
            };

            if args.verbose {
                println!("[send] '{}' is a {}", path, if is_dir { "directory" } else { "file" });
            }
        }
        _ => {
            println!("Fatal error, should not be getting anything other than the send command here.");
        }
    }
}

