# sintax_rs
Rust implementation of the SINTAX classifier.

## ToDo
- [ ] Replace python script with Rust.
- [ ] Add single thread version of reverse index build.
- [ ] Allow reusing already build reverse indexes.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the sintax_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/sintax_rs`.

## Usage
Run with:<br>
`sintax_rs --query <query.fasta> --database <database.fasta> --outfile <out.tsv>`

Optional arguments:
<pre>
<b>--bootstraps</b> [100] - Number of bootstrap iterations per query sequence.
<b>--query_hashes</b> [32] - Number of kmer hashes to randomly pick per query sequence.
<b>--kmer_size</b> [15] - FracMinHash kmer size.
<b>--downsampling_factor</b> [1] - FracMinHash downsampling factor.
<b>--threads</b> [0] - Num threads to use. By default, lets Rayons choose.
</pre>

Parse the results with the <em>parse_sintax_tsv.py</em> script. NOTE - the script assumes that the db fasta ids have the following structure:<br>
`><accession>;tax=k:<kingdom>,p:<phylum>,c:<class>,o:<order>,f:<family>,g:<genus>,s:<species>_<accession>;`