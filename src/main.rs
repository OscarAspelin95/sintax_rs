mod args;
mod classifier;
mod errors;
mod sintax;
mod utils;

use crate::classifier::sintax_classify;
use args::Args;
use clap::Parser;
use utils::Config;

use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    // Setup some config stuff.
    let config = Config::default();

    sintax_classify(&args.subject, &args.query, &config, &args.outfile).unwrap();
}
