pub mod ror13;
pub mod syswhispers2;

#[derive(Clone, Copy)]
pub enum HashAlgorithm {
    Ror13,
    SysWhispers2,
}

impl HashAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            Self::Ror13 => "ror13",
            Self::SysWhispers2 => "syswhispers2",
        }
    }

    pub fn hash(&self, input: &str) -> u32 {
        match self {
            Self::Ror13 => ror13::hash(input),
            Self::SysWhispers2 => syswhispers2::hash(input),
        }
    }
}