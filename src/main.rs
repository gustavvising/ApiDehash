use std::env;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Specify the target executable");
        exit(1);
    }

    let hashes = match File::open("hashes.txt") {
        Ok(file) => file,

        Err(e) if e.kind() == ErrorKind::NotFound => {
            // hashes.txt does not exist, creating it
            create_hashes_file()
        }

        Err(e) => {
            eprintln!("Could not open hashes.txt: {}", e);
            exit(1);
        }
    };

}

fn create_hashes_file() -> File {
    let mut file = match File::create("hashes.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not create hashes.txt: {}", e);
            exit(1);
        }
    };

    if let Err(e) = file.write_all(b"Hello, world!") {
        eprintln!("Could not write to hashes.txt: {}", e);
        exit(1);
    }

    file
}
