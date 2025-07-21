use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use rayon::prelude::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

/// We have a very slow implementation for now that:
/// * Iterates over each record in series.
/// * Kmerizes the records sequence.
/// * Adds kmer hashes as keys, hashset as values. Inserts record ID into hashset.
///
/// HashMap is <kmer_hash, hash_set_of_what_sequences_contain_kmer_hash>.
///
/// We have some issues:
/// * Not parallelized.
/// * HashSet contains Strings, not &str.
pub fn build_reverse_index(
    reference_reader: Reader<BufReader<File>>,
    config: &Config,
) -> HashMap<u64, HashSet<String>> {
    let mut map = HashMap::with_capacity(400_000);

    reference_reader.records().for_each(|record| {
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

    return map;
}
