// TODO: Automatically identify each builds unique seed
const DJB2_SEED: u32 = 0x7d895397;

pub const fn hash(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut hash = DJB2_SEED;
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as u32;
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c);
        i += 1;
    }

    hash
}
