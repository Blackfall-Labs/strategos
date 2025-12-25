//! Cartridge-specific commands
//!
//! Commands for mutable Cartridge archives (.cart)

use anyhow::{Context, Result};
use cartridge_rs::Cartridge;
use std::path::Path;

/// Create a new Cartridge archive
pub fn create(slug: &str, title: &str, output: Option<&Path>) -> Result<()> {
    let cart = if let Some(path) = output {
        Cartridge::create_at(path, slug, title)?
    } else {
        Cartridge::create(slug, title)?
    };

    println!("✅ Created Cartridge archive: {}", slug);
    println!("   Slug:  {}", cart.slug()?);
    println!("   Title: {}", cart.title()?);

    Ok(())
}

/// Write or update a file in a Cartridge archive
pub fn write(archive_path: &Path, file_path: &str, content_path: &Path) -> Result<()> {
    let mut cart = Cartridge::open(archive_path)
        .with_context(|| format!("Failed to open Cartridge: {}", archive_path.display()))?;

    let content = std::fs::read(content_path)
        .with_context(|| format!("Failed to read file: {}", content_path.display()))?;

    cart.write(file_path, &content)?;
    cart.flush()?;

    println!("✅ Wrote {} bytes to {}", content.len(), file_path);

    Ok(())
}

/// Delete a file from a Cartridge archive
pub fn delete(archive_path: &Path, file_path: &str) -> Result<()> {
    let mut cart = Cartridge::open(archive_path)
        .with_context(|| format!("Failed to open Cartridge: {}", archive_path.display()))?;

    cart.delete(file_path)?;
    cart.flush()?;

    println!("✅ Deleted: {}", file_path);

    Ok(())
}

/// Create a snapshot of a Cartridge archive
pub fn snapshot(
    archive_path: &Path,
    name: String,
    description: String,
    snapshot_dir: &Path,
) -> Result<()> {
    let cart = Cartridge::open(archive_path)
        .with_context(|| format!("Failed to open Cartridge: {}", archive_path.display()))?;

    let snapshot_id = cart.create_snapshot(name.clone(), description, snapshot_dir)?;

    println!("✅ Created snapshot: {} (ID: {})", name, snapshot_id);
    println!("   Stored in: {}", snapshot_dir.display());

    Ok(())
}

/// Freeze a Cartridge archive to Engram format (immutable)
pub fn freeze(cartridge_path: &Path, engram_output: &Path) -> Result<()> {
    // This would require engram_integration from cartridge-rs
    // For now, provide a helpful message
    println!("⚠️  Freeze functionality requires cartridge-rs engram_integration");
    println!("   Cartridge: {}", cartridge_path.display());
    println!("   Engram:    {}", engram_output.display());
    println!("\n   This feature converts a mutable Cartridge to an immutable Engram archive.");
    println!("   Implementation pending in cartridge-rs API.");

    anyhow::bail!("Freeze command not yet implemented")
}

/// List snapshots of a Cartridge archive
pub fn list_snapshots(snapshot_dir: &Path) -> Result<()> {
    if !snapshot_dir.exists() {
        println!("No snapshots found in: {}", snapshot_dir.display());
        return Ok(());
    }

    println!("📸 Snapshots in: {}", snapshot_dir.display());
    println!("{}", "─".repeat(60));

    // List snapshot metadata files
    for entry in std::fs::read_dir(snapshot_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let metadata = std::fs::read_to_string(&path)?;
            let meta: serde_json::Value = serde_json::from_str(&metadata)?;

            println!("Snapshot ID: {}", meta["id"]);
            println!("Name:        {}", meta["name"]);
            println!("Description: {}", meta["description"]);
            println!("Created:     {}", meta["timestamp"]);
            println!();
        }
    }

    Ok(())
}

/// Restore a Cartridge from a snapshot
pub fn restore(archive_path: &Path, snapshot_id: u64, snapshot_dir: &Path) -> Result<()> {
    let mut cart = Cartridge::open(archive_path)
        .with_context(|| format!("Failed to open Cartridge: {}", archive_path.display()))?;

    cart.restore_snapshot(snapshot_id, snapshot_dir)?;

    println!("✅ Restored snapshot {} to: {}", snapshot_id, archive_path.display());

    Ok(())
}
