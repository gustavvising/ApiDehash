use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;

mod algorithm;
mod compare;
mod extract;
mod file;
mod hashes;

use algorithm::HashConfig;

#[derive(Parser, Debug)]
#[command(
    name = "ApiDehash",
    about = "API Dehash - CLI",
    version,
    override_usage = "ApiDehash.exe <COMMAND> [OPTIONS]"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// SysWhispers2 seed (decimal or hexadecimal)
    #[arg(
        long,
        global = true,
        value_name = "SEED",
        value_parser = parse_seed
    )]
    syswhispers2: Option<u32>,

    /// DJB2 seed (decimal or hexadecimal)
    #[arg(
        long,
        global = true,
        value_name = "SEED",
        value_parser = parse_seed
    )]
    djb2: Option<u32>,

    /// Custom API list, one API name per line (default: built-in list)
    #[arg(
        long,
        global = true,
        value_name = "FILE"
    )]
    apis: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// Extract candidates from an executable and compare them against APIs
    Scan {
        target: String,
    },

    /// Generate and save a hash lookup table to hashes.csv
    File
}

fn parse_seed(value: &str) -> Result<u32, String> {
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        if hex.is_empty() {
            return Err("Hexadecimal seed cannot be empty".to_string());
        }

        u32::from_str_radix(hex, 16)
            .map_err(|e| {
                format!("Invalid hexadecimal seed '{value}': {e}")
            })
    } else {
        value.parse::<u32>().map_err(|e| {
            format!("Invalid decimal seed '{value}': {e}")
        })
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let config = HashConfig {
        syswhispers2_seed: cli.syswhispers2,
        djb2_seed: cli.djb2,
        api_list: cli.apis,
    };

    if let Err(message) = config.validate() {
        eprintln!("Error: {message}");
        return ExitCode::FAILURE;
    }

    let result = match cli.command {
        Commands::File => {
            match file::create_hashes_csv(&config) {
                Ok(()) => {
                    println!("Created hashes.csv");
                    Ok(())
                }
                Err(e) => {
                    Err(format!("Could not create hashes.csv: {e}"))
                }
            }
        }

        Commands::Scan { target } => {
            run_compare(&target, &config)
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("Error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run_compare(
    target: &str,
    config: &HashConfig,
) -> Result<(), String> {
    let candidates: HashSet<u32> = extract::scan(target)
        .map_err(|e| {
            format!("Failed to scan {target}: {e}")
        })?;

    let matches = compare::compare(&candidates, config)
        .map_err(|e| {
            format!("Comparison failed: {e}")
        })?;

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