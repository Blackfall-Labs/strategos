use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Archive format types supported by Strategos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// Engram (.eng) - Immutable signed archives
    Engram,
    /// Cartridge (.cart) - Mutable page-based archives
    Cartridge,
    /// DataSpool (.spool) - Append-only item collections
    DataSpool,
    /// DataCard (.card) - Compressed CML documents
    DataCard,
    /// Unknown or unsupported format
    Unknown,
}

impl ArchiveFormat {
    /// Get the typical file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ArchiveFormat::Engram => ".eng",
            ArchiveFormat::Cartridge => ".cart",
            ArchiveFormat::DataSpool => ".spool",
            ArchiveFormat::DataCard => ".card",
            ArchiveFormat::Unknown => "",
        }
    }

    /// Get the human-readable name for this format
    pub fn name(&self) -> &'static str {
        match self {
            ArchiveFormat::Engram => "Engram",
            ArchiveFormat::Cartridge => "Cartridge",
            ArchiveFormat::DataSpool => "DataSpool",
            ArchiveFormat::DataCard => "DataCard",
            ArchiveFormat::Unknown => "Unknown",
        }
    }
}

/// Magic bytes for each format
const ENGRAM_MAGIC: &[u8] = b"\x89ENG\r\n\x1a\n"; // PNG-style
const CARTRIDGE_MAGIC: &[u8] = b"CART\x00\x01\x00\x00";
const DATASPOOL_MAGIC: &[u8] = b"SP01";
const DATACARD_MAGIC: &[u8] = b"CARD";

/// Detect archive format from file header
///
/// This function reads the first 8 bytes of the file and checks magic bytes
/// to determine the format. Falls back to extension-based detection if magic
/// bytes don't match any known format.
pub fn detect_format(path: &Path) -> Result<ArchiveFormat> {
    // First try header-based detection
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    let mut header = [0u8; 8];
    match file.read(&mut header) {
        Ok(n) if n >= 4 => {
            // Check magic bytes (in priority order)
            if n >= 8 && &header == ENGRAM_MAGIC {
                return Ok(ArchiveFormat::Engram);
            }

            if n >= 8 && &header == CARTRIDGE_MAGIC {
                return Ok(ArchiveFormat::Cartridge);
            }

            if &header[0..4] == DATASPOOL_MAGIC {
                return Ok(ArchiveFormat::DataSpool);
            }

            if &header[0..4] == DATACARD_MAGIC {
                return Ok(ArchiveFormat::DataCard);
            }
        }
        _ => {}
    }

    // Fall back to extension-based detection
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "eng" => return Ok(ArchiveFormat::Engram),
            "cart" => return Ok(ArchiveFormat::Cartridge),
            "spool" => return Ok(ArchiveFormat::DataSpool),
            "card" => return Ok(ArchiveFormat::DataCard),
            _ => {}
        }
    }

    Ok(ArchiveFormat::Unknown)
}

/// Detect format from raw bytes (useful for tests)
pub fn detect_format_from_bytes(bytes: &[u8]) -> ArchiveFormat {
    if bytes.len() >= 8 && &bytes[0..8] == ENGRAM_MAGIC {
        return ArchiveFormat::Engram;
    }

    if bytes.len() >= 8 && &bytes[0..8] == CARTRIDGE_MAGIC {
        return ArchiveFormat::Cartridge;
    }

    if bytes.len() >= 4 && &bytes[0..4] == DATASPOOL_MAGIC {
        return ArchiveFormat::DataSpool;
    }

    if bytes.len() >= 4 && &bytes[0..4] == DATACARD_MAGIC {
        return ArchiveFormat::DataCard;
    }

    ArchiveFormat::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_engram_from_bytes() {
        let header = b"\x89ENG\r\n\x1a\n";
        assert_eq!(detect_format_from_bytes(header), ArchiveFormat::Engram);
    }

    #[test]
    fn test_detect_cartridge_from_bytes() {
        let header = b"CART\x00\x01\x00\x00";
        assert_eq!(detect_format_from_bytes(header), ArchiveFormat::Cartridge);
    }

    #[test]
    fn test_detect_dataspool_from_bytes() {
        let header = b"SP01";
        assert_eq!(detect_format_from_bytes(header), ArchiveFormat::DataSpool);
    }

    #[test]
    fn test_detect_datacard_from_bytes() {
        let header = b"CARD";
        assert_eq!(detect_format_from_bytes(header), ArchiveFormat::DataCard);
    }

    #[test]
    fn test_detect_unknown_from_bytes() {
        let header = b"UNKN";
        assert_eq!(detect_format_from_bytes(header), ArchiveFormat::Unknown);
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(ArchiveFormat::Engram.extension(), ".eng");
        assert_eq!(ArchiveFormat::Cartridge.extension(), ".cart");
        assert_eq!(ArchiveFormat::DataSpool.extension(), ".spool");
        assert_eq!(ArchiveFormat::DataCard.extension(), ".card");
    }

    #[test]
    fn test_format_names() {
        assert_eq!(ArchiveFormat::Engram.name(), "Engram");
        assert_eq!(ArchiveFormat::Cartridge.name(), "Cartridge");
        assert_eq!(ArchiveFormat::DataSpool.name(), "DataSpool");
        assert_eq!(ArchiveFormat::DataCard.name(), "DataCard");
    }
}
