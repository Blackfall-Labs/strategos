use anyhow::{Context, Result};
use cartridge_rs::Cartridge as CartridgeCore;
use std::path::Path;

use super::traits::{
    Archive, ArchiveInfo, FileEntry, MutableArchive, OutputFormat, QueryableArchive, SearchResult,
};

/// Wrapper for Cartridge archives (.cart)
///
/// Cartridge archives are mutable, page-based archives with SQLite VFS integration.
/// They support in-place modifications, snapshots, and can be frozen to Engram format.
pub struct CartridgeArchive {
    cartridge: CartridgeCore,
}

impl Archive for CartridgeArchive {
    fn open(path: &Path) -> Result<Self> {
        let cartridge = CartridgeCore::open(path)
            .with_context(|| format!("Failed to open Cartridge archive: {}", path.display()))?;

        Ok(Self { cartridge })
    }

    fn info(&mut self) -> Result<ArchiveInfo> {
        let manifest = self.cartridge.read_manifest()?;
        let header = self.cartridge.header();

        // List all files to count entries and calculate sizes
        let all_files = self.cartridge.list("")?;
        let entry_count = all_files.len();

        let mut total_size = 0u64;
        let mut compressed_size = 0u64;

        for file_path in &all_files {
            if let Ok(metadata) = self.cartridge.metadata(file_path) {
                total_size += metadata.size;
                compressed_size += (metadata.blocks.len() as u64) * cartridge_rs::PAGE_SIZE as u64;
            }
        }

        let compression_ratio = if compressed_size > 0 {
            total_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Build metadata JSON
        let metadata = serde_json::json!({
            "slug": manifest.slug.as_ref(),
            "title": manifest.title,
            "description": manifest.description,
            "created": manifest.created,
            "modified_at": manifest.version, // Use version as proxy for modification
            "block_size": header.block_size,
            "total_blocks": header.total_blocks,
            "free_blocks": header.free_blocks,
        });

        Ok(ArchiveInfo {
            format: "Cartridge".to_string(),
            version: format!("{}.{}", header.version_major, header.version_minor),
            entry_count,
            total_size,
            compressed_size,
            compression_ratio,
            metadata,
        })
    }

    fn list_files(&mut self) -> Result<Vec<FileEntry>> {
        let all_files = self.cartridge.list("")?;
        let mut entries = Vec::new();

        for file_path in all_files {
            match self.cartridge.metadata(&file_path) {
                Ok(metadata) => {
                    let compressed_size =
                        (metadata.blocks.len() as u64) * cartridge_rs::PAGE_SIZE as u64;

                    entries.push(FileEntry {
                        path: file_path,
                        size: metadata.size,
                        compressed_size,
                        compression_method: "none".to_string(), // Cartridge doesn't expose compression info per file
                        modified: Some(metadata.modified_at),
                        crc32: None,
                    });
                }
                Err(_) => continue,
            }
        }

        Ok(entries)
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        self.cartridge
            .read(path)
            .with_context(|| format!("Failed to read file '{}' from Cartridge archive", path))
    }

    fn extract(&mut self, output: &Path, files: Option<&[String]>) -> Result<()> {
        let files_to_extract = match files {
            Some(f) => f.to_vec(),
            None => self.cartridge.list("")?,
        };

        for file_path in files_to_extract {
            let data = self.read_file(&file_path)?;
            let output_path = output.join(&file_path);

            // Create parent directories
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            std::fs::write(&output_path, data)
                .with_context(|| format!("Failed to write file: {}", output_path.display()))?;
        }

        Ok(())
    }

    fn verify(&mut self) -> Result<bool> {
        // Cartridge doesn't have built-in signature verification like Engram
        // We can verify the archive is valid by checking header integrity
        let header = self.cartridge.header();

        // Basic sanity checks
        if header.version_major == 0 && header.version_minor == 0 {
            return Ok(false);
        }

        if header.total_blocks == 0 {
            return Ok(false);
        }

        // Archive appears valid
        Ok(true)
    }

    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let files = self.cartridge.list("")?;

        for file_path in files {
            let data = match self.cartridge.read(&file_path) {
                Ok(d) => d,
                Err(_) => continue,
            };

            let content = match String::from_utf8(data) {
                Ok(s) => s,
                Err(_) => continue, // Skip binary files
            };

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
                        file_path: file_path.clone(),
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
        "Cartridge"
    }
}

impl MutableArchive for CartridgeArchive {
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        self.cartridge
            .write(path, data)
            .with_context(|| format!("Failed to write file '{}' to Cartridge archive", path))
    }

    fn delete_file(&mut self, path: &str) -> Result<()> {
        self.cartridge
            .delete(path)
            .with_context(|| format!("Failed to delete file '{}' from Cartridge archive", path))
    }

    fn flush(&mut self) -> Result<()> {
        self.cartridge
            .flush()
            .context("Failed to flush Cartridge archive")
    }
}

impl QueryableArchive for CartridgeArchive {
    fn list_databases(&mut self) -> Result<Vec<String>> {
        let all_files = self.cartridge.list("")?;
        Ok(all_files
            .into_iter()
            .filter(|f| {
                f.ends_with(".db") || f.ends_with(".sqlite") || f.ends_with(".sqlite3")
            })
            .collect())
    }

    fn query(&mut self, database: &str, sql: &str, format: OutputFormat) -> Result<String> {
        // Cartridge has VFS support, but the API isn't exposed in the high-level wrapper
        // For now, we can extract the database temporarily and query it
        // TODO: Use VFS integration when available in cartridge-rs API

        // Check if database exists
        if !self.cartridge.exists(database)? {
            anyhow::bail!("Database '{}' not found in Cartridge archive", database);
        }

        // Read database file
        let db_data = self.cartridge.read(database)?;

        // Create temporary file
        let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
        let temp_db_path = temp_dir.path().join("temp.db");
        std::fs::write(&temp_db_path, db_data)
            .context("Failed to write temporary database file")?;

        // Open with rusqlite
        let conn = rusqlite::Connection::open(&temp_db_path)
            .context("Failed to open temporary database")?;

        execute_query(&conn, sql, format)
    }
}

/// Execute SQL query and format output (shared with Engram)
fn execute_query(
    conn: &rusqlite::Connection,
    sql: &str,
    format: OutputFormat,
) -> Result<String> {
    let mut stmt = conn
        .prepare(sql)
        .with_context(|| "Failed to prepare SQL statement")?;

    let column_count = stmt.column_count();
    let column_names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

    let rows: Vec<Vec<String>> = stmt
        .query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..column_count {
                let value: rusqlite::Result<String> = row.get(i);
                values.push(value.unwrap_or_else(|_| "NULL".to_string()));
            }
            Ok(values)
        })?
        .filter_map(|r| r.ok())
        .collect();

    match format {
        OutputFormat::Table => format_as_table(&column_names, &rows),
        OutputFormat::Json => format_as_json(&column_names, &rows),
        OutputFormat::Csv => format_as_csv(&column_names, &rows),
    }
}

fn format_as_table(columns: &[String], rows: &[Vec<String>]) -> Result<String> {
    let mut output = String::new();

    // Calculate column widths
    let mut widths: Vec<usize> = columns.iter().map(|c| c.len()).collect();
    for row in rows {
        for (i, value) in row.iter().enumerate() {
            if value.len() > widths[i] {
                widths[i] = value.len();
            }
        }
    }

    // Print header
    for (i, col) in columns.iter().enumerate() {
        output.push_str(&format!("{:width$}  ", col, width = widths[i]));
    }
    output.push('\n');

    // Print separator
    for width in &widths {
        output.push_str(&"-".repeat(*width + 2));
    }
    output.push('\n');

    // Print rows
    for row in rows {
        for (i, value) in row.iter().enumerate() {
            output.push_str(&format!("{:width$}  ", value, width = widths[i]));
        }
        output.push('\n');
    }

    Ok(output)
}

fn format_as_json(columns: &[String], rows: &[Vec<String>]) -> Result<String> {
    let json_rows: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for (i, col) in columns.iter().enumerate() {
                map.insert(col.clone(), serde_json::Value::String(row[i].clone()));
            }
            serde_json::Value::Object(map)
        })
        .collect();

    serde_json::to_string_pretty(&json_rows).context("Failed to serialize to JSON")
}

fn format_as_csv(columns: &[String], rows: &[Vec<String>]) -> Result<String> {
    let mut output = String::new();

    // Header
    output.push_str(&columns.join(","));
    output.push('\n');

    // Rows
    for row in rows {
        output.push_str(&row.join(","));
        output.push('\n');
    }

    Ok(output)
}
