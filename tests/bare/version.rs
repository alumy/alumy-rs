use alumy::bare::{crate_version, version};

#[test]
fn re_exports_version_functions() {
    assert_eq!(version::name(), "alumy");
    assert_eq!(version::version(), crate_version!());
}
