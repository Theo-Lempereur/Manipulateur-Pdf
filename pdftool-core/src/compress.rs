use std::path::Path;
use std::process::Command;

use crate::{gs_command, gs_lib_path};

const VALID_QUALITIES: &[&str] = &["screen", "ebook", "printer", "prepress"];

pub fn compress_pdf(
    input: &Path,
    output: &Path,
    quality: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    if !VALID_QUALITIES.contains(&quality) {
        return Err(format!(
            "Invalid quality '{}'. Must be one of: {}",
            quality,
            VALID_QUALITIES.join(", ")
        )
        .into());
    }

    let mut cmd = Command::new(gs_command());

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
            "-dCompatibilityLevel=1.4",
            &format!("-dPDFSETTINGS=/{}", quality),
            "-dNOPAUSE",
            "-dBATCH",
            "-dSAFER",
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
