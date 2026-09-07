mod algorithm;
mod compare;
mod extract;

use algorithm::HashAlgorithm;
use std::{env, error::Error, process::ExitCode};

const USAGE: &str = "Usage: ApiDehash <executable> [--djb2-seed <seed>] [--sw2-seed <seed>]
Seeds accept decimal or 0x-prefixed hexadecimal values.
DJB2 defaults to 5381; SysWhispers2 requires --sw2-seed.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let target = args.next().ok_or(USAGE)?;
    if target == "--help" || target == "-h" {
        println!("{USAGE}");
        return Ok(());
    }

    let mut djb2_seed = 5381;
    let mut sw2_seed = None;
    while let Some(option) = args.next() {
        if option != "--djb2-seed" && option != "--sw2-seed" {
            return Err(format!("Unknown option: {option}\n{USAGE}").into());
        }
        let value = args
            .next()
            .ok_or_else(|| format!("Missing seed for {option}"))?;
        let seed = match value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
        {
            Some(hex) => u32::from_str_radix(hex, 16),
            None => value.parse(),
        }
        .map_err(|_| format!("Invalid 32-bit seed for {option}: {value}"))?;
        if option == "--djb2-seed" {
            djb2_seed = seed;
        } else {
            sw2_seed = Some(seed);
        }
    }

    let mut algorithms = vec![HashAlgorithm::Djb2(djb2_seed)];
    if let Some(seed) = sw2_seed {
        algorithms.push(HashAlgorithm::SysWhispers2(seed));
    }
    let candidates = extract::scan(&target).map_err(|e| format!("Failed to scan {target}: {e}"))?;
    let matches = compare::compare(&candidates, &algorithms);
    if matches.is_empty() {
        println!("No API hash matches found.");
        return Ok(());
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
    Ok(())
}
