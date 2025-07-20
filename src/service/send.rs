use crate::cli;
use std::fs;

pub fn run(args: cli::Args) {
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
}
