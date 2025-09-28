use crate::database::KmerBitSet;
use crate::sintax::kmerize;
use anyhow::Result;
use bio::io::fasta::Reader;
use dashmap::DashMap;
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
    reverse_index: &DashMap<u64, KmerBitSet, FxBuildHasher>,
    valid_records: &[String],
    bootstraps: u16,
    num_query_hashes: u16,
) -> String {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    let mut iterations: Vec<String> = Vec::with_capacity(bootstraps as usize);

    // Bootstrap iterations.
    for i in 0..bootstraps {
        let mut counts: Vec<usize> = vec![0; valid_records.len()];

        for _ in 0..num_query_hashes {
            let random_index = rng.random_range(0..query_hashes.len());

            if let Some(bitset) = reverse_index.get(query_hashes[random_index]) {
                for ref_index in bitset.ones_by_iterator() {
                    counts[ref_index] += 1;
                }
            }
        }

        if let Some((index, max_count)) = counts.iter().enumerate().max_by_key(|(_, count)| *count)
        {
            let result_s = format!(
                "{}\t{}\t{}\t{}",
                query_name,
                valid_records[index],
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
    reverse_index: &DashMap<u64, KmerBitSet, FxBuildHasher>,
    valid_records: &[String],
    query_reader: Reader<BufReader<File>>,
    writer: Arc<Mutex<BufWriter<File>>>,
    bootstraps: u16,
    num_query_hashes: u16,
    kmer_size: usize,
    downsampling_factor: u64,
) -> Result<()> {
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner.set_style(ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}]",
    )?);

    query_reader.records().par_bridge().for_each(|record| {
        if let Ok(r) = record {
            let query_hashes: HashSet<u64> = kmerize(kmer_size, downsampling_factor, &r.seq());

            // This should be relatively fast if sequences are short.
            let mut query_vec: Vec<&u64> = query_hashes.iter().collect();

            let bootstrap_result = bootstrap_classify_query(
                &mut query_vec,
                &r.id(),
                &reverse_index,
                &valid_records,
                bootstraps,
                num_query_hashes,
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
