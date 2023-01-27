# getbib

CLI tool to generate a BibTex entry from a DOI. Written in Rust and based on the [`doi2bib` crate](https://crates.io/crates/doi2bib). 

## Installation

`getbib` can be installed from source using `cargo`

``` bash
cargo install --git https://github.com/nathansam.getbib.git getbib
```

## Usage

`getbib` supports an optional `-f/--file` argument for writing the BibTex entry to a file. The file will automatically be created if it does not already exist. If the file exists then the entry will be appended to the file. 

``` bash
> getbib -f test.bib 10.1177/17562848211064004
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
