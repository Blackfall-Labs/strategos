//! DataSpool-specific commands
//!
//! Commands for DataSpool archives (.spool)

use anyhow::{Context, Result};
use dataspool_rs::{SpoolBuilder, SpoolReader};
use std::path::Path;

/// Build a new DataSpool from individual card files
pub fn build(output: &Path, card_files: &[String]) -> Result<()> {
    let mut builder = SpoolBuilder::new(output)
        .with_context(|| format!("Failed to create DataSpool: {}", output.display()))?;

    println!("🔨 Building DataSpool: {}", output.display());
    println!("   Adding {} cards...", card_files.len());

    for (i, card_path) in card_files.iter().enumerate() {
        let card_data = std::fs::read(card_path)
            .with_context(|| format!("Failed to read card: {}", card_path))?;

        let entry = builder.add_card(&card_data)?;

        println!(
            "   [{}] {} → offset: {}, size: {}",
            i + 1,
            card_path,
            entry.offset,
            entry.length
        );
    }

    builder.finalize()?;

    println!("✅ DataSpool created successfully");
    println!("   {} cards packed", card_files.len());

    Ok(())
}

/// Append cards to an existing DataSpool
pub fn append(spool_path: &Path, card_files: &[String]) -> Result<()> {
    // Read existing spool
    let mut reader = SpoolReader::open(spool_path)
        .with_context(|| format!("Failed to open DataSpool: {}", spool_path.display()))?;

    let existing_count = reader.card_count();

    // Create temporary spool with all cards
    let temp_path = spool_path.with_extension("spool.tmp");
    let mut builder = SpoolBuilder::new(&temp_path)?;

    println!("📎 Appending to DataSpool: {}", spool_path.display());
    println!("   Existing cards: {}", existing_count);
    println!("   New cards:      {}", card_files.len());

    // Copy existing cards
    for i in 0..existing_count {
        let card_data = reader.read_card(i)?;
        builder.add_card(&card_data)?;
    }

    // Add new cards
    for card_path in card_files {
        let card_data = std::fs::read(card_path)
            .with_context(|| format!("Failed to read card: {}", card_path))?;

        builder.add_card(&card_data)?;
        println!("   + {}", card_path);
    }

    builder.finalize()?;

    // Replace original spool
    std::fs::rename(&temp_path, spool_path)
        .context("Failed to replace original spool")?;

    println!("✅ Appended {} new cards", card_files.len());
    println!("   Total cards: {}", existing_count + card_files.len());

    Ok(())
}

/// Extract a specific card from a DataSpool
pub fn extract_card(spool_path: &Path, index: usize, output: &Path) -> Result<()> {
    let mut reader = SpoolReader::open(spool_path)
        .with_context(|| format!("Failed to open DataSpool: {}", spool_path.display()))?;

    let card_data = reader.read_card(index)?;

    std::fs::write(output, card_data)
        .with_context(|| format!("Failed to write card: {}", output.display()))?;

    println!("✅ Extracted card {} to: {}", index, output.display());

    Ok(())
}

/// Show DataSpool index information
pub fn show_index(spool_path: &Path) -> Result<()> {
    let reader = SpoolReader::open(spool_path)
        .with_context(|| format!("Failed to open DataSpool: {}", spool_path.display()))?;

    let entries = reader.entries();

    println!("📇 DataSpool Index: {}", spool_path.display());
    println!("{}", "─".repeat(60));
    println!("{:<8} {:>12} {:>12}", "INDEX", "OFFSET", "SIZE");
    println!("{}", "─".repeat(60));

    for (i, entry) in entries.iter().enumerate() {
        println!(
            "{:<8} {:>12} {:>12}",
            i, entry.offset, entry.length
        );
    }

    println!("{}", "─".repeat(60));
    println!("Total cards: {}", entries.len());

    Ok(())
}

/// Placeholder for vector search (requires companion .db file)
pub fn vector_search(spool_path: &Path, query: &str, limit: usize) -> Result<()> {
    let db_path = spool_path.with_extension("db");

    if !db_path.exists() {
        anyhow::bail!(
            "No companion database found: {}\n\
             DataSpool vector search requires a .db file with embeddings.",
            db_path.display()
        );
    }

    println!("🔍 Vector search in: {}", spool_path.display());
    println!("   Query:    {}", query);
    println!("   Limit:    {}", limit);
    println!("   Database: {}", db_path.display());
    println!("\n⚠️  Vector search implementation pending.");
    println!("   This requires dataspool-rs::PersistentVectorStore integration.");

    anyhow::bail!("Vector search not yet implemented")
}
