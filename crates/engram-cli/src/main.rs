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
    /// Lists all files and directories contained within an Engram archive
    #[command(alias = "ls")]
    List {
        /// Path to the Engram archive file
        path: PathBuf,
    },

    /// Displays metadata and statistics about an Engram archive
    #[command(alias = "i")]
    Info {
        /// Path to the Engram archive file
        path: PathBuf,

        /// Display detailed inspection data including internal structure
        #[arg(long)]
        inspect: bool,
    },

    /// Packs files or directories into a new Engram archive
    #[command(alias = "p")]
    Pack {
        /// Path to the file or directory to pack into an archive
        path: PathBuf,

        /// Path for the output archive (defaults to input name with .engram extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Searches for a text pattern within a file and prints matching lines
    Search {
        /// Text pattern to search for in the file
        pattern: String,

        /// Path to the file to search
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

/// Searches through content line-by-line and writes lines containing the pattern to the writer
///
/// # Arguments
/// * `content` - The text content to search through
/// * `pattern` - The substring pattern to match
/// * `writer` - Output destination for matching lines
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}