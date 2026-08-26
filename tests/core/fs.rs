use alumy::fs;

#[test]
fn parse_filesize() {
    assert_eq!(fs::filesize::parse_size("4KiB"), Some(4096));
}
