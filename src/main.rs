mod file;
mod hashes;
mod algorithm;
mod extract;
mod compare;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Specify the target executable");
        exit(1);
    }

    let _hashes = match file::open_hashes_file() {
        Ok(file) => file,

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            file::create_hashes_csv()
        }

        Err(e) => {
            eprintln!("Could not open hashes.csv: {}", e);
            exit(1);
        }
    };

    let target = &args[1];

    let candidates = match extract::scan(target) {
        Ok(candidates) => candidates,
        Err(e) => {
            eprintln!("Failed to scan {target}: {e}");
            return;
        }
    };

    let matches = compare::compare(&candidates);

    if matches.is_empty() {
        println!("No API hash matches found.");
        return;
    }

    println!("API hash matches:");

    for matched in matches {
        println!(
            "{:<30} {:<15} 0x{:08X}",
            matched.api,
            matched.algorithm.name(),
            matched.hash
        );
    }

}