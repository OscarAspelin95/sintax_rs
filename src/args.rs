use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, help = "Path to asv fasta file.")]
    pub query: PathBuf,

    #[arg(short, long, help = "Path to database fasta file.")]
    pub subject: PathBuf,

    #[arg(short, long)]
    pub outfile: PathBuf,

    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,
}
