use crate::args::Args;
use crate::errors::AppError;
use needletail::parse_fastx_file;
use std::path::Path;

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

pub struct FastaRecord {
    pub id: String,
    pub seq: Vec<u8>,
}

fn validate_fasta_extension(f: &Path) -> Result<(), AppError> {
    let ext = f.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !VALID_FASTA_EXTENSIONS.contains(&ext) {
        return Err(AppError::InvalidFileExtension(f.display().to_string()));
    }
    Ok(())
}

pub fn parse_fasta(f: &Path) -> Result<Vec<FastaRecord>, AppError> {
    if !f.is_file() {
        return Err(AppError::FileNotFound(f.display().to_string()));
    }
    validate_fasta_extension(f)?;

    let mut reader = parse_fastx_file(f)
        .map_err(|e| AppError::FastaReadError(format!("{}: {}", f.display(), e)))?;

    let mut records = Vec::new();
    while let Some(result) = reader.next() {
        let rec = result.map_err(|e| AppError::FastaReadError(format!("{}: {}", f.display(), e)))?;
        let id = std::str::from_utf8(rec.id())
            .map_err(|e| AppError::FastaReadError(format!("{}: {}", f.display(), e)))?
            .to_string();
        records.push(FastaRecord {
            id,
            seq: rec.seq().to_vec(),
        });
    }

    Ok(records)
}
