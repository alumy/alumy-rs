use alumy::embassy::build_path;

#[test]
fn re_exports_build_path() {
    const PATH: &str = build_path!("/etc", "alumy", ".conf");
    assert_eq!(PATH, "/etc/alumy/alumy.conf");
}
