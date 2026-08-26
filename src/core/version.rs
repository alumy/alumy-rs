//! Crate metadata and version information.
//!
//! Provides compile-time macros and runtime functions for accessing crate
//! metadata embedded by Cargo.

/// Returns the version string of the calling crate, resolved at compile time.
///
/// Expands to the value of the `CARGO_PKG_VERSION` environment variable.
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Returns the name of the calling crate, resolved at compile time.
///
/// Expands to the value of the `CARGO_PKG_NAME` environment variable.
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Returns the version string of the `alumy` crate at runtime.
pub fn version() -> &'static str {
    crate_version!()
}

/// Returns the name of the `alumy` crate at runtime.
pub fn name() -> &'static str {
    crate_name!()
}

/// Returns a greeting string that includes the crate name and version.
#[cfg(feature = "alloc")]
pub fn hello() -> alloc::string::String {
    alloc::format!("Hello from {} {}", name(), version())
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
