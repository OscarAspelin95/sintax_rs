use crate::errors::AppError;
use crate::sintax::{build_reverse_index, classify_queries};
use crate::utils::{Config, fasta_reader};

use bio::io::fasta::Reader;
use log::info;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn sintax_classify(
    subject: &PathBuf,
    query: &PathBuf,
    config: &Config,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    // Read reference fasta.
    let reference_reader: Reader<BufReader<File>> = fasta_reader(&subject)?;

    // Build reverse index for entire database.
    info!("Building reverse index...");
    let (reverse_index, valid_records) = build_reverse_index(reference_reader, &config);

    // Read query fasta.
    let query_reader: Reader<BufReader<File>> = fasta_reader(&query)?;

    // For writing results to file.
    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(&outfile).unwrap())));

    info!("Classifying queries...");
    classify_queries(
        &config,
        &reverse_index,
        valid_records.as_slice(),
        query_reader,
        writer,
    );

    Ok(())
}
