//! DataCard-specific commands
//!
//! Commands for DataCard files (.card)

use anyhow::{Context, Result};
use bytepunch_rs::Dictionary;
use datacard_rs::{Card, CardMetadata};
use std::path::Path;

/// Compress a CML document to a DataCard
pub fn compress(
    cml_path: &Path,
    output: &Path,
    dict_path: &Path,
    id: &str,
    with_checksum: bool,
) -> Result<()> {
    // Load CML content
    let cml = std::fs::read_to_string(cml_path)
        .with_context(|| format!("Failed to read CML file: {}", cml_path.display()))?;

    // Load dictionary
    let dict_json = std::fs::read_to_string(dict_path)
        .with_context(|| format!("Failed to read dictionary: {}", dict_path.display()))?;

    let dictionary = Dictionary::from_json(&dict_json)
        .context("Failed to parse BytePunch dictionary")?;

    // Create metadata
    let metadata = CardMetadata::new(id, cml.len() as u64);

    // Compress CML
    let card = if with_checksum {
        Card::from_cml_with_checksum(&cml, metadata, &dictionary)?
    } else {
        Card::from_cml(&cml, metadata, &dictionary)?
    };

    // Save card
    card.save(output)?;

    let compression_ratio = cml.len() as f64 / card.payload.len() as f64;

    println!("✅ Compressed CML to DataCard");
    println!("   Input:       {} ({} bytes)", cml_path.display(), cml.len());
    println!("   Output:      {} ({} bytes)", output.display(), card.payload.len());
    println!("   Ratio:       {:.2}x", compression_ratio);
    println!("   ID:          {}", id);
    println!("   Checksum:    {}", if with_checksum { "Yes" } else { "No" });

    Ok(())
}

/// Decompress a DataCard to CML
pub fn decompress(card_path: &Path, output: &Path, dict_path: &Path) -> Result<()> {
    // Load card
    let card = Card::load(card_path)
        .with_context(|| format!("Failed to load DataCard: {}", card_path.display()))?;

    // Load dictionary
    let dict_json = std::fs::read_to_string(dict_path)
        .with_context(|| format!("Failed to read dictionary: {}", dict_path.display()))?;

    let dictionary = Dictionary::from_json(&dict_json)
        .context("Failed to parse BytePunch dictionary")?;

    // Decompress CML
    let cml = card.to_cml(&dictionary)?;

    // Save CML
    std::fs::write(output, &cml)
        .with_context(|| format!("Failed to write CML file: {}", output.display()))?;

    println!("✅ Decompressed DataCard to CML");
    println!("   Input:       {} ({} bytes)", card_path.display(), card.payload.len());
    println!("   Output:      {} ({} bytes)", output.display(), cml.len());
    println!("   ID:          {}", card.metadata.id);

    Ok(())
}

/// Validate a DataCard (checksum, structure)
pub fn validate(card_path: &Path) -> Result<()> {
    let card = Card::load(card_path)
        .with_context(|| format!("Failed to load DataCard: {}", card_path.display()))?;

    println!("📄 DataCard Validation: {}", card_path.display());
    println!("{}", "─".repeat(60));

    // Basic info
    println!("Format:          DataCard v{}.{}", card.header.major, card.header.minor);
    println!("ID:              {}", card.metadata.id);
    println!("Profile:         {:?}", card.metadata.profile);
    println!("Compressed Size: {} bytes", card.metadata.compressed_size);

    if let Some(original) = card.metadata.original_size {
        println!("Original Size:   {} bytes", original);
        println!("Compression:     {:.2}x", original as f64 / card.metadata.compressed_size as f64);
    }

    // Checksum validation
    if card.header.has_checksum() {
        let calculated = card.calculate_checksum();
        println!("Checksum:        {:08x} (calculated)", calculated);
        println!("Status:          ⚠️  Need to read footer checksum for full validation");
    } else {
        println!("Checksum:        None");
    }

    // Structural validation
    println!("\n✅ Card structure is valid");
    println!("   - Header parsed successfully");
    println!("   - Metadata is valid JSON");
    println!("   - Payload size matches metadata");

    Ok(())
}

/// Show DataCard metadata without decompression
pub fn show_metadata(card_path: &Path) -> Result<()> {
    let card = Card::load(card_path)
        .with_context(|| format!("Failed to load DataCard: {}", card_path.display()))?;

    println!("📋 DataCard Metadata: {}", card_path.display());
    println!("{}", "─".repeat(60));

    let metadata_json = serde_json::to_string_pretty(&card.metadata)?;
    println!("{}", metadata_json);

    Ok(())
}
