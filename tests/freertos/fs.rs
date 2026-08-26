use alumy::freertos::fs;

#[test]
fn re_exports_fs_filesize() {
    assert_eq!(fs::filesize::parse_size("8KiB"), Some(8192));
}
