use crate::args::Args;
use crate::errors::AppError;
use crate::sintax::{build_reverse_index, classify_queries};
use crate::utils::{Config, parse_fasta};

use log::info;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

pub fn sintax_classify(args: Args) -> Result<(), AppError> {
    // Canonical minimizers require k + w - 1 to be odd.
    let kw = args.kmer_size as usize + args.window_size as usize - 1;
    if kw.is_multiple_of(2) {
        return Err(AppError::InvalidParameter(format!(
            "kmer_size + window_size - 1 must be odd for canonical minimizers (got {}). \
             Try adjusting --kmer-size or --window-size by 1.",
            kw
        )));
    }

    // Read reference fasta.
    info!("Loading database sequences...");
    let db_records = parse_fasta(&args.database)?;

    // Read query fasta.
    info!("Loading query sequences...");
    let query_records = parse_fasta(&args.query)?;

    // For writing results to file.
    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(&args.outfile)?)));

    let config = Config::from(args);

    // Build reverse index for entire database.
    info!("Building reverse index...");
    let reverse_index = build_reverse_index(&db_records, &config);

    info!("Classifying queries...");
    classify_queries(
        &config,
        &reverse_index,
        &db_records,
        &query_records,
        writer,
    )?;

    Ok(())
}
