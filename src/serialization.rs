use crate::database::KmerBitSet;
use anyhow::Result;
use bincode;
use dashmap::DashMap;
use log::info;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use zstd::stream::write::Encoder;

#[derive(Serialize, Deserialize)]
struct Database {
    hashes: HashMap<u64, KmerBitSet>,
    ids: Vec<String>,
}

impl Database {
    fn into_serializable(
        dashmap: DashMap<u64, KmerBitSet, FxBuildHasher>,
        ids: Vec<String>,
    ) -> Self {
        let hashes: HashMap<u64, KmerBitSet> = dashmap.into_iter().collect();

        return Self {
            hashes: hashes,
            ids: ids,
        };
    }
}

pub fn load_db_from_file(
    db_file: &PathBuf,
) -> (DashMap<u64, KmerBitSet, FxBuildHasher>, Vec<String>) {
    let db_file = File::open(db_file).unwrap();

    let decoder = zstd::stream::read::Decoder::new(db_file).unwrap();

    let mut reader = BufReader::new(decoder);

    let db: Database = bincode::serde::decode_from_std_read(
        &mut reader,
        bincode::config::standard().with_variable_int_encoding(),
    )
    .unwrap();

    // This is a problem for large databases, is RAM intense.
    let query_hashes: DashMap<u64, KmerBitSet, FxBuildHasher> =
        db.hashes.into_iter().collect::<DashMap<_, _, _>>();

    (query_hashes, db.ids)
}

pub fn write_db_to_file(
    dashmap: DashMap<u64, KmerBitSet, FxBuildHasher>,
    record_ids: Vec<String>,
    database: &PathBuf,
) -> Result<PathBuf> {
    info!("Writing database to file...");
    // Actual struct object.
    let db = Database::into_serializable(dashmap, record_ids);

    // Target database file.
    let db_file = PathBuf::from(database);

    let outfile = File::create(&db_file)?;

    let encoder = Encoder::new(outfile, 3)?;
    let mut db_writer = BufWriter::new(encoder);

    bincode::serde::encode_into_std_write(
        db,
        &mut db_writer,
        bincode::config::standard().with_variable_int_encoding(),
    )?;

    db_writer.flush()?;
    Ok(db_file)
}
