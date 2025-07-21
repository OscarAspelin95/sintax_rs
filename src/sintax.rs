use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader, Write};

/// We should probably convert hashset to vec earlier to speed up shuffling.
pub fn bootstrap_classify_query(
    query_hashes: &HashSet<u64>,
    query_name: &str,
    reverse_index: &HashMap<u64, HashSet<String>>,
    config: &Config,
    writer: &mut BufWriter<File>,
) {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    // Bootstrap iterations.
    for i in 0..config.num_bootstraps {
        // Randomly pick hashes (currently without replacement).
        let mut query_vec: Vec<&u64> = query_hashes.iter().collect();
        query_vec.shuffle(&mut rng);
        let random_hashes: HashSet<u64> = query_vec
            .into_iter()
            .take(config.num_query_hashes)
            .map(|x| *x)
            .collect();

        //

        // We have our random hashes, now we need to check which references
        // they match against and increment their counts.

        let mut map: HashMap<&String, usize> = HashMap::new();

        for random_hash in random_hashes {
            match reverse_index.get(&random_hash) {
                None => continue,
                Some(subject_ids) => {
                    for subject_id in subject_ids {
                        map.entry(subject_id)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        // We need to sort HashMap by values (as vec)?
        // Then take <top_candidates> best ones and print
        // For now, we take the best hit.
        let (subject_id, subject_score) = map.iter().max_by_key(|entry| entry.1).unwrap();

        let result = format!(
            "{}\t{}\t{}\t{}\n",
            query_name,
            subject_id,
            subject_score,
            i + 1
        );

        writer.write_all(result.as_bytes()).unwrap();
    }
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &HashMap<u64, HashSet<String>>,
    query_reader: Reader<BufReader<File>>,
    writer: &mut BufWriter<File>,
) {
    query_reader.records().for_each(|record| {
        if let Ok(r) = record {
            let query_hashes = kmerize(&config, &r.seq());

            bootstrap_classify_query(&query_hashes, &r.id(), &reverse_index, config, writer);
        }
    });
}
