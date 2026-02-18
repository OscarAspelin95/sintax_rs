use crate::utils::Config;
use packed_seq::PackedNSeqVec;
use rstest::rstest;
use rustc_hash::FxBuildHasher;
use std::collections::HashSet;

pub fn kmerize(config: &Config, nt_string: &[u8]) -> HashSet<u64, FxBuildHasher> {
    let min_len = config.kmer_size + config.window_size - 1;
    if nt_string.len() < min_len {
        return HashSet::with_hasher(FxBuildHasher);
    }
    let packed = PackedNSeqVec::from_ascii(nt_string);
    let mut positions = vec![];
    let output = simd_minimizers::canonical_minimizers(config.kmer_size, config.window_size)
        .run_skip_ambiguous_windows(packed.as_slice(), &mut positions);
    output.values_u64().collect()
}

#[rstest]
#[case(b"AAAAAAAAAA", 3, 7)]
#[case(b"ATCGATCGATCG", 5, 7)]
fn test_kmerize_nonempty(#[case] seq: &[u8], #[case] kmer_size: usize, #[case] window_size: usize) {
    let config = Config {
        kmer_size,
        num_bootstraps: 1,
        num_query_hashes: 1,
        window_size,
    };

    let result = kmerize(&config, seq);
    assert!(!result.is_empty());
}

#[rstest]
#[case(b"NNNNNNNNNNNNN", 5, 7)]
fn test_kmerize_all_n(#[case] seq: &[u8], #[case] kmer_size: usize, #[case] window_size: usize) {
    let config = Config {
        kmer_size,
        num_bootstraps: 1,
        num_query_hashes: 1,
        window_size,
    };

    let result = kmerize(&config, seq);
    assert!(result.is_empty());
}

#[rstest]
#[case(b"AAAAAAAAAAAA", b"TTTTTTTTTTTT", 3, 7)]
#[case(b"AAAAAAAAAAT", b"ATTTTTTTTTTT", 3, 7)]
fn test_kmerize_canonical(
    #[case] seq1: &[u8],
    #[case] seq2: &[u8],
    #[case] kmer_size: usize,
    #[case] window_size: usize,
) {
    let config = Config {
        kmer_size,
        num_bootstraps: 1,
        num_query_hashes: 1,
        window_size,
    };

    let result1 = kmerize(&config, seq1);
    let result2 = kmerize(&config, seq2);

    assert_eq!(result1, result2);
}

#[test]
fn test_kmerize_too_short() {
    let config = Config {
        kmer_size: 11,
        num_bootstraps: 1,
        num_query_hashes: 1,
        window_size: 7,
    };

    let result = kmerize(&config, b"ACGTACG");
    assert!(result.is_empty());
}
