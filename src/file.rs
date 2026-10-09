use crate::algorithm::HashConfig;
use crate::hashes::generate_hashes;
use std::fs::File;
use std::io::{self, Write};

pub fn create_hashes_csv(config: &HashConfig) -> io::Result<()> {
    let generated = generate_hashes(config)?;
    let mut file = File::create("hashes.csv")?;

    let algorithms = config.algorithms();

    let mut header = String::from("API");

    for algorithm in &algorithms {
        header.push(',');
        header.push_str(algorithm.name());
    }

    header.push('\n');
    file.write_all(header.as_bytes())?;

    for (name, hashes) in generated {
        let mut line = name;

        for (_, hash) in hashes {
            line.push_str(&format!(",{:08x}", hash));
        }

        line.push('\n');
        file.write_all(line.as_bytes())?;
    }

    file.flush()
}