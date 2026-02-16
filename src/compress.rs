use std::path::Path;
use std::process::Command;

fn gs_command() -> &'static str {
    if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    }
}

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

    let status = Command::new(gs_command())
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

    println!(
        "Compressed PDF (quality: {}) saved to {}",
        quality,
        output.display()
    );
    Ok(())
}
