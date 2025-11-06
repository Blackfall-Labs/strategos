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

    let content = read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display().to_string()))?;

    // prints the entire raw contents of the file
    // println!("file content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
