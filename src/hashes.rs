use crate::algorithm::{HashAlgorithm, HashConfig};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const BUILTIN_APIS: &str = include_str!("builtin_apis.txt");

pub fn generate_hashes(
    config: &HashConfig,
) -> io::Result<Vec<(String, Vec<(HashAlgorithm, u32)>)>> {
    config
        .validate()
        .map_err(|message| {
            io::Error::new(io::ErrorKind::InvalidInput, message)
        })?;

    let reader: Box<dyn BufRead> = match &config.api_list {
        Some(path) => {
            let file = File::open(path).map_err(|e| {
                io::Error::new(e.kind(), format!("{}: {e}", path.display()))
            })?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BUILTIN_APIS.as_bytes()),
    };
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