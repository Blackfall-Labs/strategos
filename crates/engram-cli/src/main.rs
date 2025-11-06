use std::path::PathBuf;
use engram_core::ArchiveReader;

struct CliArguments {
    binary: String,
    pattern: String,
    path: PathBuf
}

fn main() {

    let binary = std::env::args().nth(0).expect("no binary given");
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = CliArguments {
        binary,
        pattern,
        path: PathBuf::from(path)
    };

    println!("PASSED ARGUMENTS TO ENGRAM CLI: ");
    println!("-> binary: {:?}", args.binary);
    println!("-> pattern: {:?}", args.pattern);
    println!("-> path: {:?}", args.path);
}