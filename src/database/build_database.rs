use crate::{database::build_reverse_index, serialization::write_db_to_file};
use anyhow::Result;
use log::info;
use std::path::PathBuf;

pub fn build_database(
    fasta: &PathBuf,
    database: &PathBuf,
    kmer_size: u16,
    downsampling_factor: u64,
) -> Result<()> {
    // Here, we need to enable caching.
    // Build reverse index for entire database
    info!("Building reverse index...");
    let (reverse_index, record_ids) =
        build_reverse_index(fasta, kmer_size as usize, downsampling_factor)?;

    info!("Writing to database...");
    write_db_to_file(reverse_index, record_ids.clone(), database)?;

    Ok(())
}
