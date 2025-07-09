use fixedbitset::FixedBitSet;
use std::collections::HashMap;

use crate::kmers::kmerize;
use crate::utils::Config;

/// Check if this can be parallelized (with DashMap?)
pub fn build_reverse_index(references: &Vec<&[u8]>, config: &Config) -> HashMap<u64, FixedBitSet> {
    let bitset_len = references.len();
    let mut map: HashMap<u64, FixedBitSet> = HashMap::new();

    references.iter().enumerate().for_each(|(i, ref_seq)| {
        let hashes = kmerize(config, ref_seq);

        hashes.iter().for_each(|h| {
            // If hash exists, flip bit at position i
            // If hash does not exists, create new fixedbitset.
            if !map.contains_key(&h) {
                map.insert(*h, FixedBitSet::with_capacity(bitset_len));
            }

            let bitset = map.get_mut(&h).unwrap();
            bitset.set(i, true);
        });
    });

    return map;
}
