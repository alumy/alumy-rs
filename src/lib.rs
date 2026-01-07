//! # alumy
//!
//! A Rust library.
//!
//! ## Example
//!
//! ```rust
//! use alumy;
//!
//! // Your example code here
//! ```

/// Returns a greeting message.
///
/// # Examples
///
/// ```
/// let greeting = alumy::hello();
/// assert_eq!(greeting, "Hello from alumy!");
/// ```
pub fn hello() -> &'static str {
    "Hello from alumy!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from alumy!");
    }
}

