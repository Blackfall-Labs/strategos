//! Ed25519 keypair management
//!
//! Handles key generation, loading, and saving for signing Engram archives.

use anyhow::{Context, Result};
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

/// Ed25519 keypair for signing engrams
pub struct KeyPair {
    signing_key: SigningKey,
}

impl KeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    /// Create keypair from raw private key bytes
    pub fn from_bytes(bytes: &[u8; SECRET_KEY_LENGTH]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes);
        Ok(Self { signing_key })
    }

    /// Load keypair from private key file (hex-encoded)
    pub fn load_private(path: impl AsRef<Path>) -> Result<Self> {
        let hex_str = fs::read_to_string(path).context("Failed to read private key file")?;
        let bytes = hex::decode(hex_str.trim()).context("Invalid hex encoding")?;

        if bytes.len() != SECRET_KEY_LENGTH {
            anyhow::bail!(
                "Invalid key length: expected {}, got {}",
                SECRET_KEY_LENGTH,
                bytes.len()
            );
        }

        let mut key_bytes = [0u8; SECRET_KEY_LENGTH];
        key_bytes.copy_from_slice(&bytes);

        Self::from_bytes(&key_bytes)
    }

    /// Get the signing key
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    /// Get the public key
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// Save keypair to files (hex-encoded)
    pub fn save(
        &self,
        private_path: impl AsRef<Path>,
        public_path: impl AsRef<Path>,
    ) -> Result<()> {
        // Save private key
        let private_hex = hex::encode(self.signing_key.to_bytes());
        fs::write(private_path, private_hex).context("Failed to write private key")?;

        // Save public key
        let public_hex = hex::encode(self.verifying_key().to_bytes());
        fs::write(public_path, public_hex).context("Failed to write public key")?;

        Ok(())
    }
}

/// Load public key from file (hex-encoded)
pub fn load_public_key(path: impl AsRef<Path>) -> Result<VerifyingKey> {
    let hex_str = fs::read_to_string(path).context("Failed to read public key file")?;
    let bytes = hex::decode(hex_str.trim()).context("Invalid hex encoding")?;

    if bytes.len() != PUBLIC_KEY_LENGTH {
        anyhow::bail!(
            "Invalid public key length: expected {}, got {}",
            PUBLIC_KEY_LENGTH,
            bytes.len()
        );
    }

    let mut key_bytes = [0u8; PUBLIC_KEY_LENGTH];
    key_bytes.copy_from_slice(&bytes);

    Ok(VerifyingKey::from_bytes(&key_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_keygen_generate() {
        let keypair = KeyPair::generate();
        assert_eq!(keypair.signing_key().to_bytes().len(), SECRET_KEY_LENGTH);
    }

    #[test]
    fn test_keygen_from_bytes() {
        let original = KeyPair::generate();
        let bytes = original.signing_key().to_bytes();
        let restored = KeyPair::from_bytes(&bytes).unwrap();

        assert_eq!(
            original.verifying_key().to_bytes(),
            restored.verifying_key().to_bytes()
        );
    }

    #[test]
    fn test_keygen_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let private_path = temp_dir.path().join("private.key");
        let public_path = temp_dir.path().join("public.key");

        let original = KeyPair::generate();
        original.save(&private_path, &public_path).unwrap();

        let loaded = KeyPair::load_private(&private_path).unwrap();
        assert_eq!(
            original.verifying_key().to_bytes(),
            loaded.verifying_key().to_bytes()
        );

        let public_key = load_public_key(&public_path).unwrap();
        assert_eq!(original.verifying_key().to_bytes(), public_key.to_bytes());
    }

    #[test]
    fn test_invalid_key_length() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("invalid.key");

        // Write invalid length hex
        fs::write(&path, hex::encode(b"too short")).unwrap();

        let result = KeyPair::load_private(&path);
        assert!(result.is_err());
    }
}
