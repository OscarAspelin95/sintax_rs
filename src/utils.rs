use crate::errors::AppError;
use bio::io::fasta::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn fasta_reader(f: &PathBuf) -> Result<Reader<BufReader<File>>, AppError> {
    if !f.is_file() {
        return Err(AppError::FastaPathError);
    }

    let reader = Reader::from_file(f).map_err(|_| AppError::FastaReadError)?;
    return Ok(reader);
}
