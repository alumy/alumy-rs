/// Parses a string representation of file size (e.g., "10M", "1G", "512K", "2KiB", "1KB") into bytes.
/// Supported units: K, KB, KiB, M, MB, MiB, G, GB, GiB, T, TB, TiB, P, PB, PiB (case-insensitive).
/// All units are treated as binary (base 1024).
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
}
