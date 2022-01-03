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
        Some(Commands::Encode(sub_args)) => {
            println!("encode {}", sub_args.filename);
        }
        Some(Commands::Decode(sub_args)) => {
            println!("decode");
        }
        Some(Commands::Remove(sub_args)) => {
            println!("remove");
        }
        Some(Commands::Print(sub_args)) => {
            println!("print");
        }
        None => {}
    }
}
