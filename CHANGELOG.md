# Changelog

## bibget 1.0.0

The bump to version 1.0.0 indicates that the binary is now considered
stable. However, additional features (such as resolving other identifiers) may
be added in the future. 

#### Changed

Shorted error message given if http request fails. 

### Dependencies updated to latest versions

* clap 4.1.4 -> 4.1.6

## bibget 0.0.4

### Added

Error handling for when the DOI is not found.

## bibget 0.0.3

### Added 

* Test suite has been set up for the CLI. 

### Changed 

* Size has been reduced by removing unused flags from dependencies and stripping symbols from the binary.

## bibget 0.0.2

### Added 

* `bibget` now ships with LICENSE files (MIT and Apache 2.0)

### Changed 

* Installation instructions now includes installing from crates.io.

* Usage demonstration in README.md now uses the DOI for a commonly cited paper. 

### Dependencies updated to latest versions

* futures 0.3.25 -> 0.3.26
* tokio 1.24.2 -> 1.25.0
