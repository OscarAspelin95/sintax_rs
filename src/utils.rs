use crate::args::Args;
use crate::errors::AppError;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const VALID_FASTA_EXTENSIONS: &[&str] = &["fasta", "fa", "fsa", "fna"];

pub struct Config {
    pub kmer_size: usize,
    pub num_bootstraps: usize,
    pub num_query_hashes: usize,
    pub window_size: usize,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            kmer_size: args.kmer_size as usize,
            num_bootstraps: args.bootstraps as usize,
            num_query_hashes: args.query_hashes as usize,
            window_size: args.window_size as usize,
        }
    }
}

fn validate_fasta_extension(f: &PathBuf) -> Result<(), AppError> {
    let ext = f
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    if !VALID_FASTA_EXTENSIONS.contains(&ext) {
        return Err(AppError::InvalidFileExtension(f.display().to_string()));
    }
    Ok(())
}

pub fn fasta_reader(f: &PathBuf) -> Result<Reader<BufReader<File>>, AppError> {
    if !f.is_file() {
        return Err(AppError::FileNotFound(f.display().to_string()));
    }
    validate_fasta_extension(f)?;
    let reader =
        Reader::from_file(f).map_err(|e| AppError::FastaReadError(format!("{}: {}", f.display(), e)))?;
    Ok(reader)
}
