use anyhow::{Context, Result};
use dataspool_rs::{SpoolReader, SpoolBuilder};
use std::path::Path;

use super::traits::{Archive, ArchiveInfo, FileEntry, MutableArchive, SearchResult};

/// Wrapper for DataSpool archives (.spool)
///
/// DataSpool is an append-only format for bundling multiple items (cards, images, etc.)
/// with a byte-offset index for random access.
pub struct DataSpoolArchive {
    reader: SpoolReader,
    path: std::path::PathBuf,
}

impl Archive for DataSpoolArchive {
    fn open(path: &Path) -> Result<Self> {
        let reader = SpoolReader::open(path)
            .with_context(|| format!("Failed to open DataSpool archive: {}", path.display()))?;

        Ok(Self {
            reader,
            path: path.to_path_buf(),
        })
    }

    fn info(&mut self) -> Result<ArchiveInfo> {
        let entry_count = self.reader.card_count();
        let entries = self.reader.entries();

        let mut total_size = 0u64;
        for entry in entries {
            total_size += entry.length as u64;
        }

        // DataSpool stores pre-compressed data, so compressed size ≈ total size
        let compressed_size = total_size;

        let metadata = serde_json::json!({
            "card_count": entry_count,
            "format": "dataspool",
            "index_entries": entry_count,
        });

        Ok(ArchiveInfo {
            format: "DataSpool".to_string(),
            version: "1".to_string(),
            entry_count,
            total_size,
            compressed_size,
            compression_ratio: 1.0, // Data is pre-compressed
            metadata,
        })
    }

    fn list_files(&mut self) -> Result<Vec<FileEntry>> {
        let entries = self.reader.entries();
        let mut file_entries = Vec::new();

        for (index, entry) in entries.iter().enumerate() {
            file_entries.push(FileEntry {
                path: format!("card_{:05}", index), // Virtual path for each card
                size: entry.length as u64,
                compressed_size: entry.length as u64, // Pre-compressed
                compression_method: "bytepunch".to_string(),
                modified: None,
                crc32: None,
            });
        }

        Ok(file_entries)
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        // Parse card index from path (e.g., "card_00000" -> 0)
        let index = if let Some(idx_str) = path.strip_prefix("card_") {
            idx_str.parse::<usize>()
                .with_context(|| format!("Invalid card path: {}", path))?
        } else {
            // Try parsing as direct index
            path.parse::<usize>()
                .with_context(|| format!("Invalid card index: {}", path))?
        };

        self.reader
            .read_card(index)
            .with_context(|| format!("Failed to read card {} from DataSpool", index))
    }

    fn extract(&mut self, output: &Path, files: Option<&[String]>) -> Result<()> {
        let card_count = self.reader.card_count();

        // If specific files requested, extract those; otherwise extract all
        if let Some(files) = files {
            for file_path in files {
                let data = self.read_file(file_path)?;
                let output_path = output.join(file_path);

                std::fs::create_dir_all(output)
                    .with_context(|| format!("Failed to create directory: {}", output.display()))?;

                std::fs::write(&output_path, data)
                    .with_context(|| format!("Failed to write file: {}", output_path.display()))?;
            }
        } else {
            // Extract all cards
            std::fs::create_dir_all(output)
                .with_context(|| format!("Failed to create directory: {}", output.display()))?;

            for index in 0..card_count {
                let data = self.reader.read_card(index)?;
                let output_path = output.join(format!("card_{:05}.card", index));

                std::fs::write(&output_path, data)
                    .with_context(|| format!("Failed to write file: {}", output_path.display()))?;
            }
        }

        Ok(())
    }

    fn verify(&mut self) -> Result<bool> {
        // Verify we can read all cards successfully
        let card_count = self.reader.card_count();

        for index in 0..card_count {
            if let Err(_) = self.reader.read_card(index) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let card_count = self.reader.card_count();

        for index in 0..card_count {
            let data = match self.reader.read_card(index) {
                Ok(d) => d,
                Err(_) => continue,
            };

            // Try to parse as text (DataCards are typically CML/XML)
            let content = match String::from_utf8(data) {
                Ok(s) => s,
                Err(_) => continue, // Skip binary cards
            };

            let card_path = format!("card_{:05}", index);

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
                        file_path: card_path.clone(),
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
        "DataSpool"
    }
}

impl MutableArchive for DataSpoolArchive {
    fn write_file(&mut self, _path: &str, data: &[u8]) -> Result<()> {
        // DataSpool is append-only, so we need to reopen as builder
        // This is a limitation of the current API - we can't append in-place with SpoolReader

        // For now, we'll create a new spool with all existing cards + new card
        let temp_path = self.path.with_extension("spool.tmp");

        let mut builder = SpoolBuilder::new(&temp_path)
            .context("Failed to create temporary spool builder")?;

        // Copy all existing cards
        let card_count = self.reader.card_count();
        for index in 0..card_count {
            let card_data = self.reader.read_card(index)?;
            builder.add_card(&card_data)?;
        }

        // Add new card
        builder.add_card(data)?;
        builder.finalize()?;

        // Replace original file
        std::fs::rename(&temp_path, &self.path)
            .context("Failed to replace original spool file")?;

        // Reopen reader
        self.reader = SpoolReader::open(&self.path)?;

        Ok(())
    }

    fn delete_file(&mut self, _path: &str) -> Result<()> {
        anyhow::bail!("DataSpool does not support deletion (append-only format)")
    }

    fn flush(&mut self) -> Result<()> {
        // DataSpool writes are immediately flushed
        Ok(())
    }
}
