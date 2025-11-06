use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
struct CliArguments {
    pattern: String,
    path: PathBuf,
}

// fn main() {
//     let args = CliArguments::parse();
//
//     // println!("PASSED ARGUMENTS TO ENGRAM CLI: ");
//     // println!("-> pattern: {:?}", args.pattern);
//     // println!("-> path: {:?}", args.path);
//
//     let result = std::fs::read_to_string("test.txt");
//     let content = match result {
//         Ok(content) => { content },
//         Err(error) => { panic!("Can't deal with {}, just exit here", error); }
//     };
//     println!("file content: {}", content);
//     let content = std::fs::read_to_string("test.txt").unwrap();
//
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CliArguments::parse();

    // verbose, not pretty
    // let result = read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { return Err(error.into()); }
    // };

    // concise, the "?" operator handles error catching
    let content = read_to_string(&args.path)?;

    // prints the entire raw contents of the file
    // println!("file content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
