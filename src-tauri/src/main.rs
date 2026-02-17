#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use pdftool_core::{compress_pdf, convert_pdf, extract_pages, extract_text, md_to_pdf, parse_page_range};

fn downloads_dir() -> PathBuf {
    dirs::download_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// Strip any existing extension from a user-provided name so we can
/// append the correct one ourselves.
fn strip_extension(name: &str) -> String {
    let p = std::path::Path::new(name);
    p.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
async fn pick_file(window: tauri::Window, filter: Option<String>) -> Result<String, String> {
    let mut dialog = rfd::AsyncFileDialog::new().set_parent(&window);
    match filter.as_deref() {
        Some("md") => {
            dialog = dialog.add_filter("Markdown", &["md", "markdown"]);
        }
        _ => {
            dialog = dialog.add_filter("PDF", &["pdf"]);
        }
    }
    let handle = dialog.pick_file().await;
    match handle {
        Some(h) => Ok(h.path().display().to_string()),
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
async fn pick_directory(window: tauri::Window) -> Result<String, String> {
    let handle = rfd::AsyncFileDialog::new()
        .set_parent(&window)
        .set_directory(downloads_dir())
        .pick_folder()
        .await;
    match handle {
        Some(h) => Ok(h.path().display().to_string()),
        None => Err("No directory selected".to_string()),
    }
}

#[tauri::command]
fn get_downloads_dir() -> String {
    downloads_dir().display().to_string()
}

#[tauri::command]
fn cmd_extract(input: String, pages: String, output_dir: String, output_name: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let dir = if output_dir.is_empty() { downloads_dir() } else { PathBuf::from(&output_dir) };
    let name = if output_name.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        format!("{}_extracted", stem)
    } else {
        strip_extension(&output_name)
    };
    let output = dir.join(format!("{}.pdf", name));

    let page_list = parse_page_range(&pages).map_err(|e| e.to_string())?;
    extract_pages(&input, &output, &page_list).map_err(|e| e.to_string())?;

    Ok(format!("Pages extracted to {}", output.display()))
}

#[tauri::command]
fn cmd_extract_text(input: String, output_dir: String, output_name: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let dir = if output_dir.is_empty() { downloads_dir() } else { PathBuf::from(&output_dir) };
    let name = if output_name.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        stem.to_string()
    } else {
        strip_extension(&output_name)
    };
    let output = dir.join(format!("{}.txt", name));

    extract_text(&input, &output).map_err(|e| e.to_string())?;

    Ok(format!("Text extracted to {}", output.display()))
}

#[tauri::command]
fn cmd_compress(input: String, quality: String, output_dir: String, output_name: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let dir = if output_dir.is_empty() { downloads_dir() } else { PathBuf::from(&output_dir) };
    let name = if output_name.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        format!("{}_compressed", stem)
    } else {
        strip_extension(&output_name)
    };
    let output = dir.join(format!("{}.pdf", name));

    compress_pdf(&input, &output, &quality).map_err(|e| e.to_string())?;

    Ok(format!("Compressed PDF saved to {}", output.display()))
}

#[tauri::command]
fn cmd_convert(input: String, format: String, dpi: u32, output_dir: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let dir = if output_dir.is_empty() { downloads_dir() } else { PathBuf::from(&output_dir) };

    convert_pdf(&input, &dir, &format, dpi).map_err(|e| e.to_string())?;

    Ok(format!("Converted to {} images in {}", format, dir.display()))
}

#[tauri::command]
fn cmd_md_to_pdf(input: String, output_dir: String, output_name: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let dir = if output_dir.is_empty() { downloads_dir() } else { PathBuf::from(&output_dir) };
    let name = if output_name.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        stem.to_string()
    } else {
        strip_extension(&output_name)
    };
    let output = dir.join(format!("{}.pdf", name));

    md_to_pdf(&input, &output).map_err(|e| e.to_string())?;

    Ok(format!("Markdown converted to PDF: {}", output.display()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            pick_file,
            pick_directory,
            get_downloads_dir,
            cmd_extract,
            cmd_extract_text,
            cmd_compress,
            cmd_convert,
            cmd_md_to_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
