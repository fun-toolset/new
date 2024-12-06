use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create new directories
    Dir {
        #[arg(value_name = "DIR")]
        dirs: Vec<PathBuf>,
    },

    /// Create new files
    File {
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,
    },
}
