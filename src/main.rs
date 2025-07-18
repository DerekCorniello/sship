use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    path: String,

    #[arg(short, long)]
    rename: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Send,
    Receive,
    Scan,
}

pub fn main() {
    let args = Args::parse();
    let path = args.path;
    // TODO: make printing a CLI thing probably?
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

    match args.cmd {
        Commands::Scan => {}
        Commands::Receive => {}
        Commands::Send => {}
    }
}
