# PDFTool

A fast, lightweight CLI tool for PDF manipulation. Built in Rust, powered by Ghostscript.

## Features

- **Extract** specific pages from a PDF
- **Compress** a PDF to reduce file size
- **Convert** PDF pages to images (PNG or JPEG)

## Installation

### Prerequisites

- [Ghostscript](https://www.ghostscript.com/releases/gsdnld.html) must be installed and available in your PATH
  - Windows: `gswin64c --version`
  - Linux/macOS: `gs --version`

### Build from source

```bash
git clone https://github.com/your-username/Manipulateur-Pdf.git
cd Manipulateur-Pdf
cargo build --release
```

The binary will be at `target/release/pdftool.exe` (Windows) or `target/release/pdftool`.

## Usage

### Extract pages

```bash
# Extract pages 2 to 5
pdftool extract input.pdf --pages 2-5 -o output.pdf

# Extract specific pages
pdftool extract input.pdf -p 1,3,7-10

# Output defaults to input_extracted.pdf
```

Supported page range formats: `3`, `2-5`, `1,3,5`, `1,3-5,8`

### Compress a PDF

```bash
# Default compression (ebook quality, 150 dpi)
pdftool compress input.pdf -o compressed.pdf

# Maximum compression (72 dpi)
pdftool compress input.pdf -q screen

# High quality for printing (300 dpi)
pdftool compress input.pdf -q printer
```

| Quality level | DPI | Best for |
|---------------|-----|----------|
| `screen` | 72 | Smallest file size, screen reading |
| `ebook` | 150 | Good balance (default) |
| `printer` | 300 | Print quality |
| `prepress` | 300 | Maximum quality, prepress |

### Convert to images

```bash
# Convert to PNG at 300 dpi (default)
pdftool convert input.pdf

# Convert to JPEG at 150 dpi
pdftool convert input.pdf -f jpeg -d 150

# Output to a specific directory
pdftool convert input.pdf -o ./images/
```

Output files are named `filename_001.png`, `filename_002.png`, etc.

## Tech Stack

- **Rust** for CLI, file management, and binary generation
- **Ghostscript** for all PDF operations
- **clap** for argument parsing

## License

MIT
