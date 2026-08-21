mod file;
mod hashes;
mod algorithm;

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
}