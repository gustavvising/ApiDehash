use std::env;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

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

struct ApiHash {
    name: String,
    ror13: u32,
    syswhispers2: u32,
}

fn ror13_hash(s: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in s.bytes() {
        hash = hash.rotate_right(13).wrapping_add(c as u32);
    }
    hash
}

/*
DWORD SW2_HashSyscall(PCSTR FunctionName)
{
    DWORD i = 0;
    DWORD Hash = SW2_SEED;

    while (FunctionName[i])
    {
        WORD PartialName = *(WORD*)((ULONG64)FunctionName + i++);
        Hash ^= PartialName + SW2_ROR8(Hash);
    }

    return Hash;
}
*/
fn syswhispers2_hash(s: &str) -> u32 {
    const SW2_SEED: u32 = 0x7d895397; // TODO: Make this automatically identified to identify each builds unique seed
    let bytes = s.as_bytes();
    let mut hash: u32 = SW2_SEED;
    let mut i = 0;

    while i < bytes.len() {
        let partial_name = if i + 1 < bytes.len() {
            u16::from_le_bytes([bytes[i], bytes[i + 1]])
        } else {
            bytes[i] as u16
        };

        let ror8_hash = hash.rotate_right(8);

        hash ^= (partial_name as u32).wrapping_add(ror8_hash);

        i += 1;
    }

    hash
}

fn generate_hashes() -> Vec<ApiHash> {
    let file = File::open("api_names.txt").unwrap_or_else(|e| {
        eprintln!("Could not open api_names.txt: {}", e);
        exit(1);
    });

    let mut results = Vec::new();
    for line in BufReader::new(file).lines() {
        let name = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        if name.is_empty() {
            continue;
        }

        results.push(ApiHash {
            ror13: ror13_hash(&name),
            syswhispers2: syswhispers2_hash(&name),
            name,
        });
    }

    results
}

fn create_hashes_file() -> File {
    let mut file = match File::create("hashes.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not create hashes.txt: {}", e);
            exit(1);
        }
    };

    for entry in generate_hashes() {
        let line = format!("{},{:08x},{:08x}\n", entry.name, entry.ror13, entry.syswhispers2);
        if let Err(e) = file.write_all(line.as_bytes()) {
            eprintln!("Could not write to hashes.txt: {}", e);
            exit(1);
        }
    }

    file
}