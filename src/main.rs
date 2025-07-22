mod index;
mod kmers;
mod sintax;
mod utils;

use bio::io::fasta::Reader;
use clap::Parser;
use index::build_reverse_index;
use log::info;
use simple_logger::SimpleLogger;
use sintax::classify_queries;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use utils::Config;

fn fasta_reader(f: &PathBuf) -> Reader<BufReader<File>> {
    assert!(f.is_file());

    let reader = Reader::from_file(f).unwrap();
    return reader;
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    query: PathBuf,

    #[arg(short, long)]
    subject: PathBuf,

    #[arg(short, long)]
    outfile: PathBuf,
}

/// Building the reverse index still takes too long. An
/// alternative is to use needletail, might be faster.
/// * Read reference once, store in HashMap with start_line_number/position as key.
/// * Kmerize sequences, store hash as key, HashSet<&[u8]> as value (in parallel)?
fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    // Setup some config stuff.
    let config = Config::default();

    // Read reference fasta.
    let reference_reader: Reader<BufReader<File>> = fasta_reader(&args.subject);

    // Build reverse index for entire database.
    info!("Building reverse index...");
    let reverse_index = build_reverse_index(reference_reader, &config);

    // Read query fasta.
    let query_reader: Reader<BufReader<File>> = fasta_reader(&args.query);

    // For writing results to file.
    let mut writer = BufWriter::new(File::create(&args.outfile).unwrap());

    info!("Classifying queries...");
    classify_queries(&config, &reverse_index, query_reader, &mut writer);
}
