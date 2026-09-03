// TODO: Automatically identify each builds unique seed
const SW2_SEED: u32 = 0x7d895397;

pub fn hash(input: &str) -> u32 {
    let bytes = input.as_bytes();

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