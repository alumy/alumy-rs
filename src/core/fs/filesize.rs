//! Human-readable file-size parsing and formatting.

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

/// Parses a human-readable file-size string into a raw byte count.
///
/// Accepted units: `K`, `KB`, `KiB`, `M`, `MB`, `MiB`, `G`, `GB`, `GiB`,
/// `T`, `TB`, `TiB`, `P`, `PB`, `PiB` (case-insensitive).
/// All units use base-1024 (binary) multipliers.
/// A bare integer with no unit is treated as bytes.
pub fn parse_size(size_str: &str) -> Option<u64> {
    let s = size_str.trim();
    if s.is_empty() {
        return None;
    }

    let unit_start = s
        .find(|c: char| !(c.is_ascii_digit() || c.is_ascii_whitespace()))
        .unwrap_or(s.len());
    let (val_str, unit) = s.split_at(unit_start);
    let val = val_str.trim().parse::<u64>().ok()?;
    let multiplier = match unit.trim() {
        "" | "B" | "b" => 1,
        u if u.eq_ignore_ascii_case("K")
            || u.eq_ignore_ascii_case("KB")
            || u.eq_ignore_ascii_case("KiB") =>
        {
            1024
        }
        u if u.eq_ignore_ascii_case("M")
            || u.eq_ignore_ascii_case("MB")
            || u.eq_ignore_ascii_case("MiB") =>
        {
            1024u64.pow(2)
        }
        u if u.eq_ignore_ascii_case("G")
            || u.eq_ignore_ascii_case("GB")
            || u.eq_ignore_ascii_case("GiB") =>
        {
            1024u64.pow(3)
        }
        u if u.eq_ignore_ascii_case("T")
            || u.eq_ignore_ascii_case("TB")
            || u.eq_ignore_ascii_case("TiB") =>
        {
            1024u64.pow(4)
        }
        u if u.eq_ignore_ascii_case("P")
            || u.eq_ignore_ascii_case("PB")
            || u.eq_ignore_ascii_case("PiB") =>
        {
            1024u64.pow(5)
        }
        _ => return None,
    };

    val.checked_mul(multiplier)
}

/// Formats a byte count as a human-readable string using binary units.
#[cfg(feature = "alloc")]
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
        assert_eq!(parse_size("512"), Some(512));
        assert_eq!(parse_size("1024B"), Some(1024));
        assert_eq!(parse_size("  1 M  "), Some(1024 * 1024));
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
