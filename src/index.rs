use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::{Reader, Record};

use fixedbitset::FixedBitSet;
use log::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::fs::File;
use std::io::BufReader;
/// We need to build a reverse index by:
/// * First reading all reference sequences as HashMap or similar structure.
/// * Initialize HashMap of key kmer_hash, value is empty bitmap with capacity num_sequences.
/// * Loop over each reference sequence, generate hashes and insert in the corresponding position.

pub fn build_reverse_index(reference_reader: Reader<BufReader<File>>, config: &Config) {
    info!("Loading sequences...");
    let valid_records: Vec<Record> = reference_reader
        .records()
        .filter_map(|record| record.ok())
        .collect();

    let num_records = valid_records.len();

    // Capacity should be the unique number of hashes from all reference sequences.
    // We don't know this, so for now we set it to the number of records. There is probably
    // some way to estimate this, e.g., the number of sequences and their lengths.
    let mut map: FxHashMap<u64, FixedBitSet> =
        FxHashMap::with_capacity_and_hasher(num_records, FxBuildHasher);

    // For now, just use normal iterator
    info!("Creating index...");
    for (i, r) in valid_records.iter().enumerate() {
        let hashes = kmerize(config, &r.seq());

        hashes.iter().for_each(|h| {
            map.entry(*h)
                .and_modify(|bitset| bitset.set(i, true))
                .or_insert_with(|| FixedBitSet::with_capacity(num_records));
        });
    }
}
