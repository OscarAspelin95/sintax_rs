# sintax_rs
🚧 Work in progress for implementing the SINTAX classifier in Rust.

At the moment, the following parameters are hard-coded in *utils.rs* but will become arguments in the future:<br>
<pre>
<b>--num_bootstraps</b> [100] - The number of bootstrap iterations.

<b>--num_query_hashes</b> [32] - The number of kmer hashes to pick during each bootstrap iteration.

<b>--kmer_size</b> [15] - Kmer size to use.

<b>--ds_factor</b> [1] - MinFracHash downsampling factor.
</pre>

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the sintax_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/sintax_rs`.

## Usage
Run with:<br>
`sintax_rs --query <query.fasta> --subject <subject.fasta> --outfile <out.tsv>`

Optional arguments:
<pre>
<b>--threads</b> [8] - Num threads to use.
</pre>


Parse the results with the <em>parse_sintax_tsv.py</em> script. NOTE - the script assumes that the db fasta ids have the following structure:<br>
`><accession>;tax=k:<kingdom>,p:<phylum>,c:<class>,o:<order>,f:<family>,g:<genus>,s:<species>_<accession>;`
