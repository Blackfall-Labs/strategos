use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "engram")]
#[command(about = "A CLI tool for managing Engram archives", long_about = None)]
struct CliArguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List files from inside an Engram
    #[command(alias = "ls")]
    List {
        /// Path to the Engram file
        path: PathBuf,
    },

    /// Show information about an Engram
    #[command(alias = "i")]
    Info {
        /// Path to the Engram file
        path: PathBuf,

        /// Show additional detailed information
        #[arg(long)]
        inspect: bool,
    },

    /// Pack files/directory into a new Engram
    #[command(alias = "p")]
    Pack {
        /// Path to pack
        path: PathBuf,

        /// Output Engram file (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Search for a pattern in a file
    Search {
        /// Pattern to search for
        pattern: String,

        /// Path to the file
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    match args.command {
        Commands::List { path } => {
            println!("Listing files from Engram: {}", path.display());
            // unimplemented!()
        }

        Commands::Info { path, inspect } => {
            println!("Information about Engram: {}", path.display());
            if inspect {
                println!("Showing detailed inspection...");
            }
            // unimplemented!()
        }

        Commands::Pack { path, output } => {
            println!("Packing: {}", path.display());
            if let Some(out) = output {
                println!("Output to: {}", out.display());
            }
            // unimplemented!()
        }

        Commands::Search { pattern, path } => {
            let content = read_to_string(&path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            find_matches(&content, &pattern, &mut std::io::stdout())
                .with_context(|| format!("failed to find matching content for pattern `{}`", pattern))?;
            
            // unimplemented!()
        }
    }

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}