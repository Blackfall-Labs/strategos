use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct CliArguments {
    pattern: String,
    path: PathBuf
}

fn main() {

    let args = CliArguments::parse();

    println!("PASSED ARGUMENTS TO ENGRAM CLI: ");
    println!("-> pattern: {:?}", args.pattern);
    println!("-> path: {:?}", args.path);
}