//! Compression utilities

use anyhow::Result;
use engram_rs::CompressionMethod;

/// Parse compression method from string
pub fn parse_compression(s: &str) -> Result<CompressionMethod> {
    match s.to_lowercase().as_str() {
        "none" => Ok(CompressionMethod::None),
        "lz4" => Ok(CompressionMethod::Lz4),
        "zstd" => Ok(CompressionMethod::Zstd),
        _ => Err(anyhow::anyhow!(
            "Invalid compression method: '{}'. Use: none, lz4, zstd",
            s
        )),
    }
}

/// Get human-readable compression name
pub fn compression_name(method: CompressionMethod) -> &'static str {
    match method {
        CompressionMethod::None => "none",
        CompressionMethod::Lz4 => "lz4",
        CompressionMethod::Zstd => "zstd",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_compression() {
        assert!(matches!(
            parse_compression("none").unwrap(),
            CompressionMethod::None
        ));
        assert!(matches!(
            parse_compression("lz4").unwrap(),
            CompressionMethod::Lz4
        ));
        assert!(matches!(
            parse_compression("zstd").unwrap(),
            CompressionMethod::Zstd
        ));
        assert!(matches!(
            parse_compression("LZ4").unwrap(),
            CompressionMethod::Lz4
        ));
        assert!(parse_compression("invalid").is_err());
    }

    #[test]
    fn test_compression_name() {
        assert_eq!(compression_name(CompressionMethod::None), "none");
        assert_eq!(compression_name(CompressionMethod::Lz4), "lz4");
        assert_eq!(compression_name(CompressionMethod::Zstd), "zstd");
    }
}
