use std::path::Path;
use std::process::Command;

use crate::{gs_command, gs_lib_path};

fn gs_device(format: &str) -> Result<&'static str, String> {
    match format {
        "png" => Ok("png16m"),
        "jpeg" | "jpg" => Ok("jpeg"),
        _ => Err(format!("Unsupported format '{}'. Must be: png, jpeg", format)),
    }
}

pub fn convert_pdf(
    input: &Path,
    output_dir: &Path,
    format: &str,
    dpi: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    let device = gs_device(format)?;

    let extension = match format {
        "jpg" => "jpeg",
        other => other,
    };

    let stem = input
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    std::fs::create_dir_all(output_dir)?;

    let output_pattern = output_dir
        .join(format!("{}_%03d.{}", stem, extension))
        .display()
        .to_string();

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
            &format!("-sDEVICE={}", device),
            &format!("-r{}", dpi),
            "-dNOPAUSE",
            "-dBATCH",
            "-dSAFER",
            &format!("-sOutputFile={}", output_pattern),
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
