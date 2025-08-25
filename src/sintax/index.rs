use crate::sintax::kmerize;
use crate::utils::Config;
use bio::io::fasta::{Reader, Record};

use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use indicatif::{ProgressBar, ProgressStyle};
use log::*;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn build_reverse_index(
    database_reader: Reader<BufReader<File>>,
    config: &Config,
) -> (DashMap<u64, FixedBitSet, FxBuildHasher>, Vec<Record>) {
    info!("Loading sequences...");
    let valid_records: Vec<Record> = database_reader
        .records()
        .filter_map(|record| record.ok())
        .collect();

    let num_records = valid_records.len();

    // We'll create the index in parallel by using DashMap.
    let map = DashMap::with_capacity_and_hasher(num_records, FxBuildHasher);

    // For now, just use normal iterator
    info!("Creating index...");
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    valid_records.par_iter().enumerate().for_each(|(i, r)| {
        let hashes = kmerize(config, &r.seq());

        hashes.iter().for_each(|h| {
            map.entry(*h)
                .and_modify(|bitset: &mut FixedBitSet| bitset.set(i, true))
                .or_insert_with(|| {
                    let mut bitset = FixedBitSet::with_capacity(num_records);
                    bitset.set(i, true);
                    bitset
                });
        });
    });

    spinner.finish();

    return (map, valid_records);
}
