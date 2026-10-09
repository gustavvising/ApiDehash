use clap::{CommandFactory, Parser, Subcommand};
use std::collections::HashSet;
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
    override_usage = "ApiDehash.exe <COMMAND> [OPTIONS]",
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
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
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate and save hashes.csv
    File,

    /// Generate API hashes in memory
    Hashes,

    /// Display the selected hashing algorithms and seeds
    Algorithm,

    /// Extract 32-bit hash candidates from an executable
    Extract {
        target: String,
    },

    /// Extract candidates and compare them against generated API hashes
    Compare {
        target: String,
    },

    /// Extract candidates and compare them against generated API hashes
    Scan {
        target: String,
    },

    /// Print this message or the help of the given subcommand(s)
    Help {
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },

    /// Print version information
    Version,
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

fn print_help(command: Option<&str>) -> Result<(), String> {
    let mut cmd = Cli::command();

    if let Some(name) = command {
        let subcommand = cmd
            .find_subcommand_mut(name)
            .ok_or_else(|| format!("Unknown command '{name}'"))?;

        subcommand
            .print_help()
            .map_err(|e| e.to_string())?;
    } else {
        cmd.print_help()
            .map_err(|e| e.to_string())?;
    }

    println!();
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Handle help and version before validating the hash configuration.
    match &cli.command {
        Commands::Help { command } => {
            return match print_help(command.as_deref()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(message) => {
                    eprintln!("Error: {message}");
                    ExitCode::FAILURE
                }
            };
        }
        Commands::Version => {
            println!(
                "{} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            );
            return ExitCode::SUCCESS;
        }
        _ => {}
    }

    let config = HashConfig {
        syswhispers2_seed: cli.syswhispers2,
        djb2_seed: cli.djb2,
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

        Commands::Hashes => {
            match hashes::generate_hashes(&config) {
                Ok(_) => Ok(()),
                Err(e) => {
                    Err(format!("Could not generate hashes: {e}"))
                }
            }
        }

        Commands::Algorithm => {
            println!("Selected algorithms:");

            if let Some(seed) = config.syswhispers2_seed {
                println!("  syswhispers2: 0x{seed:08X}");
            }

            if let Some(seed) = config.djb2_seed {
                println!("  djb2:         0x{seed:08X}");
            }

            Ok(())
        }

        Commands::Extract { target } => {
            match extract::scan(&target) {
                Ok(candidates) => {
                    println!(
                        "Extracted {} unique candidates:",
                        candidates.len()
                    );

                    for candidate in candidates {
                        println!("0x{candidate:08X}");
                    }

                    Ok(())
                }
                Err(e) => {
                    Err(format!("Failed to scan {target}: {e}"))
                }
            }
        }

        Commands::Compare { target }
        | Commands::Scan { target } => {
            run_compare(&target, &config)
        }

        Commands::Help { .. } | Commands::Version => unreachable!(),
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