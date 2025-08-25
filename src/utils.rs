use crate::errors::AppError;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Config {
    pub kmer_size: usize,
    pub num_bootstraps: usize,
    pub num_query_hashes: usize,
    pub num_top_references: usize,
    pub ds_factor: u64,
}

impl Config {
    pub fn default() -> Self {
        return Self {
            kmer_size: 15,
            num_bootstraps: 100,
            num_query_hashes: 32,
            num_top_references: 1,
            ds_factor: 1,
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
