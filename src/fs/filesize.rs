//! Human-readable file-size parsing and formatting.

/// Parses a human-readable file-size string into a raw byte count.
///
/// Accepted units: `K`, `KB`, `KiB`, `M`, `MB`, `MiB`, `G`, `GB`, `GiB`,
/// `T`, `TB`, `TiB`, `P`, `PB`, `PiB` (case-insensitive).
/// All units use base-1024 (binary) multipliers.
/// A bare integer with no unit is treated as bytes.
///
/// # Returns
///
/// `Some(bytes)` on success, or `None` if the input is empty, the numeric
/// part is missing or contains a decimal point (e.g. `"1.5G"`), the value is
/// negative, or the unit is unrecognised.
///
/// # Examples
///
/// ```
/// use alumy::fs::filesize::parse_size;
///
/// assert_eq!(parse_size("10M"),    Some(10 * 1024 * 1024));
/// assert_eq!(parse_size("1 GiB"), Some(1024 * 1024 * 1024));
/// assert_eq!(parse_size("512"),    Some(512));
/// assert_eq!(parse_size("1.5G"),   None);
/// assert_eq!(parse_size("bad"),    None);
/// assert_eq!(parse_size(""),       None);
/// ```
pub fn parse_size(size_str: &str) -> Option<u64> {
    const UNITS: &[(&str, u64)] = &[
        ("PIB", 1024u64.pow(5)), ("PB", 1024u64.pow(5)), ("P", 1024u64.pow(5)),
        ("TIB", 1024u64.pow(4)), ("TB", 1024u64.pow(4)), ("T", 1024u64.pow(4)),
        ("GIB", 1024u64.pow(3)), ("GB", 1024u64.pow(3)), ("G", 1024u64.pow(3)),
        ("MIB", 1024u64.pow(2)), ("MB", 1024u64.pow(2)), ("M", 1024u64.pow(2)),
        ("KIB", 1024),           ("KB", 1024),           ("K", 1024),
        ("B", 1),
    ];

    let s = size_str.trim().to_uppercase();
    if s.is_empty() {
        return None;
    }

    let (val_str, multiplier) = UNITS
        .iter()
        .find(|(suffix, _)| s.ends_with(suffix))
        .map(|(suffix, mult)| (&s[..s.len() - suffix.len()], *mult))
        .unwrap_or((s.as_str(), 1));

    val_str.trim().parse::<u64>().ok().map(|v| v * multiplier)
}

/// Formats a byte count as a human-readable string using binary (base-1024) units.
///
/// The output uses the shortest applicable unit with one decimal place
/// (e.g. `"1.5KB"`, `"10.0MB"`). Values smaller than 1 KiB are formatted
/// as a whole number of bytes (e.g. `"512B"`).
///
/// # Examples
///
/// ```
/// use alumy::fs::filesize::format_size;
///
/// assert_eq!(format_size(512),             "512B");
/// assert_eq!(format_size(1024),            "1.0KB");
/// assert_eq!(format_size(1_048_576),       "1.0MB");
/// assert_eq!(format_size(1_073_741_824),   "1.0GB");
/// assert_eq!(format_size(1536),            "1.5KB");
/// ```
pub fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    const TIB: u64 = GIB * 1024;
    const PIB: u64 = TIB * 1024;

    if bytes < KIB {
        format!("{}B", bytes)
    } else if bytes < MIB {
        format!("{:.1}KB", bytes as f64 / KIB as f64)
    } else if bytes < GIB {
        format!("{:.1}MB", bytes as f64 / MIB as f64)
    } else if bytes < TIB {
        format!("{:.1}GB", bytes as f64 / GIB as f64)
    } else if bytes < PIB {
        format!("{:.1}TB", bytes as f64 / TIB as f64)
    } else {
        format!("{:.1}PB", bytes as f64 / PIB as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        // Basic units (binary)
        assert_eq!(parse_size("1k"), Some(1024));
        assert_eq!(parse_size("1kb"), Some(1024));
        assert_eq!(parse_size("1kib"), Some(1024));
        assert_eq!(parse_size("10M"), Some(10 * 1024 * 1024));
        assert_eq!(parse_size("10MB"), Some(10 * 1024 * 1024));
        assert_eq!(parse_size("10MiB"), Some(10 * 1024 * 1024));
        assert_eq!(parse_size("2G"), Some(2 * 1024 * 1024 * 1024));
        assert_eq!(parse_size("2GB"), Some(2 * 1024 * 1024 * 1024));
        assert_eq!(parse_size("2GiB"), Some(2 * 1024 * 1024 * 1024));
        assert_eq!(parse_size("1T"), Some(1024u64.pow(4)));
        assert_eq!(parse_size("1TB"), Some(1024u64.pow(4)));
        assert_eq!(parse_size("1TiB"), Some(1024u64.pow(4)));
        assert_eq!(parse_size("1P"), Some(1024u64.pow(5)));
        assert_eq!(parse_size("1PB"), Some(1024u64.pow(5)));
        assert_eq!(parse_size("1PiB"), Some(1024u64.pow(5)));

        // No unit (bytes)
        assert_eq!(parse_size("512"), Some(512));
        assert_eq!(parse_size("1024B"), Some(1024));

        // Case insensitivity
        assert_eq!(parse_size("1KB"), Some(1024));
        assert_eq!(parse_size("1kb"), Some(1024));
        assert_eq!(parse_size("1Kb"), Some(1024));

        // Whitespace handling
        assert_eq!(parse_size("  1024  "), Some(1024));
        assert_eq!(parse_size("  1 M  "), Some(1024 * 1024));

        // Invalid inputs
        assert_eq!(parse_size("invalid"), None);
        assert_eq!(parse_size(""), None);
        assert_eq!(parse_size("1.5G"), None);
        assert_eq!(parse_size("-1024"), None);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512B");
        assert_eq!(format_size(1024), "1.0KB");
        assert_eq!(format_size(1024 * 1024), "1.0MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0GB");
        assert_eq!(format_size(1024u64.pow(4)), "1.0TB");
        assert_eq!(format_size(1024u64.pow(5)), "1.0PB");
        assert_eq!(format_size(1536), "1.5KB");
    }
}
