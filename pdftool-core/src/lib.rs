mod extract;
mod compress;
mod convert;

pub use extract::extract_pages;
pub use compress::compress_pdf;
pub use convert::convert_pdf;

pub fn gs_command() -> &'static str {
    if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    }
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
