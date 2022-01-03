mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Args, Commands};
use clap::Parser;
use std::process;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Some(Commands::Encode(sub_args)) => commands::encode(sub_args),
        Some(Commands::Decode(sub_args)) => commands::decode(sub_args),
        Some(Commands::Remove(sub_args)) => commands::remove(sub_args),
        Some(Commands::Print(sub_args)) => commands::print(sub_args),
        None => Err("Invalid command".into()),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(1);
    }
}
