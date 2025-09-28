use crate::database::KmerBitSet;
use crate::sintax::kmerize;
use crate::utils::fasta_reader;

use anyhow::Result;
use bio::io::fasta::Record;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use log::*;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::path::PathBuf;
use std::time::Duration;

pub fn build_reverse_index(
    fasta: &PathBuf,
    kmer_size: usize,
    downsampling_factor: u64,
) -> Result<(DashMap<u64, KmerBitSet, FxBuildHasher>, Vec<String>)> {
    // Create database reader

    let database_reader = fasta_reader(&fasta)?;

    info!("Loading sequences...");
    let valid_records: Vec<Record> = database_reader
        .records()
        .filter_map(|record| record.ok())
        .collect();

    let num_records = valid_records.len();

    // Generate and store kmer hashes in parallel with DashMap.
    // Ideally, the DashMap capacity should be the number of hashes we can create.
    // Since we don't know this beforehand, we set the capacity to the number of records instead.
    let map = DashMap::with_capacity_and_hasher(num_records, FxBuildHasher);

    // For now, just use normal iterator
    info!("Creating index...");
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    valid_records.par_iter().enumerate().for_each(|(i, r)| {
        let hashes = kmerize(kmer_size, downsampling_factor, &r.seq());

        hashes.iter().for_each(|h| {
            map.entry(*h)
                .and_modify(|bitset: &mut KmerBitSet| unsafe { bitset.set_unchecked(i) })
                .or_insert_with(|| {
                    let mut bitset = KmerBitSet::new(num_records);
                    unsafe { bitset.set_unchecked(i) };
                    bitset
                });
        });
    });

    // Finally, we don't need to store the actual asv sequences anymore.
    // We do however need to store their IDs.
    let record_ids: Vec<String> = valid_records.iter().map(|r| r.id().to_string()).collect();

    spinner.finish();

    return Ok((map, record_ids));
}
