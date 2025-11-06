use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
struct CliArguments {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = CliArguments::parse();

    // println!("PASSED ARGUMENTS TO ENGRAM CLI: ");
    // println!("-> pattern: {:?}", args.pattern);
    // println!("-> path: {:?}", args.path);

    let content = read_to_string(&args.path).expect("could not read file ");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
