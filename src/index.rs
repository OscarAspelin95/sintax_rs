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
) -> DashMap<String, HashSet<u64>> {
    // For now, store ref id as string, hashes as set.
    let map: DashMap<String, HashSet<u64>> = DashMap::with_capacity(400_000);

    reference_reader.records().par_bridge().for_each(|record| {
        let r = record.unwrap();
        let hashes = kmerize(config, &r.seq());

        // We don't like this .to_owned() because we convert &str to String.
        map.insert(r.id().to_owned(), hashes);
    });

    return map;
}
