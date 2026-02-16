# PDFTool

A fast, lightweight tool for PDF manipulation. Built in Rust, powered by Ghostscript. Available as a **CLI** and a **desktop GUI** (Tauri).

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
```

**CLI only:**

```bash
cargo build --release -p pdftool
```

Binary at `target/release/pdftool.exe` (Windows) or `target/release/pdftool`.

**Desktop GUI (Tauri):**

```bash
cargo build --release -p pdftool-gui
```

Binary at `target/release/pdftool-gui.exe` (Windows).

## GUI

The desktop application provides a simple tabbed interface for all three operations. Output files are saved to the Downloads folder by default — just pick your input PDF, configure options, and click the action button.

- Browse buttons to select input files and output directories
- File names are entered without extension (added automatically)
- Default output directory: Downloads folder

## CLI Usage

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
- **Tauri 2** for the desktop GUI (HTML/CSS/JS frontend)
- **Ghostscript** for all PDF operations
- **clap** for CLI argument parsing

## Project Structure

```
pdftool-core/    # Shared library (extract, compress, convert logic)
pdftool-cli/     # CLI binary
src-tauri/       # Tauri GUI backend (Rust commands)
ui/              # GUI frontend (HTML/CSS/JS)
```

## License

MIT
