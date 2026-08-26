use alumy::{crate_version, version};

#[test]
fn expose_version_functions() {
    assert_eq!(version::name(), "alumy");
    assert_eq!(version::version(), crate_version!());
}
