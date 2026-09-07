pub const fn hash(s: &str, seed: u32) -> u32 {
    let bytes = s.as_bytes();
    let mut hash = seed;
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as u32;
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c);
        i += 1;
    }

    hash
}
