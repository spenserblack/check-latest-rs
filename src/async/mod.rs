//! Enabled with the `async` feature
//!
//! ```rust,no_run
//! # async fn run() {
//! use check_latest::*;
//!
//! if let Ok(Some(version)) = check_max_async!().await {
//!     println!("We've released a new version: {}!", version);
//! }
//! # }
//! ```

use anyhow::{Context, Result};
use crate::{build_url, Versions};


/// Checks if there is a version available that is greater than the current
/// version.
///
/// # Returns
///
/// Assume the current version is `a.b.c`, and we are looking at versions that
/// are `x.y.z`.
///
/// - `Ok(Some(version))` if `x.y.z > a.b.c` where `version = max x.y.z`
/// - `Ok(None)` if no version meets the rule `x.y.z > a.b.c`
/// - `Err(e)` if comparison could not be made
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::check_max_async;
///
/// if let Ok(Some(version)) = check_max_async!().await {
///     println!("A new version is available: {}", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! check_max_async {
    () => {
        async {
            $crate::new_versions_async!()
                .await
                .map(|versions| {
                    let max = versions.max_unyanked_version()?
                        .clone();
                    if max > $crate::crate_version!() {
                        Some(max)
                    } else {
                        None
                    }
                })
        }
    };
}
/// Checks if there is a higher minor version available with the same major
/// version
///
/// # Returns
///
/// Assume the current version is `a.b.c`, and we are looking at versions that
/// are `a.y.z`.
///
/// - `Ok(Some(version))` if `a.y.z > a.b.c` where `version =  max a.b.z`
/// - `Ok(None)` if no version meets the rule `a.y.z > a.b.c`
/// - `Err(e)` if comparison could not be made
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::check_minor_async;
///
/// if let Ok(Some(version)) = check_minor_async!().await {
///     println!("A new version is available: {}", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! check_minor_async {
    () => {
        async {
            $crate::new_versions_async!()
                .await
                .and_then(|versions| {
                    let major_version = $crate::crate_major_version!().parse()?;
                    let max = versions.max_unyanked_minor_version(major_version);
                    let max = max.cloned();
                    let max = max.filter(|max| max > $crate::crate_version!());
                    Ok(max)
                })
        }
    };
}

/// Checks if there is a higher patch available, within the same major.minor
/// version.
///
/// # Returns
///
/// Assume the current version is `a.b.c`, and we are looking at versions that
/// are `a.b.z`.
///
/// - `Ok(Some(version))` if `a.b.z > a.b.c`, where `version = max a.b.z`
/// - `Ok(None)` if no version meets the rule `a.b.z > a.b.c`
/// - `Err(e)` if comparison could not be made
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::check_patch_async;
///
/// if let Ok(Some(version)) = check_patch_async!().await {
///     println!("We've implemented one or more bug fixes in {}", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! check_patch_async {
    () => {
        async {
            $crate::new_versions_async!()
                .await
                .and_then(|versions| {
                    let major_version = $crate::crate_major_version!().parse()?;
                    let minor_version = $crate::crate_minor_version!().parse()?;
                    let max = versions.max_unyanked_patch(major_version, minor_version);
                    let max = max.cloned();
                    let max = max.filter(|max| max > $crate::crate_version!());
                    Ok(max)
                })
        }
    };
}

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
    /// [Crates.io]: https://crates.io/
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

/// Helper for creating a new `Versions`.
///
/// Will assume the correct `crate_name` and `user_agent` based on the contents
/// of *your* `Cargo.toml`, but these values can be overridden.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::new_versions_async;
///
/// let versions = new_versions_async!().await;
/// # }
/// ```
///
/// ## Overriding Default Values
///
/// *__NOTE__ Overriding both defaults is no different than just using
/// `Versions::new`. You will probably want to override only one field, if any,
/// if using this macro.
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::new_versions_async;
///
/// let versions = new_versions_async!(
///     crate_name = "renamed-crate",
///     user_agent = "my-user-agent",
/// ).await;
/// # }
/// ```
#[macro_export]
macro_rules! new_versions_async {
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::Versions::async_new($crate_name, $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::new_versions_async!(
            crate_name = $crate_name,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr) => {
        $crate::new_versions_async!(
            crate_name = $crate_name,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr) => {
        $crate::new_versions_async!(
            crate_name = $crate::crate_name!(),
            user_agent = $user_agent,
        )
    };
    () => {
        $crate::new_versions_async!(
            crate_name = $crate::crate_name!(),
            user_agent = $crate::user_agent!(),
        )
    };
}
