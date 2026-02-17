use std::path::Path;
use std::process::Command;

/// Find the Pandoc executable.
/// Priority: bundled next to exe (pandoc/pandoc.exe) → system PATH → known install locations.
fn pandoc_command() -> String {
    if cfg!(target_os = "windows") {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                // Bundled in pandoc/ subfolder
                let bundled = exe_dir.join("pandoc").join("pandoc.exe");
                if bundled.exists() {
                    return bundled.to_string_lossy().to_string();
                }
            }
        }
        // Known install location
        if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
            let known = std::path::Path::new(&local_app).join("Pandoc").join("pandoc.exe");
            if known.exists() {
                return known.to_string_lossy().to_string();
            }
        }
    }
    "pandoc".to_string()
}

/// Detect which PDF engine is available for Pandoc.
/// Priority:
///   1. Bundled typst next to exe (pandoc/typst.exe)
///   2. pdflatex in PATH
///   3. typst in PATH
///   4. Known WinGet install locations for typst
///   5. Error with install instructions
fn find_pdf_engine() -> Result<String, Box<dyn std::error::Error>> {
    // 1. Bundled typst
    if cfg!(target_os = "windows") {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let bundled = exe_dir.join("pandoc").join("typst.exe");
                if bundled.exists() {
                    return Ok(bundled.to_string_lossy().to_string());
                }
            }
        }
    }

    // 2. Check system PATH for pdflatex, then typst
    let candidates = ["pdflatex", "typst"];
    for engine in &candidates {
        let check = if cfg!(target_os = "windows") {
            Command::new("where").arg(engine).output()
        } else {
            Command::new("which").arg(engine).output()
        };

        if let Ok(output) = check {
            if output.status.success() {
                return Ok(engine.to_string());
            }
        }
    }

    // 3. On Windows, check known install locations
    if cfg!(target_os = "windows") {
        if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
            // WinGet packages
            let winget_dir =
                std::path::Path::new(&local_app).join("Microsoft\\WinGet\\Packages");
            if winget_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&winget_dir) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.starts_with("Typst.Typst") {
                            let path = find_exe_in_dir(&entry.path(), "typst.exe");
                            if let Some(p) = path {
                                return Ok(p);
                            }
                        }
                    }
                }
            }
        }
    }

    Err("No PDF engine found. Install one of: pdflatex (MiKTeX/TinyTeX) or typst.\n\
         Install typst:  winget install typst\n\
         Install MiKTeX: winget install MiKTeX.MiKTeX"
        .into())
}

/// Recursively search for an executable in a directory.
fn find_exe_in_dir(dir: &std::path::Path, exe_name: &str) -> Option<String> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().eq_ignore_ascii_case(exe_name) {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            } else if path.is_dir() {
                if let Some(found) = find_exe_in_dir(&path, exe_name) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// Convert a Markdown file to PDF using Pandoc + an auto-detected PDF engine.
pub fn md_to_pdf(
    input: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    // Verify the file has a .md extension
    match input.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") => {}
        _ => {
            return Err("Input file must be a Markdown file (.md or .markdown)".into());
        }
    }

    let engine = find_pdf_engine()?;

    let is_typst = engine.contains("typst");

    let mut args = vec![
        input.display().to_string(),
        "-o".to_string(),
        output.display().to_string(),
        format!("--pdf-engine={}", engine),
    ];

    // For typst: reduce margins and font size for better table rendering
    if is_typst {
        args.extend([
            "-V".to_string(), "margin-top=1.5cm".to_string(),
            "-V".to_string(), "margin-bottom=1.5cm".to_string(),
            "-V".to_string(), "margin-left=1.5cm".to_string(),
            "-V".to_string(), "margin-right=1.5cm".to_string(),
            "-V".to_string(), "fontsize=10pt".to_string(),
            "-V".to_string(), "papersize=a4".to_string(),
        ]);
    }

    let status = Command::new(pandoc_command())
        .args(&args)
        .status()?;

    if !status.success() {
        return Err(format!(
            "pandoc exited with code: {} (engine: {})",
            status.code().unwrap_or(-1),
            engine
        )
        .into());
    }

    Ok(())
}
