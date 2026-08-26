use alumy::bare::{crate_name, crate_version};

#[test]
fn re_exports_crate_metadata_macros() {
    assert_eq!(crate_name!(), "alumy");
    assert!(!crate_version!().is_empty());
}
