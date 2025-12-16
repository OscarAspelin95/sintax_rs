use crate::args::Args;
use crate::sintax::{build_reverse_index, classify_queries};
use crate::utils::{Config, fasta_reader};

use bio::io::fasta::Reader;
use log::info;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};

use anyhow::Result;

pub fn sintax_classify(args: Args) -> Result<()> {
    let database_reader: Reader<BufReader<File>> = fasta_reader(&args.database)?;
    let query_reader: Reader<BufReader<File>> = fasta_reader(&args.query)?;

    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(&args.outfile)?)));
    let config = Config::from(args);

    info!("Building reverse index...");
    let (reverse_index, valid_records) = build_reverse_index(database_reader, &config);

    info!("Classifying queries...");
    classify_queries(
        &config,
        &reverse_index,
        valid_records.as_slice(),
        query_reader,
        writer,
    )?;

    Ok(())
}
