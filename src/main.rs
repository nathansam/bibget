use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[clap(
    author = "Nathan Constantine-Cooke",
    version,
    about = "Simple tool to convert a DOI to a BibTeX entry and (optionally) append it to a file."
)]
struct Cli {
    /// The path to the file to written to. If not specified, the BibTex entry will only be printed to stdout.
    #[clap(short = 'f', long = "file", required = false, default_value = "none")]
    file: String,
    /// The DOI to look up
    #[clap()]
    doi: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let doi = args.doi;
    let doi2bib = doi2bib::Doi2Bib::new().unwrap();
    let bibtex_result = doi2bib.resolve_doi(&doi).await;

    let mut bibtex = match bibtex_result {
        Ok(file) => file,
        Err(e) => {
            if e.is_status() {
                println!("❌ Getting the BibTex entry failed. This is likely either because the DOI is invalid or you do not have internet access.");
                std::process::exit(1);
            } else {
                panic!("Error: {}", e)
            }
        }
    };

    bibtex.push('\n');
    println!("{bibtex}");

    let file = args.file;
    if file != "none" {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(file)
            .expect("Unable to open file");
        f.write_all(bibtex.as_bytes())
            .expect("Unable to write data");
    }
}
