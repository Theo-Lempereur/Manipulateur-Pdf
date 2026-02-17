mod extract;
mod compress;
mod convert;
mod text_extract;
mod md_to_pdf;

pub use extract::extract_pages;
pub use compress::compress_pdf;
pub use convert::convert_pdf;
pub use text_extract::extract_text;
pub use md_to_pdf::md_to_pdf;

use std::path::PathBuf;

/// Return the path to the Ghostscript executable.
/// Priority:
/// 1. Bundled GS next to the current executable (ghostscript/bin/gswin64c.exe)
/// 2. System-installed GS in PATH
pub fn gs_command() -> String {
    if cfg!(target_os = "windows") {
        // Look for bundled Ghostscript next to our executable
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let bundled = exe_dir.join("ghostscript").join("bin").join("gswin64c.exe");
                if bundled.exists() {
                    return bundled.to_string_lossy().to_string();
                }
            }
        }
        "gswin64c".to_string()
    } else {
        "gs".to_string()
    }
}

/// Return the path to the bundled GS lib directory, if it exists.
/// Ghostscript needs this to find its init files.
pub fn gs_lib_path() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let gs_dir = exe_dir.join("ghostscript");
            let lib_dir = gs_dir.join("lib");
            let res_dir = gs_dir.join("Resource");
            if lib_dir.exists() && res_dir.exists() {
                return Some(gs_dir);
            }
        }
    }
    None
}

/// Parse a page range string into a sorted, deduplicated list of page numbers.
pub fn parse_page_range(pages: &str) -> Result<Vec<u32>, String> {
    let mut result = Vec::new();

    for part in pages.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.splitn(2, '-').collect();
            let start: u32 = bounds[0]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid range start: '{}'", bounds[0].trim()))?;
            let end: u32 = bounds[1]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid range end: '{}'", bounds[1].trim()))?;
            if start == 0 || end == 0 {
                return Err("Page numbers must be greater than 0".to_string());
            }
            if start > end {
                return Err(format!("Invalid range: {}-{} (start > end)", start, end));
            }
            for page in start..=end {
                result.push(page);
            }
        } else {
            let page: u32 = part
                .parse()
                .map_err(|_| format!("Invalid page number: '{}'", part))?;
            if page == 0 {
                return Err("Page numbers must be greater than 0".to_string());
            }
            result.push(page);
        }
    }

    result.sort();
    result.dedup();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_page() {
        assert_eq!(parse_page_range("3").unwrap(), vec![3]);
    }

    #[test]
    fn test_range() {
        assert_eq!(parse_page_range("2-5").unwrap(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_comma_separated() {
        assert_eq!(parse_page_range("1,3,5").unwrap(), vec![1, 3, 5]);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(parse_page_range("1,3-5,8").unwrap(), vec![1, 3, 4, 5, 8]);
    }

    #[test]
    fn test_dedup() {
        assert_eq!(parse_page_range("1,1,2").unwrap(), vec![1, 2]);
    }

    #[test]
    fn test_zero_page_error() {
        assert!(parse_page_range("0").is_err());
    }

    #[test]
    fn test_invalid_range_error() {
        assert!(parse_page_range("5-2").is_err());
    }

    #[test]
    fn test_invalid_input_error() {
        assert!(parse_page_range("abc").is_err());
    }
}
