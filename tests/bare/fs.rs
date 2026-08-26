use alumy::bare::fs;

#[test]
fn re_exports_fs_filesize() {
    assert_eq!(fs::filesize::parse_size("4KiB"), Some(4096));
}
