# PDFTool

A fast, lightweight tool for PDF manipulation. Built in Rust, powered by Ghostscript. Available as a **desktop GUI** and a **CLI**.

## Features

- **Extract** specific pages from a PDF
- **Extract Text** from a PDF to a `.txt` file
- **Compress** a PDF to reduce file size
- **Convert** PDF pages to images (PNG or JPEG)
- **Markdown → PDF** conversion
- **PDF → Markdown** conversion

## Install (Windows)

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/install.ps1 | iex
```

Then press **Win**, type **PDFTool**, press **Enter**.

No dependencies needed — Ghostscript, Pandoc, Typst, and pdftotext are all bundled.

### Update

Run the same install command. It detects the existing installation and updates it:

```powershell
irm https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/install.ps1 | iex
```

### Uninstall

```powershell
irm https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/uninstall.ps1 | iex
```

## GUI

The desktop application provides a simple tabbed interface:

- **Extract tab** — Extract pages or text from a PDF (mode selector)
- **Compress tab** — Reduce PDF file size with quality presets
- **Convert tab** — PDF to images, Markdown to PDF, or PDF to Markdown (mode selector)
- **Browse** buttons to select input files and output directories
- Output files are saved to the **Downloads folder** by default

## CLI Usage

The CLI tool is also available for terminal users.

### Prerequisites (CLI only)

- [Ghostscript](https://www.ghostscript.com/releases/gsdnld.html) — for extract, compress, convert
- [pdftotext](https://www.xpdfreader.com/download.html) — for text extraction & PDF→Markdown
- [Pandoc](https://pandoc.org/installing.html) + [Typst](https://typst.app/) — for Markdown→PDF

### Extract pages

```bash
pdftool extract input.pdf --pages 2-5 -o output.pdf
pdftool extract input.pdf -p 1,3,7-10
```

Supported page range formats: `3`, `2-5`, `1,3,5`, `1,3-5,8`

### Extract text

```bash
pdftool text input.pdf -o output.txt
```

### Compress a PDF

```bash
pdftool compress input.pdf -o compressed.pdf
pdftool compress input.pdf -q screen
```

| Quality | DPI | Best for |
|---------|-----|----------|
| `screen` | 72 | Smallest file, screen reading |
| `ebook` | 150 | Good balance (default) |
| `printer` | 300 | Print quality |
| `prepress` | 300 | Maximum quality |

### Convert to images

```bash
pdftool convert input.pdf
pdftool convert input.pdf -f jpeg -d 150 -o ./images/
```

### Markdown to PDF

```bash
pdftool md-to-pdf document.md -o document.pdf
```

### PDF to Markdown

```bash
pdftool pdf-to-md input.pdf -o output.md
```

## Build from source

```bash
git clone https://github.com/Theo-Lempereur/Manipulateur-Pdf.git
cd Manipulateur-Pdf
```

**GUI (with all bundled tools):**

```powershell
.\scripts\setup-ghostscript.ps1
.\scripts\setup-pandoc.ps1
cargo build --release -p pdftool-gui
```

**CLI only:**

```bash
cargo build --release -p pdftool-cli
```

## Project Structure

```
pdftool-core/    # Shared library (extract, compress, convert, text, markdown)
pdftool-cli/     # CLI binary
src-tauri/       # Tauri GUI backend
ui/              # GUI frontend (HTML/CSS/JS)
scripts/         # Build, package & install scripts
```

## Tech Stack

- **Rust** — CLI, core logic, binary generation
- **Tauri 2** — Desktop GUI
- **Ghostscript** — PDF operations engine
- **pdftotext** — Text extraction from PDFs
- **Pandoc + Typst** — Markdown ↔ PDF conversion
- **clap** — CLI argument parsing

## License

MIT
