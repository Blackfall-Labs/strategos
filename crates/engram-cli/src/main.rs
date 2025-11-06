use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use engram_core::{ArchiveReader, ArchiveWriter};
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
            list_archive(&path)?;
        }

        Commands::Info { path, inspect } => {
            show_archive_info(&path, inspect)?;
        }

        Commands::Pack { path, output } => {
            pack_archive(&path, output.as_deref())?;
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

/// Lists all files contained within an Engram archive
fn list_archive(archive_path: &Path) -> Result<()> {
    let reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("failed to open archive `{}`", archive_path.display()))?;

    for file_path in reader.list_files() {
        println!("{}", file_path);
    }

    Ok(())
}

/// Displays metadata and statistics about an Engram archive
fn show_archive_info(archive_path: &Path, inspect: bool) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("failed to open archive `{}`", archive_path.display()))?;

    // Get header information first
    let version_major = reader.header().version_major;
    let version_minor = reader.header().version_minor;
    let entry_count = reader.header().entry_count;
    let content_version = reader.header().content_version;
    let central_directory_offset = reader.header().central_directory_offset;
    let central_directory_size = reader.header().central_directory_size;
    let header_crc = reader.header().header_crc;

    println!("Archive: {}", archive_path.display());
    println!("Format Version: {}.{}", version_major, version_minor);
    println!("Total Files: {}", entry_count);
    println!("Content Version: {}", content_version);

    let mut total_uncompressed = 0u64;
    let mut total_compressed = 0u64;

    for file_path in reader.list_files() {
        if let Some(entry) = reader.get_entry(file_path) {
            total_uncompressed += entry.uncompressed_size;
            total_compressed += entry.compressed_size;
        }
    }

    let ratio = if total_uncompressed > 0 {
        (total_compressed as f64 / total_uncompressed as f64) * 100.0
    } else {
        100.0
    };

    println!("Total Size: {} bytes", total_uncompressed);
    println!("Compressed: {} bytes ({:.1}%)", total_compressed, ratio);

    // Show manifest if present
    if let Some(manifest) = reader.read_manifest()? {
        println!("\nManifest:");
        println!("{}", serde_json::to_string_pretty(&manifest)?);
    }

    // Show detailed inspection if requested
    if inspect {
        println!("\n{:-<60}", "");
        println!("Detailed File Information:");
        println!("{:-<60}", "");

        for file_path in reader.list_files() {
            if let Some(entry) = reader.get_entry(file_path) {
                let compression_ratio = if entry.uncompressed_size > 0 {
                    (entry.compressed_size as f64 / entry.uncompressed_size as f64) * 100.0
                } else {
                    100.0
                };

                println!("\n{}", entry.path);
                println!("  Compression: {:?}", entry.compression);
                println!(
                    "  Size: {} -> {} bytes ({:.1}%)",
                    entry.uncompressed_size, entry.compressed_size, compression_ratio
                );
                println!("  CRC32: {:08X}", entry.crc32);
                println!("  Offset: {}", entry.data_offset);
                println!("  Modified: {}", entry.modified_time);
            }
        }

        println!("\n{:-<60}", "");
        println!("Archive Structure:");
        println!("{:-<60}", "");
        println!("Central Directory Offset: {}", central_directory_offset);
        println!("Central Directory Size: {}", central_directory_size);
        println!("Header CRC: {:08X}", header_crc);
    }

    Ok(())
}

/// Packs files or directories into a new Engram archive
fn pack_archive(source_path: &Path, output_path: Option<&Path>) -> Result<()> {
    // Determine output path
    let output = match output_path {
        Some(p) => p.to_path_buf(),
        None => {
            let mut default_output = source_path.to_path_buf();
            default_output.set_extension("eng");
            default_output
        }
    };

    println!("Packing: {}", source_path.display());
    println!("Output: {}", output.display());

    let mut writer = ArchiveWriter::create(&output)
        .with_context(|| format!("failed to create archive `{}`", output.display()))?;

    let metadata = fs::metadata(source_path)
        .with_context(|| format!("failed to read metadata for `{}`", source_path.display()))?;

    if metadata.is_file() {
        // Pack a single file
        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("invalid file name")?;

        writer
            .add_file_from_disk(file_name, source_path)
            .with_context(|| format!("failed to add file `{}`", source_path.display()))?;

        println!("  Added: {}", file_name);
    } else if metadata.is_dir() {
        // Pack a directory recursively
        let mut file_count = 0;

        for entry in WalkDir::new(source_path)
            .follow_links(false)
            .sort_by_file_name()
        {
            let entry = entry.with_context(|| {
                format!("failed to read directory entry in `{}`", source_path.display())
            })?;

            if entry.file_type().is_file() {
                // Get relative path and normalize separators
                let relative_path = entry
                    .path()
                    .strip_prefix(source_path)
                    .with_context(|| {
                        format!(
                            "failed to get relative path for `{}`",
                            entry.path().display()
                        )
                    })?
                    .to_str()
                    .context("invalid file path")?
                    .replace('\\', "/");

                writer
                    .add_file_from_disk(&relative_path, entry.path())
                    .with_context(|| format!("failed to add file `{}`", entry.path().display()))?;

                println!("  Added: {}", relative_path);
                file_count += 1;
            }
        }

        println!("Packed {} files", file_count);
    } else {
        anyhow::bail!("path is neither a file nor a directory: {}", source_path.display());
    }

    // Finalize the archive (writes central directory and updates header)
    writer
        .finalize()
        .with_context(|| format!("failed to finalize archive `{}`", output.display()))?;

    println!("Archive created successfully: {}", output.display());

    Ok(())
}