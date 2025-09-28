use clap::Parser;
use clap::value_parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about = "Amplicon classification using the SINTAX algorithm.")]
pub struct Args {
    #[clap(subcommand)]
    pub command: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Database {
        #[arg(short, long, help = "Path to database fasta file.")]
        fasta: PathBuf,

        #[arg(
            short,
            long,
            help = "Database output file.",
            default_value = "database.srs",
            required = false
        )]
        outfile: PathBuf,

        #[arg(long, default_value_t = 15, value_parser = value_parser!(u16).range(7..31))]
        kmer_size: u16,

        #[arg(long, default_value_t = 1, value_parser = value_parser!(u64).range(1..100))]
        downsampling_factor: u64,
    },
    Classify {
        #[arg(short, long, help = "Path to asv fasta file.")]
        fasta: PathBuf,

        #[arg(short, long, help = "Path to database file (.srs).")]
        database: PathBuf,

        #[arg(long, default_value_t = 100, value_parser = value_parser!(u16).range(10..200))]
        bootstraps: u16,

        #[arg(long, default_value_t = 32, value_parser = value_parser!(u16).range(10..100))]
        query_hashes: u16,

        #[arg(long, default_value_t = 15, value_parser = value_parser!(u16).range(7..31))]
        kmer_size: u16,

        #[arg(long, default_value_t = 1, value_parser = value_parser!(u64).range(1..100))]
        downsampling_factor: u64,

        #[arg(short, long)]
        outfile: PathBuf,

        #[arg(short, long, default_value_t = 0)]
        threads: usize,
    },
}
