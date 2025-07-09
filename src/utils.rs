pub struct Config {
    pub kmer_size: usize,
    pub num_bootstraps: usize,
    pub num_query_hashes: usize,
    pub num_top_references: usize,
    pub ds_factor: u64,
}

impl Config {
    pub fn default() -> Self {
        return Self {
            kmer_size: 5,
            num_bootstraps: 5,
            num_query_hashes: 10,
            num_top_references: 2,
            ds_factor: 1,
        };
    }
}
