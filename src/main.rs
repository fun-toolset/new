mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Dir { dirs } => {
            utils::create_directories(dirs)?;
        }

        cli::Commands::File { files } => {
            utils::create_files(files)?;
        }
    }

    Ok(())
}
