# sintax_rs
Rust implementation of the SINTAX classifier using SIMD-accelerated canonical minimizers.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation

### Pre-built Binaries
Pre-built linux binaries are available from the [GitHub releases page](https://github.com/OscarAspelin95/sintax_rs/releases). Note that these binaries are **not optimized** for a specific CPU architecture.

### Building from Source (Recommended)
For best performance, compile from source with CPU-specific optimizations. Clone the repository or download the source code, enter the sintax_rs directory, and run:<br>
`RUSTFLAGS="-C target-cpu=native" cargo build --release`

The generated binary is available in `target/release/sintax_rs`.

## Database
If using `sintax_rs` standalone, a database fasta with a certain header structure is required. The currently easiest way is to use the `database.py` script from the [amplipore](https://github.com/OscarAspelin95/amplipore/) repository to generate a `sintax_rs` friendly database. Or simply, use `amplipore` itself if the goal is to run a complete classification algorithm.

## Usage
Run with:<br>
`sintax_rs --query <query.fasta> --database <database.fasta> --outfile <out.tsv>`

Accepted query/database file extensions: `.fasta`, `.fa`, `.fsa`, `.fna`.

Optional arguments:
<pre>
<b>--bootstraps</b> [100] - Number of bootstrap iterations per query sequence.
<b>--query-hashes</b> [32] - Number of kmer hashes to randomly pick per query sequence.
<b>--kmer-size</b> [15] - Kmer size for canonical minimizers.
<b>--window-size</b> [7] - Minimizer window size. Larger values = fewer minimizers => faster but less sensitive.
<b>--threads</b> [0] - Num threads to use. By default, lets Rayon choose.
</pre>

**Note:** `kmer_size + window_size - 1` must be odd (required for canonical minimizers). The defaults (15 + 7 - 1 = 21) satisfy this constraint.

## Post Processing
Either implement your own custom parser, or use [amplipore](https://github.com/OscarAspelin95/amplipore/).

## Memory usage
`sintax_rs` is not designed for low memory use, but reducing the number of threads can help at the expense of longer runtimes. Similarly, increasing the minimizer window size will reduce the number of generated kmers at the expense of decreased accuracy.
