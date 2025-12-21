//! Engram CLI - Command-line tool for managing Engram archives

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod crypto;
mod manifest;
mod utils;

#[derive(Parser)]
#[command(name = "engram")]
#[command(about = "A CLI tool for managing Engram archives", version, long_about = None)]
struct CliArguments {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Engram archive from files or directories
    #[command(alias = "p")]
    Pack {
        /// Path to the file or directory to pack
        path: PathBuf,

        /// Output archive path (defaults to input name with .eng extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Compression method: none, lz4, zstd
        #[arg(short, long, default_value = "lz4")]
        compression: String,

        /// Manifest file (manifest.toml)
        #[arg(short, long)]
        manifest: Option<PathBuf>,

        /// Private key for signing
        #[arg(short = 'k', long)]
        sign_key: Option<PathBuf>,

        /// Encrypt archive with password
        #[arg(long)]
        encrypt: bool,

        /// Encrypt each file individually with password
        #[arg(long)]
        encrypt_per_file: bool,
    },

    /// List files in an Engram archive
    #[command(alias = "ls")]
    List {
        /// Path to the Engram archive
        path: PathBuf,

        /// Show detailed information (size, compression, dates)
        #[arg(short, long)]
        long: bool,

        /// List only database files (.db, .sqlite)
        #[arg(short = 'd', long)]
        databases: bool,
    },

    /// Display metadata and statistics about an archive
    #[command(alias = "i")]
    Info {
        /// Path to the Engram archive
        path: PathBuf,

        /// Display detailed inspection data
        #[arg(long)]
        inspect: bool,

        /// Verify signatures and file hashes
        #[arg(long)]
        verify: bool,

        /// Show manifest only
        #[arg(short, long)]
        manifest: bool,
    },

    /// Extract files from an Engram archive
    #[command(alias = "x")]
    Extract {
        /// Path to the Engram archive
        archive: PathBuf,

        /// Output directory for extracted files
        #[arg(short, long)]
        output: PathBuf,

        /// Extract only specific files
        #[arg(short, long)]
        files: Option<Vec<String>>,

        /// Decrypt encrypted archive with password
        #[arg(long)]
        decrypt: bool,
    },

    /// Verify archive signatures and integrity
    Verify {
        /// Path to the Engram archive
        archive: PathBuf,

        /// Public key for signature verification
        #[arg(short = 'k', long)]
        public_key: Option<PathBuf>,

        /// Check file hashes from manifest
        #[arg(long)]
        check_hashes: bool,
    },

    /// Sign an Engram archive
    Sign {
        /// Path to the Engram archive
        archive: PathBuf,

        /// Private key for signing
        #[arg(short = 'k', long)]
        private_key: PathBuf,

        /// Signer identity
        #[arg(short, long)]
        signer: Option<String>,
    },

    /// Generate a new Ed25519 keypair
    Keygen {
        /// Output path for private key
        #[arg(short = 'r', long)]
        private_key: PathBuf,

        /// Output path for public key
        #[arg(short = 'u', long)]
        public_key: PathBuf,
    },

    /// Query SQLite databases within an archive
    #[command(alias = "q")]
    Query {
        /// Path to the Engram archive
        archive: PathBuf,

        /// List all databases in archive
        #[arg(short, long)]
        list_databases: bool,

        /// Database file path within archive
        #[arg(short, long)]
        database: Option<String>,

        /// SQL query to execute
        #[arg(short, long)]
        sql: Option<String>,

        /// Output format: json, csv, table
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Search for text patterns in files
    Search {
        /// Text pattern to search for
        pattern: String,

        /// Path to file or archive
        path: PathBuf,

        /// Search inside archive files
        #[arg(long)]
        in_archive: bool,

        /// Case-insensitive search
        #[arg(short, long)]
        case_insensitive: bool,
    },
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    // Initialize logging
    if args.verbose {
        tracing_subscriber::fmt::init();
    }

    match args.command {
        Commands::Pack {
            path,
            output,
            compression,
            manifest,
            sign_key,
            encrypt,
            encrypt_per_file,
        } => {
            commands::pack::pack(
                &path,
                output.as_deref(),
                &compression,
                manifest.as_deref(),
                sign_key.as_deref(),
                encrypt,
                encrypt_per_file,
            )?;
        }

        Commands::List {
            path,
            long,
            databases,
        } => {
            commands::list::list(&path, long, databases)?;
        }

        Commands::Info {
            path,
            inspect,
            verify,
            manifest,
        } => {
            commands::info::info(&path, inspect, verify, manifest)?;
        }

        Commands::Extract {
            archive,
            output,
            files,
            decrypt,
        } => {
            commands::extract::extract(&archive, &output, files.as_deref(), decrypt)?;
        }

        Commands::Verify {
            archive,
            public_key,
            check_hashes,
        } => {
            commands::verify::verify(&archive, public_key.as_deref(), check_hashes)?;
        }

        Commands::Sign {
            archive,
            private_key,
            signer,
        } => {
            commands::sign::sign(&archive, &private_key, signer.as_deref())?;
        }

        Commands::Keygen {
            private_key,
            public_key,
        } => {
            commands::keygen::keygen(&private_key, &public_key)?;
        }

        Commands::Query {
            archive,
            list_databases,
            database,
            sql,
            format,
        } => {
            commands::query::query(
                &archive,
                list_databases,
                database.as_deref(),
                sql.as_deref(),
                &format,
            )?;
        }

        Commands::Search {
            pattern,
            path,
            in_archive,
            case_insensitive,
        } => {
            commands::search::search(&pattern, &path, in_archive, case_insensitive)?;
        }
    }

    Ok(())
}
