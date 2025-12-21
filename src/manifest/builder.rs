//! Manifest creation and loading
//!
//! Handles loading manifest.toml files and converting to engram-rs Manifest format.

use anyhow::{Context, Result};
use engram_rs::manifest::{Author, Manifest};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// TOML manifest format (input from users)
#[derive(Debug, Deserialize)]
pub struct TomlManifest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: TomlAuthor,
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomlAuthor {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

impl TomlManifest {
    /// Load manifest from TOML file
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read manifest file")?;
        let manifest: TomlManifest =
            toml::from_str(&content).context("Failed to parse manifest")?;
        Ok(manifest)
    }

    /// Convert to engram-rs Manifest
    pub fn to_engram_manifest(&self) -> Manifest {
        let author = Author {
            name: self.author.name.clone(),
            email: self.author.email.clone(),
            url: self.author.url.clone(),
        };

        let mut manifest = Manifest::new(
            self.id.clone(),
            self.name.clone(),
            author,
            self.version.clone(),
        );

        manifest.description = self.description.clone();
        manifest.metadata.license = self.license.clone();
        manifest.metadata.tags = self.tags.clone();
        manifest.capabilities = self.capabilities.clone();

        manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_toml_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("manifest.toml");

        let toml_content = r#"
id = "test-engram"
name = "Test Engram"
description = "A test engram"
version = "1.0.0"
license = "MIT"
tags = ["test", "example"]
capabilities = ["read", "write"]

[author]
name = "Test Author"
email = "test@example.com"
url = "https://example.com"
"#;

        fs::write(&manifest_path, toml_content).unwrap();

        let toml_manifest = TomlManifest::load(&manifest_path).unwrap();
        assert_eq!(toml_manifest.id, "test-engram");
        assert_eq!(toml_manifest.name, "Test Engram");
        assert_eq!(toml_manifest.version, "1.0.0");

        let manifest = toml_manifest.to_engram_manifest();
        assert_eq!(manifest.id, "test-engram");
        assert_eq!(manifest.name, "Test Engram");
        assert_eq!(manifest.author.name, "Test Author");
    }
}
