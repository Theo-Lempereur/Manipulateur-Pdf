use std::path::Path;
use std::process::Command;

/// Find the pdftotext executable (same logic as text_extract.rs).
fn pdftotext_command() -> String {
    if cfg!(target_os = "windows") {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let bundled = exe_dir.join("pdftotext.exe");
                if bundled.exists() {
                    return bundled.to_string_lossy().to_string();
                }
            }
        }
        let known_paths = [
            r"C:\Program Files\Git\mingw64\bin\pdftotext.exe",
            r"C:\Program Files (x86)\Git\mingw64\bin\pdftotext.exe",
        ];
        for path in &known_paths {
            if Path::new(path).exists() {
                return path.to_string();
            }
        }
    }
    "pdftotext".to_string()
}

/// Returns true if the line looks like a numbered list item (e.g. "1)", "12.", "a)")
fn is_list_item(line: &str) -> bool {
    let t = line.trim_start();
    // Patterns: "1)" "1." "1:" "12)" "a)" "a." etc.
    if let Some(first) = t.chars().next() {
        if first.is_ascii_digit() {
            // Check for "123)" or "123." or "123:"
            let rest: String = t.chars().skip_while(|c| c.is_ascii_digit()).collect();
            if rest.starts_with(')') || rest.starts_with('.') || rest.starts_with(':') {
                return true;
            }
        }
        if first.is_ascii_alphabetic() && t.len() > 1 {
            let second = t.chars().nth(1).unwrap_or(' ');
            if second == ')' || second == '.' {
                return true;
            }
        }
        // Bullet points
        if first == '-' || first == '•' || first == '–' || first == '*' {
            return true;
        }
    }
    false
}

/// Determine heading level for each line using text heuristics.
/// Approach:
/// - Compute median line length of non-trivial lines
/// - Short lines that are NOT list items and appear before/between content blocks
///   are candidates for headings
/// - Lines at the very start of a page block get higher priority
fn assign_heading_levels(lines: &[String]) -> Vec<u8> {
    if lines.is_empty() {
        return Vec::new();
    }

    // Collect lengths of non-empty, non-list-item, non-page-number lines
    let mut body_lengths: Vec<usize> = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if is_list_item(trimmed) {
            continue;
        }
        if trimmed.chars().all(|c| c.is_ascii_digit()) && trimmed.len() <= 4 {
            continue;
        }
        body_lengths.push(trimmed.len());
    }

    if body_lengths.is_empty() {
        return vec![0; lines.len()];
    }

    body_lengths.sort();
    let median_len = body_lengths[body_lengths.len() / 2];

    // Threshold: lines shorter than 50% of median are heading candidates
    // but only if median is reasonably large (>20 chars)
    let heading_threshold = if median_len > 20 {
        median_len / 2
    } else {
        0 // don't detect headings if lines are generally short
    };

    let mut levels: Vec<u8> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Default: body text
        if trimmed.is_empty()
            || is_list_item(trimmed)
            || (trimmed.chars().all(|c| c.is_ascii_digit()) && trimmed.len() <= 4)
            || heading_threshold == 0
        {
            levels.push(0);
            continue;
        }

        let len = trimmed.len();

        // Check if this line looks like a heading:
        // 1. Shorter than threshold
        // 2. Preceded by a blank line or is the first line
        // 3. Followed by a non-empty line or blank then content
        let preceded_by_blank = i == 0
            || lines
                .get(i.wrapping_sub(1))
                .map_or(true, |l| l.trim().is_empty());

        let is_short = len <= heading_threshold && len >= 3;

        if is_short && preceded_by_blank {
            // Assign # for very short (title-like), ## for slightly longer
            if len <= heading_threshold / 2 {
                levels.push(1);
            } else {
                levels.push(2);
            }
        } else {
            levels.push(0);
        }
    }

    levels
}

/// Convert raw extracted text into Markdown with heading detection.
fn format_as_markdown(raw: &str) -> String {
    let lines: Vec<String> = raw.lines().map(|l| l.to_string()).collect();
    let levels = assign_heading_levels(&lines);

    let mut md = String::new();
    let mut prev_blank = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip form feed characters (page breaks)
        if trimmed == "\u{0C}" || trimmed.is_empty() {
            if !prev_blank && !md.is_empty() {
                md.push('\n');
                prev_blank = true;
            }
            continue;
        }

        // Skip standalone page numbers
        if trimmed.chars().all(|c| c.is_ascii_digit()) && trimmed.len() <= 4 {
            continue;
        }

        prev_blank = false;
        let level = levels[i];

        if level > 0 {
            // Ensure blank line before heading
            if !md.is_empty() && !md.ends_with("\n\n") {
                if !md.ends_with('\n') {
                    md.push('\n');
                }
                md.push('\n');
            }
            for _ in 0..level {
                md.push('#');
            }
            md.push(' ');
            md.push_str(trimmed);
            md.push_str("\n\n");
        } else {
            md.push_str(trimmed);
            md.push('\n');
        }
    }

    // Clean up multiple blank lines
    while md.contains("\n\n\n") {
        md = md.replace("\n\n\n", "\n\n");
    }

    md.trim().to_string() + "\n"
}

/// Convert a PDF file to Markdown using pdftotext with UTF-8 encoding.
pub fn pdf_to_md(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        return Err(format!("Input file not found: {}", input.display()).into());
    }

    let temp_txt = output.with_extension("tmp.txt");

    let status = Command::new(pdftotext_command())
        .args([
            "-enc",
            "UTF-8",
            "-layout",
            &input.display().to_string(),
            &temp_txt.display().to_string(),
        ])
        .status()?;

    if !status.success() {
        let _ = std::fs::remove_file(&temp_txt);
        return Err(format!(
            "pdftotext exited with code: {}",
            status.code().unwrap_or(-1)
        )
        .into());
    }

    // Read as bytes and decode (UTF-8 should work now, Latin-1 fallback just in case)
    let raw_bytes = std::fs::read(&temp_txt)?;
    let _ = std::fs::remove_file(&temp_txt);
    let raw_text = match String::from_utf8(raw_bytes.clone()) {
        Ok(s) => s,
        Err(_) => raw_bytes.iter().map(|&b| b as char).collect(),
    };

    let markdown = format_as_markdown(&raw_text);
    std::fs::write(output, &markdown)?;

    Ok(())
}
