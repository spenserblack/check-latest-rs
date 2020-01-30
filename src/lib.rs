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
use anyhow::{Context, Result};
use semver::Version as SemVersion;
use serde::Deserialize;
use std::fmt::{self, Display};
use time::OffsetDateTime;

/// A collection of `Version`s
#[derive(Debug, Deserialize)]
pub struct Versions {
    versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(rename = "num")]
    version: SemVersion,
    /// If this version was yanked
    pub yanked: bool,
    /// When this version was published
    pub created_at: OffsetDateTime,
}

#[cfg(feature = "async")]
impl Versions {
    /// - `crate_name`: The crate that the version should be checked for.
    /// - `user_agent`: without a proper User-Agent, the request to the
    ///   [Crates.io] API will result in the response below, which we won't
    ///   be able to parse into crate versions.
    ///
    /// # Example Response from Bad User Agent
    ///
    /// ```text
    /// We require that all requests include a `User-Agent` header.  To allow us to determine the impact your bot has on our service, we ask that your user agent actually identify your bot, and not just report the HTTP client library you're using.  Including contact information will also reduce the chance that we will need to take action against your bot.
    ///
    /// Bad:
    ///   User-Agent: <bad user agent that you used>
    ///
    /// Better:
    ///   User-Agent: my_crawler
    ///
    /// Best:
    ///   User-Agent: my_crawler (my_crawler.com/info)
    ///   User-Agent: my_crawler (help@my_crawler.com)
    ///
    /// If you believe you've received this message in error, please email help@crates.io and include the request id {}.
    /// ```
    ///
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use check_latest::Versions;
    ///
    /// if let Ok(versions) = Versions::async_new("my-awesome-crate-bin", "my-awesome-crate-bin/1.0.0").await {
    ///     /* Do your stuff */
    /// }
    /// # }
    /// ```
    ///
    pub async fn async_new(crate_name: &str, user_agent: &str) -> Result<Versions> {
        let url = build_url(crate_name);
        let response: Versions = reqwest::Client::builder()
            .user_agent(user_agent)
            .build()
            .context("Couldn't build client")?
            .get(&url)
            .send()
            .await
            .context("Couldn't request crate info")?
            .json()
            .await
            .context("Couldn't read as JSON")?;
        Ok(response)
    }
}

#[cfg(feature = "blocking")]
impl Versions {
    /// - `crate_name`: The crate that the version should be checked for.
    /// - `user_agent`: without a proper User-Agent, the request to the
    ///   [Crates.io] API will result in the response below, which we won't
    ///   be able to parse into crate versions.
    ///
    /// # Example Response from Bad User Agent
    ///
    /// ```text
    /// We require that all requests include a `User-Agent` header.  To allow us to determine the impact your bot has on our service, we ask that your user agent actually identify your bot, and not just report the HTTP client library you're using.  Including contact information will also reduce the chance that we will need to take action against your bot.
    ///
    /// Bad:
    ///   User-Agent: <bad user agent that you used>
    ///
    /// Better:
    ///   User-Agent: my_crawler
    ///
    /// Best:
    ///   User-Agent: my_crawler (my_crawler.com/info)
    ///   User-Agent: my_crawler (help@my_crawler.com)
    ///
    /// If you believe you've received this message in error, please email help@crates.io and include the request id {}.
    /// ```
    ///
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// if let Ok(versions) = Versions::new("my-awesome-crate-bin", "my-awesome-crate-bin/1.0.0") {
    ///     /* Do your stuff */
    /// }
    /// ```
    ///
    pub fn new(crate_name: &str, user_agent: &str) -> Result<Versions> {
        let url = build_url(crate_name);
        let response: Versions = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()
            .context("Couldn't build client")?
            .get(&url)
            .send()
            .context("Couldn't request crate info")?
            .json()
            .context("Couldn't read as JSON")?;
        Ok(response)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)?;
        if self.yanked {
            write!(f, " (yanked)")
        } else {
            Ok(())
        }
    }
}

impl From<Version> for SemVersion {
    fn from(v: Version) -> SemVersion {
        v.version
    }
}

fn build_url(crate_name: &str) -> String {
    format!(
        "https://crates.io/api/v1/crates/{crate_name}",
        crate_name = crate_name,
    )
}

/// Check for version updates with asynchronous requests.
// #[cfg(feature = "async")]
// pub mod r#async;

/// Check for version updates with blocking requests.
// #[cfg(feature = "blocking")]
// pub mod blocking;

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
