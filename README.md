# PDFTool

A fast, lightweight tool for PDF manipulation. Built in Rust, powered by Ghostscript. Available as a **desktop GUI** and a **CLI**.

## Features

- **Extract** specific pages from a PDF
- **Compress** a PDF to reduce file size
- **Convert** PDF pages to images (PNG or JPEG)

## Install (Windows)

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/install.ps1 | iex
```

Then press **Win**, type **PDFTool**, press **Enter**.

No dependencies needed — Ghostscript is bundled.

### Uninstall

```powershell
irm https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/uninstall.ps1 | iex
```

## GUI

The desktop application provides a simple tabbed interface for all three operations:

- **Browse** buttons to select input files and output directories
- Output files are saved to the **Downloads folder** by default
- File names are entered without extension (added automatically)

Just pick your input PDF, configure options, and click the action button.

## CLI Usage

The CLI tool is also available for terminal users.

### Prerequisites (CLI only)

[Ghostscript](https://www.ghostscript.com/releases/gsdnld.html) must be installed and in your PATH.

### Extract pages

```bash
pdftool extract input.pdf --pages 2-5 -o output.pdf
pdftool extract input.pdf -p 1,3,7-10
```

Supported page range formats: `3`, `2-5`, `1,3,5`, `1,3-5,8`

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

## Build from source

```bash
git clone https://github.com/Theo-Lempereur/Manipulateur-Pdf.git
cd Manipulateur-Pdf
```

**GUI (with bundled Ghostscript):**

```powershell
.\scripts\setup-ghostscript.ps1
cargo build --release -p pdftool-gui
```

**CLI only:**

```bash
cargo build --release -p pdftool-cli
```

## Project Structure

```
pdftool-core/    # Shared library (extract, compress, convert)
pdftool-cli/     # CLI binary
src-tauri/       # Tauri GUI backend
ui/              # GUI frontend (HTML/CSS/JS)
scripts/         # Build & install scripts
```

## Tech Stack

- **Rust** — CLI, core logic, binary generation
- **Tauri 2** — Desktop GUI
- **Ghostscript** — PDF operations engine
- **clap** — CLI argument parsing

## License

MIT
