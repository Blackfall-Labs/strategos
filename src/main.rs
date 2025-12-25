//! Strategos - Universal archive management CLI
//!
//! Strategos provides unified command-line interface for managing multiple
//! archive formats: Engram, Cartridge, DataSpool, and DataCard.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod crypto;
mod formats;
mod manifest;
mod utils;

#[derive(Parser)]
#[command(name = "strategos")]
#[command(
    about = "Universal archive management CLI for Engram, Cartridge, DataSpool, and DataCard",
    version,
    long_about = "Strategos provides a unified interface for working with multiple archive formats.\n\n\
                  Supported formats:\n  \
                  - Engram (.eng): Immutable signed archives\n  \
                  - Cartridge (.cart): Mutable page-based archives\n  \
                  - DataSpool (.spool): Append-only item collections\n  \
                  - DataCard (.card): Compressed CML documents"
)]
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

    // === Cartridge-specific commands ===
    /// Create a new Cartridge archive (mutable)
    CartridgeCreate {
        /// Slug (kebab-case identifier)
        slug: String,

        /// Title (human-readable name)
        title: String,

        /// Output path (optional, defaults to slug.cart)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Write a file to a Cartridge archive
    CartridgeWrite {
        /// Cartridge archive path
        archive: PathBuf,

        /// File path within archive
        file_path: String,

        /// Source file to write
        source: PathBuf,
    },

    /// Delete a file from a Cartridge archive
    CartridgeDelete {
        /// Cartridge archive path
        archive: PathBuf,

        /// File path to delete
        file_path: String,
    },

    /// Create a snapshot of a Cartridge
    CartridgeSnapshot {
        /// Cartridge archive path
        archive: PathBuf,

        /// Snapshot name
        #[arg(short, long)]
        name: String,

        /// Snapshot description
        #[arg(short, long)]
        description: String,

        /// Snapshot directory
        #[arg(short = 'd', long)]
        snapshot_dir: PathBuf,
    },

    // === DataSpool-specific commands ===
    /// Build a new DataSpool from card files
    DataSpoolBuild {
        /// Output spool path
        #[arg(short, long)]
        output: PathBuf,

        /// Card files to add
        cards: Vec<String>,
    },

    /// Append cards to an existing DataSpool
    DataSpoolAppend {
        /// Spool path
        spool: PathBuf,

        /// Card files to append
        cards: Vec<String>,
    },

    /// Show DataSpool index
    DataSpoolIndex {
        /// Spool path
        spool: PathBuf,
    },

    // === DataCard-specific commands ===
    /// Compress CML to DataCard
    DataCardCompress {
        /// CML input file
        cml: PathBuf,

        /// Output card path
        #[arg(short, long)]
        output: PathBuf,

        /// BytePunch dictionary path
        #[arg(short, long)]
        dict: PathBuf,

        /// Document ID
        #[arg(long)]
        id: String,

        /// Add checksum
        #[arg(long)]
        checksum: bool,
    },

    /// Decompress DataCard to CML
    DataCardDecompress {
        /// Card input file
        card: PathBuf,

        /// Output CML path
        #[arg(short, long)]
        output: PathBuf,

        /// BytePunch dictionary path
        #[arg(short, long)]
        dict: PathBuf,
    },

    /// Validate a DataCard
    DataCardValidate {
        /// Card path
        card: PathBuf,
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
            // Use new format-agnostic shared command
            commands::shared::list(&path, long, databases)?;
        }

        Commands::Info {
            path,
            inspect,
            verify,
            manifest,
        } => {
            // Use new format-agnostic shared command
            commands::shared::info(&path, inspect, verify, manifest)?;
        }

        Commands::Extract {
            archive,
            output,
            files,
            decrypt: _decrypt,
        } => {
            // Use new format-agnostic shared command
            commands::shared::extract(&archive, &output, files)?;
        }

        Commands::Verify {
            archive,
            public_key: _public_key,
            check_hashes: _check_hashes,
        } => {
            // Use new format-agnostic shared command
            commands::shared::verify(&archive)?;
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
            in_archive: _in_archive,
            case_insensitive,
        } => {
            // Use new format-agnostic shared command
            commands::shared::search(&path, &pattern, case_insensitive)?;
        }

        // Cartridge commands
        Commands::CartridgeCreate {
            slug,
            title,
            output,
        } => {
            commands::cartridge::create(&slug, &title, output.as_deref())?;
        }

        Commands::CartridgeWrite {
            archive,
            file_path,
            source,
        } => {
            commands::cartridge::write(&archive, &file_path, &source)?;
        }

        Commands::CartridgeDelete { archive, file_path } => {
            commands::cartridge::delete(&archive, &file_path)?;
        }

        Commands::CartridgeSnapshot {
            archive,
            name,
            description,
            snapshot_dir,
        } => {
            commands::cartridge::snapshot(&archive, name, description, &snapshot_dir)?;
        }

        // DataSpool commands
        Commands::DataSpoolBuild { output, cards } => {
            commands::dataspool::build(&output, &cards)?;
        }

        Commands::DataSpoolAppend { spool, cards } => {
            commands::dataspool::append(&spool, &cards)?;
        }

        Commands::DataSpoolIndex { spool } => {
            commands::dataspool::show_index(&spool)?;
        }

        // DataCard commands
        Commands::DataCardCompress {
            cml,
            output,
            dict,
            id,
            checksum,
        } => {
            commands::datacard::compress(&cml, &output, &dict, &id, checksum)?;
        }

        Commands::DataCardDecompress { card, output, dict } => {
            commands::datacard::decompress(&card, &output, &dict)?;
        }

        Commands::DataCardValidate { card } => {
            commands::datacard::validate(&card)?;
        }
    }

    Ok(())
}
