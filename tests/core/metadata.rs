use alumy::{crate_name, crate_version};

#[test]
fn expose_crate_metadata_macros() {
    assert_eq!(crate_name!(), "alumy");
    assert!(!crate_version!().is_empty());
}
