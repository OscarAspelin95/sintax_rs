mod args;
mod classifier;
mod errors;
mod sintax;
mod utils;

use crate::classifier::sintax_classify;
use args::Args;
use clap::Parser;

use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;
use std::process;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    if let Err(e) = sintax_classify(args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
