mod cli;
mod compress;
mod convert;
mod extract;

use std::process;

use clap::Parser;
use cli::{Cli, Commands, parse_page_range};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            input,
            pages,
            output,
        } => {
            let pages = match parse_page_range(&pages) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error parsing page range: {}", e);
                    process::exit(1);
                }
            };

            let output = output.unwrap_or_else(|| {
                let stem = input
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}_extracted.pdf", stem))
            });

            if let Err(e) = extract::extract_pages(&input, &output, &pages) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }

        Commands::Compress {
            input,
            quality,
            output,
        } => {
            let output = output.unwrap_or_else(|| {
                let stem = input
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}_compressed.pdf", stem))
            });

            if let Err(e) = compress::compress_pdf(&input, &output, &quality) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }

        Commands::Convert {
            input,
            format,
            dpi,
            output,
        } => {
            let output_dir = output.unwrap_or_else(|| std::path::PathBuf::from("."));

            if let Err(e) = convert::convert_pdf(&input, &output_dir, &format, dpi) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
