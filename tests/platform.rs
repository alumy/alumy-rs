//! Cross-platform integration-test entry point.

#[cfg(feature = "bare")]
#[path = "bare/mod.rs"]
mod bare;

#[cfg(feature = "freertos")]
#[path = "freertos/mod.rs"]
mod freertos;

#[cfg(feature = "embassy")]
#[path = "embassy/mod.rs"]
mod embassy;

#[cfg(feature = "linux")]
#[path = "linux/mod.rs"]
mod linux;
