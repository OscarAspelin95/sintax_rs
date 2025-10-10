# sintax_rs
Rust implementation of the SINTAX classifier.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the sintax_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/sintax_rs`.

## Database
If using `sintax_rs` standalone, a database fasta with a certain header structure is required. The currently easiest way is to use the `database.py` script from the [amplipore](https://github.com/OscarAspelin95/amplipore/) repository to generate a `sintax_rs` friendly database.

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

## Post Processing
The raw `sintax_rs` results need to be parsed in order to generate classification confidence scores. The easiest way is to use `parse_sintax_tsv.py`, available in the [repository](https://github.com/OscarAspelin95/sintax_rs).

## Memory usage
`sintax_rs` is not designed for low memory use, but reducing the number of threads can help at the expense of longer runtimes. Similarly, increasing the FracMinHash downsampling factor will reduce the number of generated kmers at the expense of decreased accuracy.