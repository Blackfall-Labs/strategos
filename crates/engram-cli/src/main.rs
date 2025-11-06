use anyhow::{Context, Result};
use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
struct CliArguments {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    match args.pattern.as_str() {
        "ls" | "list" => {
            println!("We should attempt to list files from inside an Engram");
            return Ok(());
        },
        "i" | "info" => {
            println!("We should attempt to list information about an Engram");
            return Ok(());
        },
        "p" | "pack" => {
            println!("We should attempt to pack given files/directory into a new Engram.");
            return Ok(());
        },
        "inspect" => {
            println!("Get additional information appended to `info` argument");
            return Ok(());
        }
        &_ => {} // do nothing, but should probably fail gracefully
    }

    let content = read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display().to_string()))?;

    // prints the entire raw contents of the file
    // println!("file content: {}", content);

    // find_matches(&content, &args.pattern);
    find_matches(&content, &args.pattern, &mut std::io::stdout()).with_context(|| {
        format!(
            "failed to find matching content for pattern `{}`",
            &args.pattern
        )
    })?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}
