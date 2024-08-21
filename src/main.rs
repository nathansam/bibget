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

    let bibtex = match bibtex_result {
        Ok(file) => file,
        Err(e) => {
            if e.is_status() {
                println!("❌ Getting the BibTex entry failed. Please check the DOI is valid and you have internet access.");
                std::process::exit(1);
            } else {
                panic!("Error: {}", e)
            }
        }
    };
    // Count the number of layers curly braces to ensure we only add newlines
    // after the first level of curly braces (i.e between new BibTex keys)
    let mut curly_count = 0;
    // Format BibTex output to be more readable (new lines and spaces)
    let mut format_bibtex = String::from("");

    for char in bibtex.chars() {
        if char == '{' {
            curly_count = curly_count + 1;
        } else if char == '}' {
            curly_count = curly_count - 1;
        }
        if char == ',' && curly_count == 1 {
            format_bibtex.push_str(",\n  ");
        } else {
            format_bibtex.push(char);
        }
    }

    format_bibtex = format_bibtex.replace("} }", "}\n}"); // Format final line
    format_bibtex = format_bibtex.replace("=", " = "); // Add spaces around equals sign

    println!("{format_bibtex}");

    // Write to file (if specified)
    let file = args.file;
    if file != "none" {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(file)
            .expect("Unable to open file");
        f.write_all(format_bibtex.as_bytes())
            .expect("Unable to write data");
    }
}
