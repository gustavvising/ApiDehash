use crate::algorithm::{HashAlgorithm, HashConfig};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn generate_hashes(
    config: &HashConfig,
) -> io::Result<Vec<(String, Vec<(HashAlgorithm, u32)>)>> {
    config
        .validate()
        .map_err(|message| {
            io::Error::new(io::ErrorKind::InvalidInput, message)
        })?;

    let file = File::open("apis.txt")?;
    let reader = BufReader::new(file);
    let algorithms = config.algorithms();
    let mut results = Vec::new();

    for line in reader.lines() {
        let name = line?.trim().to_string();

        if name.is_empty() {
            continue;
        }

        let hashes = algorithms
            .iter()
            .filter_map(|algorithm| {
                config
                    .hash(*algorithm, &name)
                    .map(|hash| (*algorithm, hash))
            })
            .collect();

        results.push((name, hashes));
    }

    Ok(results)
}