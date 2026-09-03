use crate::hashes::generate_hashes;
use crate::algorithm::HashAlgorithm;
use std::fs::File;
use std::io::{self, Write};

pub fn open_hashes_file() -> io::Result<File> {
    File::open("hashes.csv")
}

pub fn create_hashes_csv() -> File {
    let mut file = match File::create("hashes.csv") {
        Ok(file) => file,

        Err(e) => {
            eprintln!("Could not create hashes.csv: {}", e);
            std::process::exit(1);
        }
    };

    let algorithms = [
        HashAlgorithm::SysWhispers2,
        HashAlgorithm::DJB2
    ];

    // CSV Headers
    let mut header = String::from("API");

    for algorithm in &algorithms {
        header.push(',');
        header.push_str(algorithm.name());
    }
    header.push('\n');

    if let Err(e) = file.write_all(header.as_bytes()) {
        eprintln!("Could not write to hashes.csv: {}", e);
        std::process::exit(1);
    }

    // Hashes
    for (name, hashes) in generate_hashes() {
        let mut line = name;

        for (_, hash) in hashes {
            line.push_str(&format!(",{:08x}", hash));
        }
        line.push('\n');

        if let Err(e) = file.write_all(line.as_bytes()) {
            eprintln!("Could not write to hashes.csv: {}", e);
            std::process::exit(1);
        }
    }

    file
}