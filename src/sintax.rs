use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use bio::io::fasta::Record;
use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::prelude::*;
use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn bootstrap_classify_query(
    query_hashes: &mut Vec<&u64>,
    query_name: &str,
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    config: &Config,
) -> String {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    let mut iterations: Vec<String> = Vec::with_capacity(config.num_bootstraps);

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
        // NOTE - we should probably do this in some other way, such as
        // initializing an zero-array of num_references length.
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
        // For now, we take the best hit. We can probably
        // use something better than a hashmap.
        match map.iter().max_by_key(|entry| entry.1) {
            Some((subject_id, subject_score)) => {
                let result_s = format!(
                    "{}\t{}\t{}\t{}",
                    query_name,
                    valid_records.get(*subject_id).unwrap().id(),
                    subject_score,
                    i + 1
                );
                iterations.push(result_s);
            }
            None => {}
        };
    }
    let bootstrap_result = iterations.join("\n");
    return bootstrap_result;
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    query_reader: Reader<BufReader<File>>,
    writer: &Arc<Mutex<BufWriter<File>>>,
) {
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    query_reader.records().par_bridge().for_each(|record| {
        if let Ok(r) = record {
            let query_hashes: HashSet<u64> = kmerize(&config, &r.seq());

            // This should be relatively fast if sequences are short.
            let mut query_vec: Vec<&u64> = query_hashes.iter().collect();

            let bootstrap_result = bootstrap_classify_query(
                &mut query_vec,
                &r.id(),
                &reverse_index,
                &valid_records,
                config,
            );

            let mut w = writer.lock().unwrap();
            writeln!(w, "{}", bootstrap_result).unwrap()
        }
    });

    spinner.finish();
}
