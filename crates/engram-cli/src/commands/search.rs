//! Search command - Search for text patterns

use anyhow::{Context, Result};
use engram_rs::ArchiveReader;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

pub fn search(
    pattern: &str,
    path: &Path,
    in_archive: bool,
    case_insensitive: bool,
) -> Result<()> {
    if in_archive {
        // Search inside archive
        search_in_archive(pattern, path, case_insensitive)
    } else {
        // Search regular file
        search_in_file(pattern, path, case_insensitive)
    }
}

fn search_in_file(pattern: &str, path: &Path, case_insensitive: bool) -> Result<()> {
    let content = read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    find_matches(&content, pattern, &mut std::io::stdout(), case_insensitive)
        .with_context(|| format!("Failed to find matching content for pattern `{}`", pattern))?;

    Ok(())
}

fn search_in_archive(pattern: &str, archive_path: &Path, case_insensitive: bool) -> Result<()> {
    let mut reader = ArchiveReader::open(archive_path)?;
    reader.initialize()?;

    let mut found_any = false;

    // Clone the files list to avoid borrowing issues
    let all_files = reader.list_files().to_vec();

    for file_path in &all_files {
        // Try to read as text
        if let Ok(data) = reader.read_file(file_path)
            && let Ok(content) = String::from_utf8(data)
        {
            let mut matches = Vec::new();

            for line in content.lines() {
                let matches_line = if case_insensitive {
                    line.to_lowercase().contains(&pattern.to_lowercase())
                } else {
                    line.contains(pattern)
                };

                if matches_line {
                    matches.push(line.to_string());
                }
            }

            if !matches.is_empty() {
                println!("\n{}:", file_path);
                for line in matches {
                    println!("  {}", line);
                }
                found_any = true;
            }
        }
    }

    if !found_any {
        println!("No matches found");
    }

    Ok(())
}

fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl Write,
    case_insensitive: bool,
) -> Result<()> {
    for line in content.lines() {
        let matches = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if matches {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}
