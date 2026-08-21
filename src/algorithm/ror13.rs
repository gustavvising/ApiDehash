pub fn hash(input: &str) -> u32 {
    let mut hash: u32 = 0;

    for c in input.bytes() {
        hash = hash.rotate_right(13).wrapping_add(c as u32);
    }

    hash
}