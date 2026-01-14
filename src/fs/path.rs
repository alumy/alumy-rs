/// Macro to build a path in the format: `dir/name/name.suffix`
#[allow(unused_macros)]
macro_rules! build_path {
    ($dir:expr, $name:expr, $suffix:expr) => {
        concat!($dir, "/", $name, "/", $name, $suffix)
    };
}

#[cfg(test)]
mod tests {
    use crate::crate_name;

    #[test]
    fn test_paths() {
        assert_eq!(build_path!("/etc", crate_name!(), ".conf"), concat!("/etc/alumy/alumy.conf"));
        assert_eq!(build_path!("/var/log", crate_name!(), ".log"), concat!("/var/log/alumy/alumy.log"));
    }

    #[test]
    fn test_macro() {
        let path = build_path!("/tmp", "test", ".txt");
        assert_eq!(path, "/tmp/test/test.txt");
    }
}
