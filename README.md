# bibget

[![CI](https://github.com/nathansam/bibget/actions/workflows/rust.yml/badge.svg)](https://github.com/nathansam/bibget/actions/workflows/rust.yml) [![crates.io](https://img.shields.io/crates/v/bibget.svg)](https://crates.io/crates/bibget) [![dependency status](https://deps.rs/repo/github/nathansam/bibget/status.svg)](https://deps.rs/repo/github/nathansam/bibget) [![MIT](https://img.shields.io/crates/l/bibget.svg)](https://github.com/nathansam/bibget/blob/master/LICENSE)

CLI tool to generate a BibTex entry from a DOI. Written in Rust and based on the [`doi2bib` crate](https://crates.io/crates/doi2bib). 

## Installation

`bibget` can be installed from source using `cargo`




``` bash
# release version from crates.io
cargo install bibget 

# development version from GitHub
cargo install --git https://github.com/nathansam/bibget.git bibget 
```

## Usage

`bibget` supports an optional `-f/--file` argument for writing the BibTex entry to a file. The file will automatically be created if it does not already exist. If the file exists then the entry will be appended to the file. 

``` bash
> bibget -f test.bib 10.1002/sim.1186
```

```
@article{Higgins_2002,
	doi = {10.1002/sim.1186},
	url = {https://doi.org/10.1002%2Fsim.1186},
	year = 2002,
	publisher = {Wiley},
	volume = {21},
	number = {11},
	pages = {1539--1558},
	author = {Julian P. T. Higgins and Simon G. Thompson},
	title = {Quantifying heterogeneity in a meta-analysis},
	journal = {Statistics in Medicine}
}
```
