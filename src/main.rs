mod args;
mod classifier;
mod errors;
mod sintax;
mod utils;

use crate::classifier::sintax_classify;
use args::Args;
use clap::Parser;

use log::error;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");

    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .expect("Failed to build rayon thread pool");

    let result = sintax_classify(args);

    if result.is_err() {
        error!("Error: {:?}", result.err());
        std::process::exit(1);
    }
}
