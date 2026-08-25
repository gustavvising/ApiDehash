pub mod syswhispers2;

#[derive(Clone, Copy)]
pub enum HashAlgorithm {
    SysWhispers2,
}

impl HashAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            Self::SysWhispers2 => "syswhispers2",
        }
    }

    pub fn hash(&self, input: &str) -> u32 {
        match self {
            Self::SysWhispers2 => syswhispers2::hash(input),
        }
    }
}