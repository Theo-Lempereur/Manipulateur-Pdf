use std::path::Path;
use std::process::Command;

use crate::{gs_command, gs_lib_path};

pub fn extract_pages(
    input: &Path,
    output: &Path,
    pages: &[u32],
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    let page_list: String = pages
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut cmd = Command::new(gs_command());

    // Set GS library search path if bundled
    if let Some(gs_dir) = gs_lib_path() {
        let search_path = format!(
            "{};{};{}",
            gs_dir.join("lib").display(),
            gs_dir.join("Resource").display(),
            gs_dir.join("iccprofiles").display(),
        );
        cmd.env("GS_LIB", &search_path);
    }

    let status = cmd
        .args([
            "-sDEVICE=pdfwrite",
            "-dNOPAUSE",
            "-dBATCH",
            "-dSAFER",
            &format!("-sPageList={}", page_list),
            &format!("-sOutputFile={}", output.display()),
            &input.display().to_string(),
        ])
        .status()?;

    if !status.success() {
        return Err(format!(
            "Ghostscript exited with code: {}",
            status.code().unwrap_or(-1)
        )
        .into());
    }

    Ok(())
}
