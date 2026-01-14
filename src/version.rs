/// Macro to get the current version of the crate at compile time.
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Macro to get the name of the crate at compile time.
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Returns the current version of the crate.
pub fn version() -> &'static str {
    crate_version!()
}

/// Returns the name of the crate.
pub fn name() -> &'static str {
    crate_name!()
}

/// Returns a greeting message including the crate name and version.
///
/// # Examples
///
/// ```
/// let greeting = alumy::version::hello();
/// assert!(greeting.contains("alumy"));
/// ```
pub fn hello() -> String {
    format!("Hello from {} {}", name(), version())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(crate_version!(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_name() {
        assert_eq!(name(), "alumy");
        assert_eq!(crate_name!(), "alumy");
    }

    #[test]
    fn test_hello() {
        let greeting = hello();
        assert!(greeting.contains("alumy"));
        assert!(greeting.contains(env!("CARGO_PKG_VERSION")));
    }
}
