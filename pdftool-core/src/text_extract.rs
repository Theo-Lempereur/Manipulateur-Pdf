use std::path::Path;
use std::process::Command;

/// Find the pdftotext executable.
/// Priority: bundled next to exe, then known locations, then system PATH.
fn pdftotext_command() -> String {
    if cfg!(target_os = "windows") {
        // 1. Bundled next to our executable
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let bundled = exe_dir.join("pdftotext.exe");
                if bundled.exists() {
                    return bundled.to_string_lossy().to_string();
                }
            }
        }
        // 2. Known install locations
        let known_paths = [
            r"C:\Program Files\Git\mingw64\bin\pdftotext.exe",
            r"C:\Program Files (x86)\Git\mingw64\bin\pdftotext.exe",
        ];
        for path in &known_paths {
            if std::path::Path::new(path).exists() {
                return path.to_string();
            }
        }
    }
    // 3. Fall back to system PATH
    "pdftotext".to_string()
}

pub fn extract_text(
    input: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    let status = Command::new(pdftotext_command())
        .args([
            "-layout",
            &input.display().to_string(),
            &output.display().to_string(),
        ])
        .status()?;

    if !status.success() {
        return Err(format!(
            "pdftotext exited with code: {}",
            status.code().unwrap_or(-1)
        )
        .into());
    }

    Ok(())
}
