#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use pdftool_core::{compress_pdf, convert_pdf, extract_pages, parse_page_range};

#[tauri::command]
fn cmd_extract(input: String, pages: String, output: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let output = if output.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
        parent.join(format!("{}_extracted.pdf", stem))
    } else {
        PathBuf::from(&output)
    };

    let page_list = parse_page_range(&pages).map_err(|e| e.to_string())?;
    extract_pages(&input, &output, &page_list).map_err(|e| e.to_string())?;

    Ok(format!("Pages extracted to {}", output.display()))
}

#[tauri::command]
fn cmd_compress(input: String, quality: String, output: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let output = if output.is_empty() {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
        parent.join(format!("{}_compressed.pdf", stem))
    } else {
        PathBuf::from(&output)
    };

    compress_pdf(&input, &output, &quality).map_err(|e| e.to_string())?;

    Ok(format!("Compressed PDF saved to {}", output.display()))
}

#[tauri::command]
fn cmd_convert(input: String, format: String, dpi: u32, output: String) -> Result<String, String> {
    let input = PathBuf::from(&input);
    let output_dir = if output.is_empty() {
        input.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()
    } else {
        PathBuf::from(&output)
    };

    convert_pdf(&input, &output_dir, &format, dpi).map_err(|e| e.to_string())?;

    Ok(format!("Converted to {} images in {}", format, output_dir.display()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cmd_extract, cmd_compress, cmd_convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
