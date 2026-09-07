use crate::algorithm::HashAlgorithm;
use std::collections::HashSet;

pub struct Match {
    pub api: &'static str,
    pub algorithm: HashAlgorithm,
    pub hash: u32,
}

pub fn compare(candidates: &HashSet<u32>, algorithms: &[HashAlgorithm]) -> Vec<Match> {
    let mut matches = Vec::new();

    for api in include_str!("../apis.txt")
        .lines()
        .map(str::trim)
        .filter(|api| !api.is_empty())
    {
        for &algorithm in algorithms {
            let hash = algorithm.hash(api);
            if candidates.contains(&hash) {
                matches.push(Match {
                    api,
                    algorithm,
                    hash,
                });
            }
        }
    }

    matches
}
