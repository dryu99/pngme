mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Args, Commands};
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Encode {
            file_path,
            chunk_type,
            message,
        }) => {
            println!("encode");
        }
        Some(Commands::Decode {
            file_path,
            chunk_type,
        }) => {
            println!("decode");
        }
        Some(Commands::Remove {
            file_path,
            chunk_type,
        }) => {
            println!("remove");
        }
        Some(Commands::Print { file_path }) => {
            println!("print");
        }
        None => {}
    }
}
