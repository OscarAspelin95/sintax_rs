use crate::kmers::kmerize;
use crate::utils::Config;
use bio::io::fasta::Reader;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

pub fn build_reverse_index(
    reference_reader: Reader<BufReader<File>>,
    config: &Config,
) -> HashMap<u64, HashSet<Rc<str>>> {
    let mut map: HashMap<u64, HashSet<Rc<str>>> = HashMap::new();

    reference_reader.records().for_each(|record| {
        if let Ok(r) = record {
            let id_rc: Rc<str> = Rc::from(r.id());
            let hashes = kmerize(config, &r.seq());

            for hash in hashes {
                map.entry(hash)
                    .or_insert_with(|| HashSet::new())
                    .insert(id_rc.clone());
            }
        }
    });

    return map;
}
