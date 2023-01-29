# bibget

[![CI](https://github.com/nathansam/bibget/actions/workflows/ci.yml/badge.svg)](https://github.com/nathansam/bibget/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/bibget.svg)](https://crates.io/crates/bibget) [![Documentation](https://docs.rs/bibget/badge.svg)](https://docs.rs/bibget) [![dependency status](https://deps.rs/repo/github/nathansam/bibget/status.svg)](https://deps.rs/repo/github/nathansam/bibget) [![MIT](https://img.shields.io/crates/l/bibget.svg)](https://github.com/nathansam/bibget/blob/master/LICENSE)

CLI tool to generate a BibTex entry from a DOI. Written in Rust and based on the [`doi2bib` crate](https://crates.io/crates/doi2bib). 

## Installation

`bibget` can be installed from source using `cargo`

``` bash
cargo install --git https://github.com/nathansam/bibget.git bibget 
```

## Usage

`bibget` supports an optional `-f/--file` argument for writing the BibTex entry to a file. The file will automatically be created if it does not already exist. If the file exists then the entry will be appended to the file. 

``` bash
> bibget -f test.bib 10.1177/17562848211064004
```

```
@article{Lucaciu_2021,
	doi = {10.1177/17562848211064004},
	url = {https://doi.org/10.1177%2F17562848211064004},
	year = 2021,
	month = {jan},
	publisher = {{SAGE} Publications},
	volume = {14},
	pages = {175628482110640},
	author = {Laura A. Lucaciu and Nathan Constantine-Cooke and Nikolas Plevris and Spyros Siakavellas and Lauranne A.A.P. Derikx and Gareth-Rhys Jones and Charles W. Lees},
	title = {Real-world experience with tofacitinib in ulcerative colitis: a systematic review and meta-analysis},
	journal = {Therapeutic Advances in Gastroenterology}
}
```
