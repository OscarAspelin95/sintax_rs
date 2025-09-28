mod args;
mod classifier;
mod database;
mod errors;
mod serialization;
mod sintax;
mod utils;

use crate::classifier::sintax_classify;
use crate::database::build_database;

use args::Args;
use args::SubCommand::{Classify, Database};
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    match args.command {
        Database {
            fasta,
            outfile,
            kmer_size,
            downsampling_factor,
        } => build_database(&fasta, &outfile, kmer_size, downsampling_factor).unwrap(),
        Classify {
            fasta,
            database,
            bootstraps,
            query_hashes,
            kmer_size,
            downsampling_factor,
            outfile,
            threads,
        } => sintax_classify(
            &fasta,
            &database,
            bootstraps,
            query_hashes,
            kmer_size,
            downsampling_factor,
            &outfile,
            threads,
        )
        .unwrap(),
    }
}
