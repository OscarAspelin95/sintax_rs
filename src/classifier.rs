use crate::serialization::load_db_from_file;
use crate::sintax::classify_queries;
use crate::utils::fasta_reader;

use anyhow::Result;
use bio::io::fasta::Reader;
use log::info;
use rayon::ThreadPoolBuilder;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn sintax_classify(
    fasta: &PathBuf,
    database: &PathBuf,
    bootstraps: u16,
    num_query_hashes: u16,
    kmer_size: u16,
    downsampling_factor: u64,
    outfile: &PathBuf,
    threads: usize,
) -> Result<()> {
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;

    // Read query fasta.
    let query_reader: Reader<BufReader<File>> = fasta_reader(fasta)?;

    // For writing results to file.
    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(outfile)?)));

    info!("Loading database...");
    let (reverse_index, record_ids) = load_db_from_file(database);

    info!("Classifying queries...");
    classify_queries(
        &reverse_index,
        record_ids.as_slice(),
        query_reader,
        writer,
        bootstraps,
        num_query_hashes,
        kmer_size as usize,
        downsampling_factor,
    )?;

    Ok(())
}
