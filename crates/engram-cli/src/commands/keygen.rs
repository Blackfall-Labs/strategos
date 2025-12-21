//! Key generation command

use crate::crypto::keys::KeyPair;
use anyhow::Result;
use std::path::PathBuf;

/// Generate a new Ed25519 keypair
pub fn keygen(private_key_path: &PathBuf, public_key_path: &PathBuf) -> Result<()> {
    println!("Generating new Ed25519 keypair...");

    let keypair = KeyPair::generate();
    keypair.save(private_key_path, public_key_path)?;

    println!("✓ Private key saved to: {}", private_key_path.display());
    println!("✓ Public key saved to: {}", public_key_path.display());
    println!();
    println!("Keep your private key secure and never share it!");
    println!("You can share your public key for signature verification.");

    Ok(())
}
