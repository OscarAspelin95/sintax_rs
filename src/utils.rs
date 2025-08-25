use crate::args::Args;
use crate::errors::AppError;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Config {
    pub kmer_size: usize,
    pub num_bootstraps: usize,
    pub num_query_hashes: usize,
    pub ds_factor: u64,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        return Self {
            kmer_size: args.kmer_size as usize,
            num_bootstraps: args.bootstraps as usize,
            num_query_hashes: args.query_hashes as usize,
            ds_factor: args.downsampling_factor as u64,
        };
    }
}

pub fn fasta_reader(f: &PathBuf) -> Result<Reader<BufReader<File>>, AppError> {
    if !f.is_file() {
        return Err(AppError::FastaPathError);
    }

    let reader = Reader::from_file(f).map_err(|_| AppError::FastaReadError)?;
    return Ok(reader);
}
