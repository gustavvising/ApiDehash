use object::{Object, ObjectSection};
use std::collections::HashSet;
use std::fs;

pub fn scan(target: &str) -> Result<HashSet<u32>, Box<dyn std::error::Error>> {
    let data = fs::read(target)?;
    let file = object::File::parse(&*data)?;

    let mut candidates = HashSet::new();

    for section in file.sections() {
        let name = section.name().unwrap_or("");

        if name == ".text" || name == ".rdata" || name == ".data" {
            if let Ok(bytes) = section.data() {
                for i in 0..bytes.len().saturating_sub(3) {
                    let value = u32::from_le_bytes([
                        bytes[i],
                        bytes[i + 1],
                        bytes[i + 2],
                        bytes[i + 3],
                    ]);

                    candidates.insert(value);
                }
            }
        }
    }

    Ok(candidates)
}