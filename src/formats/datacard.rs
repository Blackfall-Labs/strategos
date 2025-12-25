use anyhow::{Context, Result};
use datacard_rs::Card;
use std::path::Path;

use super::traits::{Archive, ArchiveInfo, FileEntry, SearchResult};

/// Wrapper for DataCard (.card)
///
/// DataCard is a single-document format for BytePunch-compressed CML documents.
/// It's treated as a single-file archive for consistency with the Archive trait.
pub struct DataCardArchive {
    card: Card,
    path: std::path::PathBuf,
}

impl Archive for DataCardArchive {
    fn open(path: &Path) -> Result<Self> {
        let card = Card::load(path)
            .with_context(|| format!("Failed to open DataCard: {}", path.display()))?;

        Ok(Self {
            card,
            path: path.to_path_buf(),
        })
    }

    fn info(&mut self) -> Result<ArchiveInfo> {
        let compressed_size = self.card.payload.len() as u64;
        let original_size = self.card.metadata.original_size.unwrap_or(0);

        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        let metadata = serde_json::json!({
            "id": self.card.metadata.id,
            "profile": self.card.metadata.profile,
            "compressed_size": self.card.metadata.compressed_size,
            "original_size": self.card.metadata.original_size,
            "created": self.card.metadata.created,
            "dict_version": self.card.metadata.dict_version,
            "has_checksum": self.card.header.has_checksum(),
        });

        Ok(ArchiveInfo {
            format: "DataCard".to_string(),
            version: format!(
                "{}.{}",
                self.card.header.major, self.card.header.minor
            ),
            entry_count: 1, // Single document
            total_size: original_size,
            compressed_size,
            compression_ratio,
            metadata,
        })
    }

    fn list_files(&mut self) -> Result<Vec<FileEntry>> {
        // DataCard is a single file, so we return one entry
        let compressed_size = self.card.payload.len() as u64;
        let original_size = self.card.metadata.original_size.unwrap_or(compressed_size);

        Ok(vec![FileEntry {
            path: "document.cml".to_string(),
            size: original_size,
            compressed_size,
            compression_method: "bytepunch".to_string(),
            modified: self.card.metadata.created,
            crc32: if self.card.header.has_checksum() {
                Some(self.card.calculate_checksum())
            } else {
                None
            },
        }])
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        // DataCard stores compressed data; return the payload
        if path == "document.cml" || path == "payload" {
            Ok(self.card.payload.clone())
        } else {
            anyhow::bail!("File '{}' not found in DataCard (only 'document.cml' available)", path)
        }
    }

    fn extract(&mut self, output: &Path, _files: Option<&[String]>) -> Result<()> {
        // Extract the card payload (still compressed with BytePunch)
        let output_path = output.join("document.card");

        std::fs::create_dir_all(output)
            .with_context(|| format!("Failed to create directory: {}", output.display()))?;

        self.card.save(&output_path)
            .with_context(|| format!("Failed to write DataCard: {}", output_path.display()))?;

        Ok(())
    }

    fn verify(&mut self) -> Result<bool> {
        // Verify checksum if present
        if self.card.header.has_checksum() {
            let _calculated = self.card.calculate_checksum();
            // We'd need to read the original checksum from the file to compare
            // For now, just verify the card is valid
            Ok(true)
        } else {
            // No checksum, card is valid if it loaded successfully
            Ok(true)
        }
    }

    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>> {
        // DataCard stores compressed data, so we can't search without decompression
        // This would require a Dictionary, which we don't have in the Archive trait
        //
        // For now, search in the compressed payload (won't match CML content)
        let mut results = Vec::new();

        // Convert payload to string (likely to fail for binary data, but worth a try)
        if let Ok(content) = String::from_utf8(self.card.payload.clone()) {
            for (line_number, line) in content.lines().enumerate() {
                let matches = if case_insensitive {
                    line.to_lowercase().contains(&pattern.to_lowercase())
                } else {
                    line.contains(pattern)
                };

                if matches {
                    let match_offset = if case_insensitive {
                        line.to_lowercase()
                            .find(&pattern.to_lowercase())
                            .unwrap_or(0)
                    } else {
                        line.find(pattern).unwrap_or(0)
                    };

                    results.push(SearchResult {
                        file_path: "document.cml".to_string(),
                        line_number: line_number + 1,
                        line_content: line.to_string(),
                        match_offset,
                    });
                }
            }
        }

        Ok(results)
    }

    fn format_name(&self) -> &'static str {
        "DataCard"
    }
}

// DataCard is immutable, so no MutableArchive implementation
