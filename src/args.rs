// TODO consider renaming this module to cli

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode message into png
    Encode(EncodeArgs),

    /// Decode message from png
    Decode(DecodeArgs),

    /// Remove message from png
    Remove(RemoveArgs),

    /// Print message from png
    Print(PrintArgs),
}

#[derive(clap::Args)]
pub struct EncodeArgs {
    pub filename: String,
    pub chunk_type: String,
    pub message: String,
}

#[derive(clap::Args)]
pub struct DecodeArgs {
    pub filename: String,
    pub chunk_type: String,
}

#[derive(clap::Args)]
pub struct RemoveArgs {
    pub filename: String,
    pub chunk_type: String,
}

#[derive(clap::Args)]
pub struct PrintArgs {
    pub filename: String,
}
