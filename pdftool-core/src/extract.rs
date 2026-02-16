use std::path::Path;
use std::process::Command;

use crate::gs_command;

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

    let status = Command::new(gs_command())
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
