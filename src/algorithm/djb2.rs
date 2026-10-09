pub fn hash(s: &str, seed: u32) -> u32 {
    let mut hash = seed;

    for &byte in s.as_bytes() {
        hash = hash
            .wrapping_mul(33)
            .wrapping_add(byte as u32);
    }

    hash
}