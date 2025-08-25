use clap::Parser;
use clap::value_parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, help = "Path to asv fasta file.")]
    pub query: PathBuf,

    #[arg(short, long, help = "Path to database fasta file.")]
    pub database: PathBuf,

    #[arg(long, default_value_t = 100, value_parser = value_parser!(u16).range(10..200))]
    pub bootstraps: u16,

    #[arg(long, default_value_t = 32, value_parser = value_parser!(u16).range(10..100))]
    pub query_hashes: u16,

    #[arg(long, default_value_t = 15, value_parser = value_parser!(u16).range(7..31))]
    pub kmer_size: u16,

    #[arg(long, default_value_t = 1, value_parser = value_parser!(u64).range(1..100))]
    pub downsampling_factor: u64,

    #[arg(short, long)]
    pub outfile: PathBuf,

    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,
}
