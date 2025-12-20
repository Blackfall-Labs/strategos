//! Verify command - Verify signatures and integrity

use anyhow::{Context, Result};
use engram_rs::ArchiveReader;
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::crypto::keys::load_public_key;

pub fn verify(
    archive_path: &Path,
    public_key_path: Option<&Path>,
    check_hashes: bool,
) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)
        .with_context(|| format!("Failed to open archive `{}`", archive_path.display()))?;

    println!("Verifying: {}", archive_path.display());

    // Read manifest
    let manifest_value = reader.read_manifest()?
        .context("No manifest found in archive")?;

    let manifest: engram_rs::Manifest = serde_json::from_value(manifest_value)?;

    // Verify signatures
    if let Some(key_path) = public_key_path {
        let _public_key = load_public_key(key_path)?;
        println!("\nVerifying signatures...");

        if manifest.signatures.is_empty() {
            println!("  No signatures found");
        } else {
            let results = manifest.verify_signatures()?;
            let all_valid = results.iter().all(|&v| v);

            for (i, valid) in results.iter().enumerate() {
                if *valid {
                    println!("  ✓ Signature {} valid", i + 1);
                } else {
                    println!("  ✗ Signature {} invalid", i + 1);
                }
            }

            if !all_valid {
                anyhow::bail!("Some signatures are invalid");
            }
        }
    }

    // Check file hashes
    if check_hashes {
        println!("\nVerifying file hashes...");

        if manifest.files.is_empty() {
            println!("  No file hashes in manifest");
        } else {
            let mut all_valid = true;

            for file_entry in &manifest.files {
                let data = reader.read_file(&file_entry.path)?;
                let hash = hex::encode(Sha256::digest(&data));

                if hash == file_entry.sha256 {
                    println!("  ✓ {} hash valid", file_entry.path);
                } else {
                    println!("  ✗ {} hash mismatch", file_entry.path);
                    all_valid = false;
                }
            }

            if !all_valid {
                anyhow::bail!("Some file hashes are invalid");
            }
        }
    }

    println!("\n✓ Verification successful");

    Ok(())
}
