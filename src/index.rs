use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use dashmap::DashMap;
use rayon::prelude::*;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn build_reverse_index(
    reference_reader: Reader<BufReader<File>>,
    config: &Config,
) -> DashMap<u64, HashSet<String>> {
    let map = DashMap::with_capacity(400_000);

    reference_reader.records().par_bridge().for_each(|record| {
        if let Ok(r) = record {
            let id_str = r.id().to_string();
            let hashes = kmerize(config, &r.seq());

            for hash in hashes {
                map.entry(hash)
                    .or_insert_with(HashSet::new)
                    .insert(id_str.clone());
            }
        }
    });

    map
}
