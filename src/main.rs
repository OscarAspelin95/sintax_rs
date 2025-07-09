mod index;
mod kmers;
mod sintax;
mod utils;

use bio::io::fasta::Reader;
use index::build_reverse_index;
use log::info;
use simple_logger::SimpleLogger;
use sintax::classify_queries;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use utils::Config;

fn mock_reference<'a>() -> (Vec<&'a str>, Vec<&'a [u8]>) {
    let mut reference_names: Vec<&str> = Vec::new();
    let mut reference_sequences: Vec<&[u8]> = Vec::new();

    let r: Vec<(&str, &[u8])> = vec![
        ("r1", b"ATCGAAAGGGTTTGGAGATAGATAGATAGAGCGACGGACTGCAGCTG"),
        ("r2", b"AAAAAGGGGGGGGGGGGGGGGGGG"),
        ("r3", b"ATCGAAAGGGTTTGGAGATAGATAGATAGAGCGACGGACTGCAGCTG"),
        ("r4", b"NNNNNNNNNNNNNNNNNNNNNNNNNN"),
        ("r5", b"NNNNNNNNNNNNNNNNNNNNNNNNNN"),
    ];

    r.iter().for_each(|(ref_name, ref_seq)| {
        reference_names.push(ref_name);
        reference_sequences.push(ref_seq);
    });

    return (reference_names, reference_sequences);
}

fn mock_queries<'a>() -> (Vec<&'a str>, Vec<&'a [u8]>) {
    let mut query_names: Vec<&str> = Vec::new();
    let mut query_sequences: Vec<&[u8]> = Vec::new();

    let q: Vec<(&str, &[u8])> = vec![(
        "q1",
        b"ATCGAAAGGGTTTGGAGATAGATAGATAGAGCGACGGACTGCAGCTGCAGCTGCAGCT",
    )];

    q.iter().for_each(|(query_name, query_seq)| {
        query_names.push(query_name);
        query_sequences.push(query_seq);
    });

    return (query_names, query_sequences);
}

// fn get_real_query() -> Reader<BufReader<File>> {
//     let query_fasta = PathBuf::from("/home/oscar/github/sintax_rs/query.fasta");
//     assert!(query_fasta.is_file());
//     let reader = Reader::from_file(query_fasta).unwrap();

//     return reader;
// }

// fn get_real_reference() -> Reader<BufReader<File>> {
//     let reference_fasta = PathBuf::from("/home/oscar/github/sintax_rs/silva.fasta");
//     assert!(reference_fasta.is_file());
//     let reader = Reader::from_file(reference_fasta).unwrap();

//     return reader;
// }
fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Building reverse index...");
    let config = Config::default();

    // Mock references for now.
    let (reference_names, reference_seqs): (Vec<&str>, Vec<&[u8]>) = mock_reference();

    // Build reverse index for entire database.
    let reverse_index = build_reverse_index(&reference_seqs, &config);

    // Mock queries for now.
    let (query_names, query_seqs) = mock_queries();

    //
    info!("Classifying queries...");
    let _ = classify_queries(
        &config,
        &reverse_index,
        &query_names,
        &query_seqs,
        &reference_names,
    );
}
