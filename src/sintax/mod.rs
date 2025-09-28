pub mod kmers;
pub use kmers::kmerize;

pub mod index;
pub use index::build_reverse_index;

pub mod sintax;
pub use sintax::classify_queries;

pub mod kmer_bitset;
pub use kmer_bitset::KmerBitSet;
