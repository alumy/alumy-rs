/// Macro to build a path in the format: `dir/name/name.suffix`
///
/// This macro uses `concat!` to produce a `&'static str`, meaning all arguments
/// must be string literals or macros that expand to string literals (like `env!`).
#[macro_export]
macro_rules! build_path {
    ($dir:expr, $name:expr, $suffix:expr) => {
        concat!($dir, "/", $name, "/", $name, $suffix)
    };
}

/// Returns the configuration file path for the application.
pub fn config_file() -> &'static str {
    build_path!("/etc", crate::crate_name!(), ".conf")
}

/// Returns the log file path for the application.
pub fn log_file() -> &'static str {
    build_path!("/var/log", crate::crate_name!(), ".log")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        assert_eq!(config_file(), concat!("/etc/alumy/alumy.conf"));
        assert_eq!(log_file(), concat!("/var/log/alumy/alumy.log"));
    }

    #[test]
    fn test_macro() {
        let path = build_path!("/tmp", "test", ".txt");
        assert_eq!(path, "/tmp/test/test.txt");
    }
}
