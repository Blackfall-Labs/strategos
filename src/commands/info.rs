//! Info command - Display archive metadata

use anyhow::{Context, Result};
use engram_rs::ArchiveReader;
use std::path::Path;

use crate::utils::compression::compression_name;

pub fn info(archive_path: &Path, inspect: bool, verify: bool, manifest_only: bool) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("Failed to open archive `{}`", archive_path.display()))?;
    reader.initialize()?;

    // Show manifest only
    if manifest_only {
        if let Ok(Some(manifest)) = reader.read_manifest() {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        } else {
            println!("No manifest found");
        }
        return Ok(());
    }

    // Get header information (clone to avoid borrow conflicts)
    let header = reader.header().clone();
    println!("Archive: {}", archive_path.display());
    println!(
        "Format Version: {}.{}",
        header.version_major, header.version_minor
    );
    println!("Total Files: {}", header.entry_count);
    println!("Content Version: {}", header.content_version);

    // Calculate totals
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
    if let Ok(Some(manifest_value)) = reader.read_manifest()
        && let Ok(manifest) = serde_json::from_value::<engram_rs::Manifest>(manifest_value.clone())
    {
        println!("\nManifest:");
        println!("  ID: {}", manifest.id);
        println!("  Name: {}", manifest.name);
        println!("  Version: {}", manifest.metadata.version);
        println!("  Author: {}", manifest.author.name);
        if !manifest.signatures.is_empty() {
            println!("  Signatures: {}", manifest.signatures.len());
        }

        // Verify signatures
        if verify {
            println!("\nVerifying signatures...");
            if manifest.signatures.is_empty() {
                println!("  No signatures found");
            } else {
                match manifest.verify_signatures() {
                    Ok(results) => {
                        for (i, valid) in results.iter().enumerate() {
                            if *valid {
                                println!("  ✓ Signature {} valid", i + 1);
                            } else {
                                println!("  ✗ Signature {} invalid", i + 1);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  Error verifying signatures: {}", e);
                    }
                }
            }
        }
    }

    // Show detailed inspection
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
                println!("  Compression: {}", compression_name(entry.compression));
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
        println!(
            "Central Directory Offset: {}",
            header.central_directory_offset
        );
        println!("Central Directory Size: {}", header.central_directory_size);
        println!("Header CRC: {:08X}", header.header_crc);
    }

    Ok(())
}
