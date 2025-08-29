use crate::sintax::kmerize;
use crate::utils::Config;
use anyhow::Result;
use bio::io::fasta::Reader;
use bio::io::fasta::Record;
use dashmap::DashMap;
use fixedbitset::FixedBitSet;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::prelude::*;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn bootstrap_classify_query(
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
        let mut counts: Vec<usize> = vec![0; valid_records.len()];

        for _ in 0..config.num_query_hashes {
            let random_index = rng.random_range(0..query_hashes.len());

            if let Some(bitset) = reverse_index.get(query_hashes[random_index]) {
                for ref_index in bitset.ones() {
                    counts[ref_index] += 1;
                }
            }
        }

        if let Some((index, max_count)) = counts.iter().enumerate().max_by_key(|(_, count)| *count)
        {
            let result_s = format!(
                "{}\t{}\t{}\t{}",
                query_name,
                valid_records[index].id(),
                max_count,
                i + 1
            );
            iterations.push(result_s);
        }
    }
    let bootstrap_result = iterations.join("\n");
    return bootstrap_result;
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &DashMap<u64, FixedBitSet, FxBuildHasher>,
    valid_records: &[Record],
    query_reader: Reader<BufReader<File>>,
    writer: Arc<Mutex<BufWriter<File>>>,
) -> Result<()> {
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}]",
    )?);

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

            let mut w = writer.lock().expect("Mutex lock fail.");
            writeln!(w, "{}", bootstrap_result).unwrap()
        }
    });

    let mut writer = Arc::into_inner(writer).unwrap().into_inner()?;
    writer.flush()?;

    spinner.finish();

    Ok(())
}
