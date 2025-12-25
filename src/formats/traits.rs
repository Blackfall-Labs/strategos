use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Archive metadata and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveInfo {
    pub format: String,
    pub version: String,
    pub entry_count: usize,
    pub total_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub metadata: serde_json::Value,
}

/// File entry metadata
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub compressed_size: u64,
    pub compression_method: String,
    pub modified: Option<u64>,
    pub crc32: Option<u32>,
}

/// Search result from pattern matching
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub match_offset: usize,
}

/// Output format for query results
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

/// Core trait for all archive formats
///
/// Provides read-only access to archive contents
pub trait Archive {
    /// Open an archive from a file path
    fn open(path: &Path) -> Result<Self>
    where
        Self: Sized;

    /// Get archive metadata and statistics
    fn info(&mut self) -> Result<ArchiveInfo>;

    /// List all files/entries in the archive
    fn list_files(&mut self) -> Result<Vec<FileEntry>>;

    /// Read a specific file's contents
    fn read_file(&mut self, path: &str) -> Result<Vec<u8>>;

    /// Extract files to output directory
    /// If files is None, extract all files
    fn extract(&mut self, output: &Path, files: Option<&[String]>) -> Result<()>;

    /// Verify archive integrity (checksums, signatures)
    fn verify(&mut self) -> Result<bool>;

    /// Search for text pattern in files
    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>>;

    /// Get the archive format name
    fn format_name(&self) -> &'static str;
}

/// Trait for mutable archives (Cartridge, DataSpool with append)
///
/// Extends Archive with write capabilities
pub trait MutableArchive: Archive {
    /// Write or update a file in the archive
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()>;

    /// Delete a file from the archive
    fn delete_file(&mut self, path: &str) -> Result<()>;

    /// Flush pending changes to disk
    fn flush(&mut self) -> Result<()>;
}

/// Trait for archives that support SQL queries (Engram, Cartridge)
///
/// Provides VFS-based database access
pub trait QueryableArchive: Archive {
    /// List all SQLite databases in the archive
    fn list_databases(&mut self) -> Result<Vec<String>>;

    /// Execute a SQL query on a database
    fn query(&mut self, database: &str, sql: &str, format: OutputFormat) -> Result<String>;

    /// Check if a specific file is a SQLite database
    fn is_database(&mut self, path: &str) -> Result<bool> {
        Ok(path.ends_with(".db") || path.ends_with(".sqlite") || path.ends_with(".sqlite3"))
    }
}
