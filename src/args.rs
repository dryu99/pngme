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
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
    },
    /// Decode message from png
    Decode {
        file_path: String,
        chunk_type: String,
    },
    /// Remove message from png
    Remove {
        file_path: String,
        chunk_type: String,
    },
    /// Print message from png
    Print { file_path: String },
}
