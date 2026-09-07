mod djb2;
mod syswhispers2;

#[derive(Clone, Copy)]
pub enum HashAlgorithm {
    SysWhispers2(u32),
    Djb2(u32),
}

impl HashAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            Self::SysWhispers2(_) => "syswhispers2",
            Self::Djb2(_) => "djb2",
        }
    }

    pub fn hash(&self, input: &str) -> u32 {
        match self {
            Self::SysWhispers2(seed) => syswhispers2::hash(input, *seed),
            Self::Djb2(seed) => djb2::hash(input, *seed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HashAlgorithm::{Djb2, SysWhispers2};

    #[test]
    fn known_hashes_with_standard_and_custom_seeds() {
        assert_eq!(Djb2(5381).hash("VirtualAlloc"), 0x382c0f97);
        assert_eq!(Djb2(0x7d895397).hash("VirtualAlloc"), 0x93b5b929);
        assert_eq!(
            SysWhispers2(0x7d895397).hash("ZwAllocateVirtualMemory"),
            0xf768efe9
        );
        assert_eq!(
            SysWhispers2(0x12345678).hash("ZwAllocateVirtualMemory"),
            0x0d9f0319
        );
    }
}
