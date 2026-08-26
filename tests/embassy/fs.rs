use alumy::embassy::fs;

#[test]
fn re_exports_fs_filesize() {
    assert_eq!(fs::filesize::parse_size("16KiB"), Some(16384));
}
