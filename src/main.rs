use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[clap(
    author = "Nathan Constantine-Cooke",
    version,
    about = "Simple tool to convert DOI(s) to BibTeX and (optionally) write to storage."
)]
struct Cli {
    /// A path for writing the BibTex to. The file will be created if it does not already exist or appended to if it already exists.
    #[clap(short, long, required = false, default_value = None)]
    file: Option<String>,

    /// The DOI(s) to convert. Multiple DOIs should be separated by spaces.
    #[clap(value_delimiter = ' ', num_args = 1..)]
    doi: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let doi_vec = args.doi;
    let file = args.file;
    let mut format_bibtex_combined = String::from("");

    let mut it = doi_vec.iter().peekable();
    while let Some(doi) = it.next() {
        let doi2bib = doi2bib::Doi2Bib::new().unwrap();
        let bibtex_result = doi2bib.resolve_doi(doi).await;

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
  

        // Format BibTex output to be more readable (new lines and spaces)
        let format_bibtex = bib_format(bibtex);
        format_bibtex_combined.push_str(&format_bibtex);

        if it.peek().is_some() {
            format_bibtex_combined.push('\n');
        }
    }

    println!("{format_bibtex_combined}");

    // Write to file (if specified)

    if file.is_some() {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(file.unwrap())
            .expect("Unable to open file");

        f.write_all(format_bibtex_combined.as_bytes())
            .expect("Unable to write data");
    }
}


fn bib_format(bibtex:String) -> String {
    let mut curly_count = 0;
    let mut format_bibtex = String::from("");
        for char in bibtex.chars() {
            if char == '{' {
                curly_count += 1;
            } else if char == '}' {
                curly_count -= 1;
            }
            if char == ',' && curly_count == 1 {
                format_bibtex.push_str(",\n  ");
            } else {
                format_bibtex.push(char);
            }
        }
        format_bibtex = format_bibtex.replace("} }", "}\n}"); // Format final line
        format_bibtex = format_bibtex.replace(" }", "}");
        format_bibtex = format_bibtex.replace("=", " = "); // Add spaces around equals sign

        return format_bibtex
    }


#[test]
fn test_bracket() {
    let mut example = String::from("title={Article name},");
    assert_eq!(bib_format(example), "title = {Article name},");
    example = String::from("author = {{Jane Doe} and {John Smith}\n},");
    assert_eq!(bib_format(example), "author  =  {{Jane Doe} and {John Smith}\n},");
}