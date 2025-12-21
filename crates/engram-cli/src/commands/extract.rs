//! Extract command - Extract files from archives

use anyhow::{Context, Result};
use engram_rs::ArchiveReader;
use std::fs;
use std::path::Path;

pub fn extract(
    archive_path: &Path,
    output_dir: &Path,
    files: Option<&[String]>,
    _decrypt: bool,
) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("Failed to open archive `{}`", archive_path.display()))?;
    reader.initialize()?;

    // Create output directory
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory `{}`", output_dir.display()))?;

    // Determine which files to extract
    let files_to_extract: Vec<String> = if let Some(specific_files) = files {
        specific_files.to_vec()
    } else {
        reader.list_files().to_vec()
    };

    println!("Extracting to: {}", output_dir.display());

    for file_path in files_to_extract {
        // Read file from archive
        let data = reader.read_file(&file_path)
            .with_context(|| format!("Failed to read file `{}` from archive", file_path))?;

        // Create output path
        let output_path = output_dir.join(&file_path);

        // Create parent directories
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write file
        fs::write(&output_path, data)
            .with_context(|| format!("Failed to write file `{}`", output_path.display()))?;

        println!("  Extracted: {}", file_path);
    }

    println!("Extraction complete");

    Ok(())
}
