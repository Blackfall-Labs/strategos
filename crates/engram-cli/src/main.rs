use engram_core::ArchiveReader;

fn main() {

    let binary = std::env::args().nth(0).expect("no binary given");
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("PASSED ARGUMENTS TO ENGRAM CLI: ");
    println!("-> binary: {:?}", binary);
    println!("-> pattern: {:?}", pattern);
    println!("-> path: {:?}", path);
}