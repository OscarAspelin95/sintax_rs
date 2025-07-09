use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use dashmap::DashMap;
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

pub fn bootstrap_classify_query(
    query_hashes: &HashSet<u64>,
    query_name: &str,
    reverse_index: &DashMap<String, HashSet<u64>>,
    config: &Config,
) {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    // Bootstrap iterations.
    for i in 0..config.num_bootstraps {
        // Randomly shuffle query hashes and take num_query_hashes first hashes.
        // NOTE - here we should use sampling WITH replacement, which we currently don't.
        let mut query_vec: Vec<&u64> = query_hashes.iter().collect();
        query_vec.shuffle(&mut rng);
        let random_hashes: HashSet<u64> = query_vec
            .into_iter()
            .take(config.num_query_hashes)
            .map(|x| *x)
            .collect();

        // We have our random hashes, now we need to check which references
        // they match against and increment their counts.
        let mut count_vec: Vec<(String, usize)> = Vec::with_capacity(reverse_index.len());

        reverse_index.iter().for_each(|r| {
            let ref_name = r.key();
            let ref_hashes = r.value();

            let num_matching = ref_hashes.intersection(&random_hashes).count();

            if num_matching <= 3 {
                return;
            }

            count_vec.push((ref_name.to_owned(), num_matching));
        });

        // Sort by largest count.
        count_vec.sort_by(|a, b| Reverse(b.1).cmp(&Reverse(a.1)));

        count_vec
            .iter()
            .take(config.num_top_references)
            .for_each(|(ref_name, count)| {
                println!("{query_name}\t{ref_name}\t{i}\t{count}");
                std::io::stdout().flush().unwrap();
            });
    }
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &DashMap<String, HashSet<u64>>,
    query_reader: Reader<BufReader<File>>,
) {
    query_reader.records().par_bridge().for_each(|record| {
        let r = record.unwrap();
        let query_hashes = kmerize(&config, &r.seq());

        bootstrap_classify_query(&query_hashes, &r.id(), &reverse_index, config);
    });
}
