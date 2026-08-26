use alumy::build_path;

#[test]
fn build_path_at_compile_time() {
    const PATH: &str = build_path!("/etc", "alumy", ".conf");
    assert_eq!(PATH, "/etc/alumy/alumy.conf");
}
