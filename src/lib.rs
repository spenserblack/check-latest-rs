//! Depending on the features you have enabled/disabled, you should view the
//! documentation for `blocking` and/or `async` for basic usage.
//!
//! # Features
//! ## `blocking`
//!
//! This feature is enabled by default.
//!
//! Provides the basic usage.
//!
//! ## `async`
//!
//! Allows you to asynchronously check for available versions.
//! If enabled, it will provide async versions of the macros, which can be used
//! with `<macro_name>_async!` For example, `max_version_async!`.
//!
//! ```toml
//! [dependencies.check-latest]
//! default-features = false # If you want async, you probably don't want blocking
//! features = ["async"]
//! ```
pub use error::*;
use semver::Version;

#[derive(Debug)]
pub struct Versions {
    /// The maximum version.
    pub max_version: Version,
    /// The newest version. Not necessarily the maximum version.
    pub newest_version: Version,
}

/// Check for version updates with asynchronous requests.
#[cfg(feature = "async")]
pub mod r#async;

/// Check for version updates with blocking requests.
#[cfg(feature = "blocking")]
pub mod blocking;

mod error;

#[doc(hidden)]
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! user_agent {
    () => {
        concat!($crate::crate_name!(), "/", $crate::crate_version!())
    };
}
