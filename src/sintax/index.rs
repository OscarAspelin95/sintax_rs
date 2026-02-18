use crate::sintax::kmerize;
use crate::utils::{Config, FastaRecord};

use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use log::*;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::time::Duration;

pub fn build_reverse_index(
    valid_records: &[FastaRecord],
    config: &Config,
) -> HashMap<u64, FixedBitSet, FxBuildHasher> {
    let num_records = valid_records.len();

    // Build in parallel using DashMap for concurrent writes.
    let dash_map: DashMap<u64, FixedBitSet, FxBuildHasher> =
        DashMap::with_capacity_and_hasher(num_records, FxBuildHasher);

    info!("Creating index...");
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}]").unwrap());

    valid_records.par_iter().enumerate().for_each(|(i, r)| {
        let hashes = kmerize(config, &r.seq);

        hashes.iter().for_each(|h| {
            dash_map
                .entry(*h)
                .and_modify(|bitset: &mut FixedBitSet| bitset.set(i, true))
                .or_insert_with(|| {
                    let mut bitset = FixedBitSet::with_capacity(num_records);
                    bitset.set(i, true);
                    bitset
                });
        });
    });

    spinner.finish();

    // Convert to lock-free hashbrown::HashMap for the read-only query phase.
    let mut map: HashMap<u64, FixedBitSet, FxBuildHasher> =
        HashMap::with_capacity_and_hasher(dash_map.len(), FxBuildHasher);
    map.extend(dash_map.into_iter());
    map
}
