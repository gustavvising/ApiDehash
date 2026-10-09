pub mod syswhispers2;
mod djb2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashAlgorithm {
    SysWhispers2,
    DJB2,
}

impl HashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SysWhispers2 => "syswhispers2",
            Self::DJB2 => "djb2",
        }
    }

    pub fn hash(&self, input: &str, seed: u32) -> u32 {
        match self {
            Self::SysWhispers2 => syswhispers2::hash(input, seed),
            Self::DJB2 => djb2::hash(input, seed),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HashConfig {
    pub syswhispers2_seed: Option<u32>,
    pub djb2_seed: Option<u32>,
}

impl HashConfig {
    pub fn algorithms(&self) -> Vec<HashAlgorithm> {
        let mut algorithms = Vec::new();

        if self.syswhispers2_seed.is_some() {
            algorithms.push(HashAlgorithm::SysWhispers2);
        }

        if self.djb2_seed.is_some() {
            algorithms.push(HashAlgorithm::DJB2);
        }

        algorithms
    }

    pub fn seed(&self, algorithm: HashAlgorithm) -> Option<u32> {
        match algorithm {
            HashAlgorithm::SysWhispers2 => self.syswhispers2_seed,
            HashAlgorithm::DJB2 => self.djb2_seed,
        }
    }

    pub fn hash(
        &self,
        algorithm: HashAlgorithm,
        input: &str,
    ) -> Option<u32> {
        self.seed(algorithm)
            .map(|seed| algorithm.hash(input, seed))
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.syswhispers2_seed.is_none()
            && self.djb2_seed.is_none()
        {
            return Err(
                "Specify at least one seed: \
                 --syswhispers2 <SEED> or --djb2 <SEED>"
            );
        }

        Ok(())
    }
}