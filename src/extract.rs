use object::{Object, ObjectSection, SectionKind};
use std::collections::HashSet;
use std::fs;

pub fn scan(target: &str) -> Result<HashSet<u32>, Box<dyn std::error::Error>> {
    let data = fs::read(target)?;
    let file = object::File::parse(&*data)?;

    let mut candidates = HashSet::new();

    for section in file.sections() {
        if matches!(
            section.kind(),
            SectionKind::Text | SectionKind::ReadOnlyData | SectionKind::Data
        ) {
            for bytes in section.data()?.windows(4) {
                candidates.insert(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
            }
        }
    }

    Ok(candidates)
}
