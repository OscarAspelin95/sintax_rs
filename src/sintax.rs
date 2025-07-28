use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use bio::io::fasta::Record;
use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use rand::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader, Write};

pub fn bootstrap_classify_query(
    query_hashes: &mut Vec<&u64>,
    query_name: &str,
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    config: &Config,
    writer: &mut BufWriter<File>,
) {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    // Bootstrap iterations.
    for i in 0..config.num_bootstraps {
        // Randomly pick hashes (currently without replacement).
        // Not sure this is the best way to do it.
        let random_hashes: Vec<&u64> = (0..config.num_query_hashes)
            .map(|_| {
                let i = rng.random_range(0..query_hashes.len());
                query_hashes[i]
            })
            .collect();

        // We have our random hashes, now we need to check which references
        // they match against (from the reverse index) and increment their counts.
        let mut map: FxHashMap<usize, usize> =
            FxHashMap::with_capacity_and_hasher(10_000, FxBuildHasher);

        for random_hash in random_hashes {
            match reverse_index.get(&random_hash) {
                None => continue,
                Some(bitset) => {
                    for ref_index in bitset.ones() {
                        map.entry(ref_index)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        // We need to sort HashMap by values (as vec)?
        // Then take <top_candidates> best ones and print
        // For now, we take the best hit.
        match map.iter().max_by_key(|entry| entry.1) {
            Some((subject_id, subject_score)) => writeln!(
                writer,
                "{}\t{}\t{}\t{}",
                query_name,
                valid_records.get(*subject_id).unwrap().id(),
                subject_score,
                i + 1
            )
            .unwrap(),
            None => {}
        };
    }
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    query_reader: Reader<BufReader<File>>,
    writer: &mut BufWriter<File>,
) {
    query_reader.records().for_each(|record| {
        if let Ok(r) = record {
            let query_hashes: HashSet<u64> = kmerize(&config, &r.seq());

            // This should be relatively fast if sequences are short.
            let mut query_vec: Vec<&u64> = query_hashes.iter().collect();

            bootstrap_classify_query(
                &mut query_vec,
                &r.id(),
                &reverse_index,
                &valid_records,
                config,
                writer,
            );
        }
    });
}
