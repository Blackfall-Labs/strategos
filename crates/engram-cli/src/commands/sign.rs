//! Sign command - Add signatures to archives

use anyhow::{Context, Result};
use engram_rs::{ArchiveReader, Manifest};
use std::path::Path;

use crate::crypto::keys::KeyPair;

pub fn sign(
    archive_path: &Path,
    private_key_path: &Path,
    signer: Option<&str>,
) -> Result<()> {
    println!("Signing: {}", archive_path.display());

    // Load signing key
    let keypair = KeyPair::load_private(private_key_path)?;

    // Read archive to get manifest
    let mut reader = ArchiveReader::open(archive_path)?;

    let manifest_value = reader.read_manifest()?
        .context("No manifest found in archive")?;

    // Parse manifest
    let mut manifest: Manifest = serde_json::from_value(manifest_value)?;

    // Sign the manifest
    manifest.sign(keypair.signing_key(), signer.map(|s| s.to_string()))?;

    println!("  Signature added");
    println!("  Signer: {}", signer.unwrap_or("(anonymous)"));
    println!("  Public key: {}", hex::encode(keypair.verifying_key().to_bytes()));

    // TODO: Write updated manifest back to archive
    println!("\nWarning: Manifest update not yet fully implemented");
    println!("The signature was generated but not written back to the archive");

    Ok(())
}
