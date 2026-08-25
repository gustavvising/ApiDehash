use crate::algorithm::HashAlgorithm;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn generate_hashes() -> Vec<(String, Vec<(HashAlgorithm, u32)>)> {
    let file = File::open("apis.txt").unwrap_or_else(|e| {
        eprintln!("Could not open apis.txt: {}", e);
        std::process::exit(1);
    });

    let algorithms = [
        HashAlgorithm::SysWhispers2,
    ];

    let mut results = Vec::new();

    for line in BufReader::new(file).lines() {
        let name = match line {
            Ok(line) => line.trim().to_string(),
            Err(e) => {
                eprintln!("Could not read line: {}", e);
                continue;
            }
        };

        if name.is_empty() {
            continue;
        }

        let hashes = algorithms
            .iter()
            .map(|algorithm| (*algorithm, algorithm.hash(&name)))
            .collect();

        results.push((name, hashes));
    }

    results
}