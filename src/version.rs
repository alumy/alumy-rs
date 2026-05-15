//! Crate metadata and version information.
//!
//! Provides compile-time macros and runtime functions for accessing
//! the crate name and version string embedded by Cargo.

/// Returns the version string of the calling crate, resolved at compile time.
///
/// Expands to the value of the `CARGO_PKG_VERSION` environment variable.
///
/// # Examples
///
/// ```
/// let v = alumy::crate_version!();
/// assert!(!v.is_empty());
/// ```
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Returns the name of the calling crate, resolved at compile time.
///
/// Expands to the value of the `CARGO_PKG_NAME` environment variable.
///
/// # Examples
///
/// ```
/// let name = alumy::crate_name!();
/// assert!(!name.is_empty());
/// ```
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Returns the version string of the `alumy` crate at runtime.
///
/// This is a thin wrapper around [`crate_version!`].
///
/// # Examples
///
/// ```
/// let v = alumy::version::version();
/// assert_eq!(v, env!("CARGO_PKG_VERSION"));
/// ```
pub fn version() -> &'static str {
    crate_version!()
}

/// Returns the name of the `alumy` crate at runtime.
///
/// This is a thin wrapper around [`crate_name!`].
///
/// # Examples
///
/// ```
/// let n = alumy::version::name();
/// assert_eq!(n, "alumy");
/// ```
pub fn name() -> &'static str {
    crate_name!()
}

/// Returns a greeting string that includes the crate name and version.
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
