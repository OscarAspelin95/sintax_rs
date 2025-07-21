use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufReader, Write};
use std::rc::Rc;

/// We should probably convert hashset to vec earlier to speed up shuffling.
pub fn bootstrap_classify_query(
    query_hashes: &mut Vec<&u64>,
    query_name: &str,
    reverse_index: &HashMap<u64, HashSet<Rc<str>>>,
    config: &Config,
    writer: &mut BufWriter<File>,
) {
    // For randomizing hashes.
    let mut rng: ThreadRng = rand::rng();

    // Bootstrap iterations.
    for i in 0..config.num_bootstraps {
        // Randomly pick hashes (currently without replacement).
        query_hashes.shuffle(&mut rng);
        let random_hashes: &[&u64] =
            &query_hashes[..config.num_query_hashes.min(query_hashes.len())];

        // We have our random hashes, now we need to check which references
        // they match against and increment their counts.

        let mut map: HashMap<&str, usize> = HashMap::new();

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

        writeln!(
            writer,
            "{}\t{}\t{}\t{}\n",
            query_name,
            subject_id,
            subject_score,
            i + 1
        )
        .unwrap()
    }
}

pub fn classify_queries(
    config: &Config,
    reverse_index: &HashMap<u64, HashSet<Rc<str>>>,
    query_reader: Reader<BufReader<File>>,
    writer: &mut BufWriter<File>,
) {
    query_reader.records().for_each(|record| {
        if let Ok(r) = record {
            let query_hashes: HashSet<u64> = kmerize(&config, &r.seq());

            let mut query_vec: Vec<&u64> = query_hashes.iter().collect();

            bootstrap_classify_query(&mut query_vec, &r.id(), &reverse_index, config, writer);
        }
    });
}
