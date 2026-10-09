use object::{Object, ObjectSection, SectionKind};
use std::collections::HashSet;
use std::fs;

pub fn scan(
    target: &str,
) -> Result<HashSet<u32>, Box<dyn std::error::Error>> {
    let data = fs::read(target)?;
    let file = object::File::parse(&*data)?;

    let mut candidates = HashSet::new();

    for section in file.sections() {
        let relevant = matches!(
            section.kind(),
            SectionKind::Text
                | SectionKind::ReadOnlyData
                | SectionKind::Data
                | SectionKind::UninitializedData
        );

        if !relevant {
            continue;
        }

        let bytes = section.data()?;

        for chunk in bytes.windows(4) {
            let value = u32::from_le_bytes([
                chunk[0],
                chunk[1],
                chunk[2],
                chunk[3],
            ]);

            candidates.insert(value);
        }
    }

    Ok(candidates)
}