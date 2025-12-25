use anyhow::{Context, Result};
use engram_rs::{ArchiveReader, VfsReader};
use rusqlite::Connection;
use std::path::Path;

use super::traits::{Archive, ArchiveInfo, FileEntry, OutputFormat, QueryableArchive, SearchResult};

/// Wrapper for Engram archives (.eng)
///
/// Engram archives are immutable, cryptographically signed archives
/// designed for long-term knowledge preservation.
pub struct EngramArchive {
    reader: ArchiveReader,
    path: std::path::PathBuf,
}

impl Archive for EngramArchive {
    fn open(path: &Path) -> Result<Self> {
        let reader = ArchiveReader::open(path)
            .with_context(|| format!("Failed to open Engram archive: {}", path.display()))?;

        Ok(Self {
            reader,
            path: path.to_path_buf(),
        })
    }

    fn info(&mut self) -> Result<ArchiveInfo> {
        let files = self.reader.list_files().to_vec();
        let entry_count = files.len();

        let mut total_size = 0u64;
        let compressed_size = 0u64; // Engram doesn't expose compressed size easily

        for file in &files {
            if let Ok(data) = self.reader.read_file(file) {
                total_size += data.len() as u64;
                // Engram stores uncompressed data in memory, so compressed_size
                // is approximated from the archive file
            }
        }

        // Try to read manifest for additional metadata
        let manifest = self.reader.read_manifest().ok().flatten().unwrap_or_else(|| {
            serde_json::json!({
                "version": "unknown",
                "format": "engram"
            })
        });

        let compression_ratio = if compressed_size > 0 {
            total_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        Ok(ArchiveInfo {
            format: "Engram".to_string(),
            version: "1.0".to_string(),
            entry_count,
            total_size,
            compressed_size,
            compression_ratio,
            metadata: manifest,
        })
    }

    fn list_files(&mut self) -> Result<Vec<FileEntry>> {
        let files = self.reader.list_files().to_vec();
        let mut entries = Vec::new();

        for file_path in files {
            // Engram doesn't expose per-file metadata easily, so we approximate
            if let Ok(data) = self.reader.read_file(&file_path) {
                entries.push(FileEntry {
                    path: file_path,
                    size: data.len() as u64,
                    compressed_size: 0, // Not available from ArchiveReader
                    compression_method: "unknown".to_string(),
                    modified: None,
                    crc32: None,
                });
            }
        }

        Ok(entries)
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        self.reader
            .read_file(path)
            .with_context(|| format!("Failed to read file '{}' from Engram archive", path))
    }

    fn extract(&mut self, output: &Path, files: Option<&[String]>) -> Result<()> {
        let files_to_extract = match files {
            Some(f) => f.to_vec(),
            None => self.reader.list_files().to_vec(),
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
        // Check if manifest exists and verify signatures
        if let Some(manifest_value) = self.reader.read_manifest()? {
            if let Ok(manifest) = serde_json::from_value::<engram_rs::Manifest>(manifest_value) {
                match manifest.verify_signatures() {
                    Ok(results) => {
                        // All signatures must be valid
                        return Ok(results.iter().all(|&valid| valid));
                    }
                    Err(_) => return Ok(false),
                }
            }
        }

        // No manifest or signatures to verify
        Ok(true)
    }

    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let files = self.reader.list_files().to_vec();

        for file_path in files {
            let data = match self.reader.read_file(&file_path) {
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
        "Engram"
    }
}

impl QueryableArchive for EngramArchive {
    fn list_databases(&mut self) -> Result<Vec<String>> {
        let all_files = self.reader.list_files().to_vec();
        Ok(all_files
            .into_iter()
            .filter(|f| {
                f.ends_with(".db") || f.ends_with(".sqlite") || f.ends_with(".sqlite3")
            })
            .collect())
    }

    fn query(&mut self, database: &str, sql: &str, format: OutputFormat) -> Result<String> {
        // Open VFS reader for the archive
        let mut vfs = VfsReader::open(&self.path)
            .with_context(|| "Failed to open VFS reader for Engram archive")?;

        let conn = vfs
            .open_database(database)
            .with_context(|| format!("Failed to open database '{}'", database))?;

        execute_query(&conn, sql, format)
    }
}

/// Execute SQL query and format output
fn execute_query(conn: &Connection, sql: &str, format: OutputFormat) -> Result<String> {
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
