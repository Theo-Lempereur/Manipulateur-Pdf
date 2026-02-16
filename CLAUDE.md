# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PDFTool is a Rust-based CLI tool for PDF manipulation (extraction, compression, conversion to images). The project deliberately does NOT reimplement a PDF engine—instead, it wraps Ghostscript for all PDF operations.

## Architecture

**Core Design Pattern:**
- Rust handles CLI interface, command orchestration, file management, and binary generation
- Ghostscript (`gs` command) performs all PDF operations via `std::process::Command`
- Single executable output, no runtime Rust installation required

**Key Philosophy:**
- Minimal, fast, portable tool
- Leverage existing robust tools (Ghostscript) rather than reimplementing
- Target non-technical end users

## Dependencies

**External Tool:**
- Ghostscript must be installed on the system
- Verify with: `gs --version`

**Rust Crates:**
- `clap` is recommended for CLI parsing (but not enforced)
- Architecture, modules, error handling, and logging approach are left to developer discretion

## Development Commands

**Build:**
```bash
cargo build --release
```

**Test Ghostscript integration:**
```bash
gs --version
```

## Feature Priority

Strict order for implementation:
1. Page extraction (`pdftool extract input.pdf --pages 2-5 -o out.pdf`)
2. PDF compression (`pdftool compress input.pdf -o compressed.pdf`)
3. PDF to image conversion (`pdftool convert input.pdf --format png`)

**Out of scope:**
- Conversion to Word/HTML
- Graphical interface (CLI only)

## Platform Support

- Primary target: Windows
- Goal: Cross-platform compatibility (Windows/Linux/macOS)
