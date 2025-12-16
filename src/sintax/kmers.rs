use crate::utils::Config;
use rstest::rstest;
use std::collections::HashSet;

pub const LOOKUP: [u8; 256] = [
    0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

#[inline]
pub fn mm_hash64(kmer: u64) -> u64 {
    let mut key = kmer;
    key = !key.wrapping_add(key << 21);
    key = key ^ key >> 24;
    key = (key.wrapping_add(key << 3)).wrapping_add(key << 8);
    key = key ^ key >> 14;
    key = (key.wrapping_add(key << 2)).wrapping_add(key << 4);
    key = key ^ key >> 28;
    key = key.wrapping_add(key << 31);
    key
}

pub fn kmerize(config: &Config, nt_string: &[u8]) -> HashSet<u64> {
    assert!(config.kmer_size <= nt_string.len());

    // Forward related kmer stuff
    let mut kmer_forward: u64 = 0;

    let nbits = config.kmer_size << 1;
    let mask: u64 = (1 << nbits) - 1;

    // Reverse related kmer stuff.
    let mut kmer_reverse: u64 = 0;
    let shift = ((config.kmer_size - 1) * 2) as u64;

    // Storage.
    let mut canonical_hashes: HashSet<u64> =
        HashSet::with_capacity(nt_string.len() - config.kmer_size + 1);

    let mut valid_kmer_index: usize = 0;

    nt_string.iter().for_each(|nt_char| {
        // Forward kmer.
        let nt = LOOKUP[*nt_char as usize] as u64;

        if nt >= 4 {
            valid_kmer_index = 0;
            kmer_forward = 0;
            kmer_reverse = 0;
            return;
        }
        kmer_forward = (kmer_forward << 2 | nt) & mask;

        // Reverse kmer.
        let nt_rev = 3 - nt;
        kmer_reverse = kmer_reverse >> 2 | nt_rev << shift;

        if valid_kmer_index >= config.kmer_size - 1 {
            let canonical = match kmer_forward < kmer_reverse {
                true => kmer_forward,
                false => kmer_reverse,
            };
            // MinFracHash
            if canonical <= u64::MAX / config.ds_factor {
                canonical_hashes.insert(mm_hash64(canonical));
            }
        }

        valid_kmer_index += 1;
    });

    canonical_hashes
}

#[rstest]
#[case(b"AAAAAAAA", 3, 1)]
#[case(b"AAAAAAAC", 3, 2)]
#[case(b"ATCGATCGATCG", 4, 3)]
#[case(b"ATCNATCNATCN", 4, 0)]
fn test_kmerize(#[case] seq: &[u8], #[case] kmer_size: usize, #[case] expected_num_hashes: usize) {
    let config = Config {
        kmer_size: kmer_size,
        num_bootstraps: 1,
        num_query_hashes: 1,
        ds_factor: 1,
    };

    let result = kmerize(&config, seq);
    assert_eq!(result.len(), expected_num_hashes);
}

#[rstest]
#[case(b"AAAAAAAA", b"TTTTTTTT", 3)]
#[case(b"AAAAAAAAAT", b"ATTTTTTTTT", 3)]
fn test_kmerize_reverse(#[case] seq1: &[u8], #[case] seq2: &[u8], #[case] kmer_size: usize) {
    let config = Config {
        kmer_size: kmer_size,
        num_bootstraps: 1,
        num_query_hashes: 1,
        ds_factor: 1,
    };

    let result1 = kmerize(&config, seq1);
    let result2 = kmerize(&config, seq2);

    assert_eq!(result1, result2);
}
