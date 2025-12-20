//! Query command - Query SQLite databases in archives

use anyhow::{Context, Result};
use engram_rs::VfsReader;
use std::path::Path;

pub fn query(
    archive_path: &Path,
    list_databases: bool,
    database: Option<&str>,
    sql: Option<&str>,
    format: &str,
) -> Result<()> {
    let mut vfs = VfsReader::open(archive_path)
        .with_context(|| format!("Failed to open archive `{}`", archive_path.display()))?;

    // List databases
    if list_databases {
        let databases = vfs.list_databases();

        if databases.is_empty() {
            println!("No databases found in archive");
        } else {
            println!("Databases:");
            for db in databases {
                println!("  {}", db);
            }
        }

        return Ok(());
    }

    // Execute query
    if let (Some(db_path), Some(query_str)) = (database, sql) {
        println!("Querying database: {}", db_path);

        let conn = vfs.open_database(db_path)?;

        let mut stmt = conn.prepare(query_str)?;

        let column_count = stmt.column_count();
        let column_names: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();

        match format {
            "json" => {
                let mut results = Vec::new();

                let rows = stmt.query_map([], |row| {
                    let mut map = serde_json::Map::new();
                    for (i, name) in column_names.iter().enumerate() {
                        let value: String = row.get(i).unwrap_or_default();
                        map.insert(name.clone(), serde_json::Value::String(value));
                    }
                    Ok(serde_json::Value::Object(map))
                })?;

                for row in rows {
                    results.push(row?);
                }

                println!("{}", serde_json::to_string_pretty(&results)?);
            }

            "csv" => {
                // Print header
                println!("{}", column_names.join(","));

                // Print rows
                let rows = stmt.query_map([], |row| {
                    let values: Vec<String> = (0..column_count)
                        .map(|i| row.get::<_, String>(i).unwrap_or_default())
                        .collect();
                    Ok(values.join(","))
                })?;

                for row in rows {
                    println!("{}", row?);
                }
            }

            _ => {
                // Table format (default)
                println!("{}", column_names.join(" | "));
                println!("{}", "-".repeat(60));

                let rows = stmt.query_map([], |row| {
                    let values: Vec<String> = (0..column_count)
                        .map(|i| row.get::<_, String>(i).unwrap_or_default())
                        .collect();
                    Ok(values.join(" | "))
                })?;

                for row in rows {
                    println!("{}", row?);
                }
            }
        }
    } else {
        anyhow::bail!("Must specify both --database and --sql for queries");
    }

    Ok(())
}
