use crate::cli::{Cli, Command};
use std::fs;
use log::{error, debug};

pub fn run(args: Cli) {
    match args.cmd {
        Command::Send(args) => {
            let path = args.path;
            let is_dir = match fs::metadata(&path) {
                Ok(meta) if meta.is_file() => false,
                Ok(meta) if meta.is_dir() => true,
                Ok(meta) => {
                    error!("Unsupported file type: '{:?}'", meta.file_type());
                    return;
                }
                Err(e) => {
                    error!("Error accessing '{}': {}", path, e);
                    return;
                }
            };

                debug!("[send] '{}' is a {}", path, if is_dir { "directory" } else { "file" });
        }
        _ => {
            error!("Fatal error, should not be getting anything other than the send command here.");
        }
    }
}

