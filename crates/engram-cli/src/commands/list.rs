//! List command - List files in archives

use anyhow::{Context, Result};
use engram_rs::ArchiveReader;
use std::path::Path;

use crate::utils::compression::compression_name;

pub fn list(archive_path: &Path, long: bool, databases: bool) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("Failed to open archive `{}`", archive_path.display()))?;
    reader.initialize()?;

    // Clone the files list to avoid borrowing issues
    let all_files: Vec<String> = reader.list_files().to_vec();

    let files: Vec<String> = if databases {
        // Filter database files
        all_files
            .iter()
            .filter(|f| f.ends_with(".db") || f.ends_with(".sqlite") || f.ends_with(".sqlite3"))
            .cloned()
            .collect()
    } else {
        all_files
    };

    if files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    for file_path in files {
        if long {
            // Show detailed information
            if let Some(entry) = reader.get_entry(&file_path) {
                let compression = compression_name(entry.compression);
                let ratio = if entry.uncompressed_size > 0 {
                    (entry.compressed_size as f64 / entry.uncompressed_size as f64) * 100.0
                } else {
                    100.0
                };

                println!(
                    "{:50} {:>10} {:>10} {:>6} ({:.1}%)",
                    file_path, entry.uncompressed_size, entry.compressed_size, compression, ratio
                );
            }
        } else {
            // Simple list
            println!("{}", file_path);
        }
    }

    Ok(())
}
