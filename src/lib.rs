#![cfg_attr(not(feature = "linux"), no_std)]
#![warn(missing_docs)]
//! # alumy
//!
//! A compact cross-platform Rust SDK facade for Linux and MCU development.
//!
//! `alumy` always exports no_std-friendly helpers. Platform integrations are
//! selected through concise features: `linux`, `bare`, `freertos`, and
//! `embassy`.

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use crate::core::fs;
#[doc(inline)]
pub use crate::core::version;

#[cfg(feature = "bare")]
pub mod bare {
    //! Bare-metal MCU layer for Alumy.
    //!
    //! This module is for firmware without an RTOS. It stays no_std-compatible
    //! and re-exports the shared core helpers.

    #[doc(no_inline)]
    pub use crate::{build_path, crate_name, crate_version, fs, version};
}

#[cfg(feature = "embassy")]
pub mod embassy {
    //! Embassy MCU layer for Alumy.
    //!
    //! This module is the home for Embassy-specific async adapters. It stays
    //! no_std-compatible and re-exports the shared core helpers.

    #[doc(no_inline)]
    pub use crate::{build_path, crate_name, crate_version, fs, version};
}

#[cfg(feature = "freertos")]
pub mod freertos {
    //! FreeRTOS MCU layer for Alumy.
    //!
    //! This module is the home for FreeRTOS-specific adapters. It stays
    //! no_std-compatible and re-exports the shared core helpers.

    #[doc(no_inline)]
    pub use crate::{build_path, crate_name, crate_version, fs, version};
}

#[cfg(feature = "linux")]
pub mod linux;

mod core;

#[cfg(feature = "linux")]
#[doc(inline)]
pub use linux::{log, sys, LogConfig};

#[cfg(feature = "linux")]
#[doc(no_inline)]
pub use linux::{debug, error, info, trace, warn, Level};
