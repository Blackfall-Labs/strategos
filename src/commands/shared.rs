//! Shared commands that work across all archive formats
//!
//! This module provides format-agnostic implementations of common operations
//! like info, list, extract, verify, and search.

use anyhow::{Context, Result};
use std::path::Path;

use crate::formats::{
    detect_format, Archive, ArchiveFormat, CartridgeArchive, DataCardArchive, DataSpoolArchive,
    EngramArchive, MutableArchive, QueryableArchive,
};

/// Dispatch info command to the appropriate format handler
pub fn info(path: &Path, inspect: bool, verify_sigs: bool, show_manifest: bool) -> Result<()> {
    let format = detect_format(path)?;

    match format {
        ArchiveFormat::Engram => {
            let mut archive = EngramArchive::open(path)?;
            display_info(&mut archive, inspect, verify_sigs, show_manifest)
        }
        ArchiveFormat::Cartridge => {
            let mut archive = CartridgeArchive::open(path)?;
            display_info(&mut archive, inspect, verify_sigs, show_manifest)
        }
        ArchiveFormat::DataSpool => {
            let mut archive = DataSpoolArchive::open(path)?;
            display_info(&mut archive, inspect, verify_sigs, show_manifest)
        }
        ArchiveFormat::DataCard => {
            let mut archive = DataCardArchive::open(path)?;
            display_info(&mut archive, inspect, verify_sigs, show_manifest)
        }
        ArchiveFormat::Unknown => {
            anyhow::bail!("Unknown archive format: {}", path.display())
        }
    }
}

fn display_info<A: Archive>(
    archive: &mut A,
    inspect: bool,
    verify_sigs: bool,
    show_manifest: bool,
) -> Result<()> {
    let info = archive.info()?;

    println!("📦 Archive Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Format:           {}", info.format);
    println!("Version:          {}", info.version);
    println!("Entry Count:      {}", info.entry_count);
    println!("Total Size:       {} bytes ({:.2} MB)", info.total_size, info.total_size as f64 / 1_048_576.0);

    if info.compressed_size > 0 {
        println!("Compressed Size:  {} bytes ({:.2} MB)", info.compressed_size, info.compressed_size as f64 / 1_048_576.0);
        println!("Compression:      {:.2}x", info.compression_ratio);
    }

    if show_manifest {
        println!("\n📄 Metadata:");
        println!("{}", serde_json::to_string_pretty(&info.metadata)?);
    }

    if verify_sigs {
        println!("\n🔐 Verifying signatures...");
        match archive.verify()? {
            true => println!("✅ All signatures valid"),
            false => println!("❌ Signature verification failed"),
        }
    }

    if inspect {
        println!("\n📋 Files:");
        let files = archive.list_files()?;
        for (i, file) in files.iter().take(10).enumerate() {
            println!("  {}: {} ({} bytes)", i + 1, file.path, file.size);
        }
        if files.len() > 10 {
            println!("  ... and {} more", files.len() - 10);
        }
    }

    Ok(())
}

/// Dispatch list command to the appropriate format handler
pub fn list(path: &Path, long_format: bool, databases_only: bool) -> Result<()> {
    let format = detect_format(path)?;

    match format {
        ArchiveFormat::Engram => {
            let mut archive = EngramArchive::open(path)?;
            display_list(&mut archive, long_format, databases_only)
        }
        ArchiveFormat::Cartridge => {
            let mut archive = CartridgeArchive::open(path)?;
            display_list(&mut archive, long_format, databases_only)
        }
        ArchiveFormat::DataSpool => {
            let mut archive = DataSpoolArchive::open(path)?;
            display_list(&mut archive, long_format, databases_only)
        }
        ArchiveFormat::DataCard => {
            let mut archive = DataCardArchive::open(path)?;
            display_list(&mut archive, long_format, databases_only)
        }
        ArchiveFormat::Unknown => {
            anyhow::bail!("Unknown archive format: {}", path.display())
        }
    }
}

fn display_list<A: Archive>(
    archive: &mut A,
    long_format: bool,
    databases_only: bool,
) -> Result<()> {
    let files = archive.list_files()?;

    let filtered: Vec<_> = if databases_only {
        files
            .into_iter()
            .filter(|f| f.path.ends_with(".db") || f.path.ends_with(".sqlite"))
            .collect()
    } else {
        files
    };

    if long_format {
        println!("{:<50} {:>12} {:>12} {:>10}", "PATH", "SIZE", "COMPRESSED", "METHOD");
        println!("{}", "─".repeat(88));
        for file in filtered {
            println!(
                "{:<50} {:>12} {:>12} {:>10}",
                file.path,
                format_size(file.size),
                format_size(file.compressed_size),
                file.compression_method
            );
        }
    } else {
        for file in filtered {
            println!("{}", file.path);
        }
    }

    Ok(())
}

/// Dispatch extract command to the appropriate format handler
pub fn extract(archive_path: &Path, output: &Path, files: Option<Vec<String>>) -> Result<()> {
    let format = detect_format(archive_path)?;

    match format {
        ArchiveFormat::Engram => {
            let mut archive = EngramArchive::open(archive_path)?;
            archive.extract(output, files.as_deref())?;
        }
        ArchiveFormat::Cartridge => {
            let mut archive = CartridgeArchive::open(archive_path)?;
            archive.extract(output, files.as_deref())?;
        }
        ArchiveFormat::DataSpool => {
            let mut archive = DataSpoolArchive::open(archive_path)?;
            archive.extract(output, files.as_deref())?;
        }
        ArchiveFormat::DataCard => {
            let mut archive = DataCardArchive::open(archive_path)?;
            archive.extract(output, files.as_deref())?;
        }
        ArchiveFormat::Unknown => {
            anyhow::bail!("Unknown archive format: {}", archive_path.display())
        }
    }

    println!("✅ Extracted to: {}", output.display());
    Ok(())
}

/// Dispatch verify command to the appropriate format handler
pub fn verify(path: &Path) -> Result<()> {
    let format = detect_format(path)?;

    let result = match format {
        ArchiveFormat::Engram => {
            let mut archive = EngramArchive::open(path)?;
            archive.verify()?
        }
        ArchiveFormat::Cartridge => {
            let mut archive = CartridgeArchive::open(path)?;
            archive.verify()?
        }
        ArchiveFormat::DataSpool => {
            let mut archive = DataSpoolArchive::open(path)?;
            archive.verify()?
        }
        ArchiveFormat::DataCard => {
            let mut archive = DataCardArchive::open(path)?;
            archive.verify()?
        }
        ArchiveFormat::Unknown => {
            anyhow::bail!("Unknown archive format: {}", path.display())
        }
    };

    if result {
        println!("✅ Archive verified successfully");
        Ok(())
    } else {
        anyhow::bail!("❌ Archive verification failed")
    }
}

/// Dispatch search command to the appropriate format handler
pub fn search(path: &Path, pattern: &str, case_insensitive: bool) -> Result<()> {
    let format = detect_format(path)?;

    let results = match format {
        ArchiveFormat::Engram => {
            let mut archive = EngramArchive::open(path)?;
            archive.search(pattern, case_insensitive)?
        }
        ArchiveFormat::Cartridge => {
            let mut archive = CartridgeArchive::open(path)?;
            archive.search(pattern, case_insensitive)?
        }
        ArchiveFormat::DataSpool => {
            let mut archive = DataSpoolArchive::open(path)?;
            archive.search(pattern, case_insensitive)?
        }
        ArchiveFormat::DataCard => {
            let mut archive = DataCardArchive::open(path)?;
            archive.search(pattern, case_insensitive)?
        }
        ArchiveFormat::Unknown => {
            anyhow::bail!("Unknown archive format: {}", path.display())
        }
    };

    if results.is_empty() {
        println!("No matches found");
    } else {
        println!("Found {} matches:\n", results.len());
        for result in results.iter().take(100) {
            println!(
                "{}:{}:{}",
                result.file_path, result.line_number, result.line_content
            );
        }
        if results.len() > 100 {
            println!("\n... and {} more matches", results.len() - 100);
        }
    }

    Ok(())
}

/// Format byte size for display
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
