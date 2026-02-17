use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use pdftool_core::{compress_pdf, convert_pdf, extract_pages, extract_text, md_to_pdf, parse_page_range};

#[derive(Parser)]
#[command(name = "pdftool", about = "CLI tool for PDF manipulation using Ghostscript")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract specific pages from a PDF file
    Extract {
        /// Input PDF file
        input: PathBuf,
        /// Page range (e.g. "2-5", "1,3,5", "1,3-5,8")
        #[arg(short, long)]
        pages: String,
        /// Output PDF file (default: input_extracted.pdf)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert PDF pages to images
    Convert {
        /// Input PDF file
        input: PathBuf,
        /// Output image format: png, jpeg
        #[arg(short, long, default_value = "png")]
        format: String,
        /// Resolution in DPI
        #[arg(short, long, default_value = "300")]
        dpi: u32,
        /// Output directory (default: current directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Extract text content from a PDF to a text file
    Text {
        /// Input PDF file
        input: PathBuf,
        /// Output text file (default: input.txt)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Compress a PDF file
    Compress {
        /// Input PDF file
        input: PathBuf,
        /// Compression quality: screen, ebook, printer, prepress
        #[arg(short, long, default_value = "ebook")]
        quality: String,
        /// Output PDF file (default: input_compressed.pdf)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert a Markdown file to PDF
    MdToPdf {
        /// Input Markdown file
        input: PathBuf,
        /// Output PDF file (default: input.pdf)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

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
                let stem = input.file_stem().unwrap_or_default().to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}_extracted.pdf", stem))
            });

            if let Err(e) = extract_pages(&input, &output, &pages) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Extracted pages to {}", output.display());
        }

        Commands::Text { input, output } => {
            let output = output.unwrap_or_else(|| {
                let stem = input.file_stem().unwrap_or_default().to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}.txt", stem))
            });

            if let Err(e) = extract_text(&input, &output) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Text extracted to {}", output.display());
        }

        Commands::Compress {
            input,
            quality,
            output,
        } => {
            let output = output.unwrap_or_else(|| {
                let stem = input.file_stem().unwrap_or_default().to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}_compressed.pdf", stem))
            });

            if let Err(e) = compress_pdf(&input, &output, &quality) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Compressed PDF saved to {}", output.display());
        }

        Commands::Convert {
            input,
            format,
            dpi,
            output,
        } => {
            let output_dir = output.unwrap_or_else(|| PathBuf::from("."));

            if let Err(e) = convert_pdf(&input, &output_dir, &format, dpi) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Converted PDF to {} images in {}", format, output_dir.display());
        }

        Commands::MdToPdf { input, output } => {
            let output = output.unwrap_or_else(|| {
                let stem = input.file_stem().unwrap_or_default().to_string_lossy();
                let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
                parent.join(format!("{}.pdf", stem))
            });

            if let Err(e) = md_to_pdf(&input, &output) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Converted Markdown to PDF: {}", output.display());
        }
    }
}
