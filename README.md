# <img src="https://raw.githubusercontent.com/nathansam/bibget/master/bibget.png" alt = "bibget logo" width = 200>

[![CI](https://github.com/nathansam/bibget/actions/workflows/rust.yml/badge.svg)](https://github.com/nathansam/bibget/actions/workflows/rust.yml) [![crates.io](https://img.shields.io/crates/v/bibget.svg)](https://crates.io/crates/bibget) [![dependency status](https://deps.rs/repo/github/nathansam/bibget/status.svg)](https://deps.rs/repo/github/nathansam/bibget) [![MIT](https://img.shields.io/crates/l/bibget.svg)](https://github.com/nathansam/bibget/blob/master/LICENSE) 

Command line interface tool for generating BibTex entries from DOIs. Written in 
Rust and based on the [`doi2bib` crate](https://crates.io/crates/doi2bib).  

## Installation

Pre-compiled binaries for MacOS and Linux are available from the
[releases page](https://github.com/nathansam/bibget/releases) and do not require
Rust to be installed. 

`bibget` can be installed from source using `cargo`. The most stable version is
hosted on [crates.io](https://crates.io/crates/bibget).

``` bash
# release version from crates.io
cargo install bibget 
```

You can also install from GitHub if you want access to the latest pre-release
(likely with newer dependencies) or development (more experimental) builds.

``` bash
# pre-release version
cargo install --git https://github.com/nathansam/bibget.git bibget 
# development version 
cargo install --git https://github.com/nathansam/bibget.git --branch dev bibget 
```

## Usage

`bibget` is a command line tool that takes a DOI as an argument and returns
output in BibTex format. Multiple DOIs can be passed via whitespace separation.

``` bash
bibget 10.1002/sim.1186 10.1007/978-3-319-19425-7
```

``` bibtex
 @article{Higgins_2002,
   title = {Quantifying heterogeneity in a meta‐analysis},
   volume = {21},
   ISSN = {1097-0258},
   url = {http://dx.doi.org/10.1002/sim.1186},
   DOI = {10.1002/sim.1186},
   number = {11},
   journal = {Statistics in Medicine},
   publisher = {Wiley},
   author = {Higgins, Julian P. T. and Thompson, Simon G.},
   year = {2002},
   month = may,
   pages = {1539–1558}
}

 @book{Harrell__2015,
  _2015,
   title = {Regression Modeling Strategies: With Applications to Linear Models, Logistic and Ordinal Regression, and Survival Analysis},
   ISBN = {9783319194257},
   ISSN = {2197-568X},
   url = {http://dx.doi.org/10.1007/978-3-319-19425-7},
   DOI = {10.1007/978-3-319-19425-7},
   journal = {Springer Series in Statistics},
   publisher = {Springer International Publishing},
   author = {Harrell , Frank E.},
   year = {2015}
}
```

`bibget` supports an optional `-f/--file` flag for writing the BibTex to
file. If the specified file does not already exist, it will be created first.
Entries will be appended to the end of existing files instead. 

``` bash
> bibget -f test.bib 10.1002/sim.1186
```

`bibget` can also be called via a lightweight docker image. Both Docker Hub and 
the GitHub container registry are supported:

``` bash
docker run nathansam12/bibget <doi> # Docker Hub
docker run ghcr.io/nathansam/bibget <doi> # GitHub Container Registry
```