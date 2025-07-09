use crate::utils::Config;
use fixedbitset::FixedBitSet;
use rand::prelude::*;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

use crate::kmers::kmerize;

pub fn bootstrap_classify_query(
    query_hashes: &HashSet<u64>,
    query_name: &str,
    reverse_index: &HashMap<u64, FixedBitSet>,
    reference_names: &Vec<&str>,
    config: &Config,
) {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    // Bootstrap iterations. We
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
        let mut count_map: HashMap<usize, usize> = HashMap::new();

        random_hashes
            .iter()
            // For each random hash we selected, we check if present in reverse index.
            .for_each(|random_hash| match reverse_index.get(random_hash) {
                // If it is, we get the corresponding bitset.
                Some(bitset) => {
                    // For each bit set to one (corresponding to a reference)
                    bitset.ones().for_each(|index| {
                        // Increment the count hashmap by one.
                        if !count_map.contains_key(&index) {
                            count_map.insert(index, 1);
                        }
                        let count = count_map.get_mut(&index).unwrap();
                        *count += 1;
                    });
                }
                None => {}
            });

        // Sort by largest count.
        let mut s = count_map.iter().collect::<Vec<_>>();
        s.sort_by(|a, b| Reverse(b.1).cmp(&Reverse(a.1)));

        s.iter()
            .take(config.num_top_references)
            .for_each(|(index, count)| {
                let ref_name = reference_names[**index];
                println!("{query_name}\t{ref_name}\t{i}\t{count}");
                std::io::stdout().flush().unwrap();
            });
    }
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &HashMap<u64, FixedBitSet>,
    query_names: &Vec<&str>,
    query_seqs: &Vec<&[u8]>,
    reference_names: &Vec<&str>,
) {
    query_names
        .iter()
        .zip(query_seqs)
        .for_each(|(query_name, query_seq)| {
            let query_hashes = kmerize(&config, &query_seq);

            bootstrap_classify_query(
                &query_hashes,
                &query_name,
                &reverse_index,
                &reference_names,
                config,
            );
        });
}
