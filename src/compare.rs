use crate::algorithm::HashAlgorithm;
use crate::hashes::generate_hashes;
use std::collections::HashSet;

pub struct Match {
    pub api: String,
    pub algorithm: HashAlgorithm,
    pub hash: u32,
}

pub fn compare(candidates: &HashSet<u32>) -> Vec<Match> {
    let hashes = generate_hashes();
    let mut matches = Vec::new();

    for (api, api_hashes) in hashes {
        for &(algorithm, hash) in &api_hashes {
            if candidates.contains(&hash) {
                matches.push(Match {
                    api: api.clone(),
                    algorithm,
                    hash,
                });
            }
        }
    }
    
    matches
}
