//! Pack command - Create Engram archives

use anyhow::{Context, Result};
use engram_rs::{ArchiveWriter, CompressionMethod};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::crypto::keys::KeyPair;
use crate::manifest::builder::TomlManifest;
use crate::utils::{compression::parse_compression, paths::normalize_path};

pub fn pack(
    source_path: &Path,
    output_path: Option<&Path>,
    compression_str: &str,
    manifest_path: Option<&Path>,
    sign_key_path: Option<&Path>,
    encrypt: bool,
    encrypt_per_file: bool,
) -> Result<()> {
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

    // Parse compression method
    let _compression = parse_compression(compression_str)?;

    // Create archive writer
    let mut writer = ArchiveWriter::create(&output)
        .with_context(|| format!("Failed to create archive `{}`", output.display()))?;

    // Handle encryption
    if encrypt && encrypt_per_file {
        anyhow::bail!("Cannot use both --encrypt and --encrypt-per-file");
    }

    // TODO: Implement encryption support
    if encrypt || encrypt_per_file {
        println!("Warning: Encryption not yet implemented in this version");
    }

    // Add manifest if provided
    if let Some(manifest_file) = manifest_path {
        let toml_manifest = TomlManifest::load(manifest_file)?;
        let engram_manifest = toml_manifest.to_engram_manifest();

        // Add manifest.json to archive
        let manifest_json = serde_json::to_vec_pretty(&engram_manifest)?;
        writer.add_file_with_compression(
            "manifest.json",
            &manifest_json,
            CompressionMethod::None,
        )?;
        println!("  Added: manifest.json");
    }

    // Add files
    let metadata = fs::metadata(source_path)
        .with_context(|| format!("Failed to read metadata for `{}`", source_path.display()))?;

    let file_count = if metadata.is_file() {
        // Pack a single file
        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file name")?;

        writer
            .add_file_from_disk(file_name, source_path)
            .with_context(|| format!("Failed to add file `{}`", source_path.display()))?;

        println!("  Added: {}", file_name);
        1
    } else if metadata.is_dir() {
        // Pack a directory recursively
        let mut count = 0;

        for entry in WalkDir::new(source_path)
            .follow_links(false)
            .sort_by_file_name()
        {
            let entry = entry.with_context(|| {
                format!(
                    "Failed to read directory entry in `{}`",
                    source_path.display()
                )
            })?;

            if entry.file_type().is_file() {
                // Get relative path and normalize separators
                let relative_path = entry
                    .path()
                    .strip_prefix(source_path)
                    .with_context(|| {
                        format!(
                            "Failed to get relative path for `{}`",
                            entry.path().display()
                        )
                    })?
                    .to_str()
                    .context("Invalid file path")?;

                let archive_path = normalize_path(relative_path);

                writer
                    .add_file_from_disk(&archive_path, entry.path())
                    .with_context(|| format!("Failed to add file `{}`", entry.path().display()))?;

                println!("  Added: {}", archive_path);
                count += 1;
            }
        }

        count
    } else {
        anyhow::bail!(
            "Path is neither a file nor a directory: {}",
            source_path.display()
        );
    };

    // Finalize the archive
    writer
        .finalize()
        .with_context(|| format!("Failed to finalize archive `{}`", output.display()))?;

    println!("Packed {} files", file_count);

    // Sign if key provided
    if let Some(key_path) = sign_key_path {
        println!("Signing archive...");
        let _keypair = KeyPair::load_private(key_path)?;

        // TODO: Implement signing
        println!("Warning: Signing not yet fully implemented");
    }

    println!("Archive created successfully: {}", output.display());

    Ok(())
}
