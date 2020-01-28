//! Enabled with the `async` feature
//!
//! ```rust,no_run
//! # async fn run() {
//! use check_latest::*;
//!
//! if let Ok(Some(version)) = max_version_async!().await {
//!     println!("We've released a new version: {}!", version);
//! }
//! # }
//! ```

use crate::{Result, Versions};
pub use max::*;
pub use newest::*;
use semver::Version;

/// *__NOTE__ You probably want to use `versions_async!`*
///
/// `crate_name`: The crate that the version should be checked for.
///
/// `user_agent`: without a proper User-Agent, the request to the [Crates.io] API
/// will result in the following response, which we won't be able to parse into
/// crate versions.
///
/// ```text
/// We require that all requests include a `User-Agent` header.  To allow us to determine the impact your bot has on our service, we ask that your user agent actually identify your bot, and not just report the HTTP client library you're using.  Including contact information will also reduce the chance that we will need to take action against your bot.
///
/// Bad:
///   User-Agent: <badge user agent that you used>
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
/// use check_latest::r#async::get_versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = get_versions("my-awesome-crate-bin", "my-awesome-crate-bin/1.0.0").await {
///     if versions.max_version > current_version {
///         println!("Go get the new version!");
///     }
/// }
/// # }
/// ```
///
/// [Crates.io]: https://crates.io/
pub async fn get_versions(crate_name: &str, user_agent: &str) -> Result<Versions> {
    let url = format!(
        "https://crates.io/api/v1/crates/{crate_name}",
        crate_name = crate_name,
    );
    let response: serde_json::Value = reqwest::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .map_err(|_| "Couldn't build client")?
        .get(&url)
        .send()
        .await
        .map_err(|_| "Couldn't request crate info")?
        .json()
        .await
        .map_err(|_| "Couldn't parse response to JSON")?;

    let crate_data = response.get("crate").ok_or("Unexpected JSON format")?;
    let max_version = crate_data
        .get("max_version")
        .ok_or("Unexpected JSON format")?
        .as_str()
        .ok_or("Couldn't parse max version as str")?;
    let newest_version = crate_data
        .get("newest_version")
        .ok_or("Unexpected JSON format")?
        .as_str()
        .ok_or("Couldn't parse newest version as str")?;

    let max_version = Version::parse(max_version).map_err(|_| "Couldn't parse max version")?;
    let newest_version = Version::parse(newest_version)
        .map_err(|_| "Couldn't parse newest version")?;
    let versions = Versions {
        max_version,
        newest_version,
    };
    Ok(versions)
}

async fn get_version_list(crate_name: &str, user_agent: &str) -> Result<Vec<Version>> {
    let url = format!(
        "https://crates.io/api/v1/crates/{crate_name}",
        crate_name = crate_name,
    );
    let response: serde_json::Value = reqwest::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .map_err(|_| "Couldn't build client")?
        .get(&url)
        .send()
        .await
        .map_err(|_| "Couldn't request crate info")?
        .json()
        .await
        .map_err(|_| "Couldn't parse response to JSON")?;
    let versions = response
        .get("versions")
        .ok_or("Version list not found")?
        .as_array()
        .ok_or("Couldn't parse version list as array")?;
    let versions = versions
        .iter()
        .filter_map(|v| v.get("num"))
        .filter_map(|v| v.as_str())
        .map(|v| Version::parse(v))
        .filter_map(|v| v.ok())
        .collect();
    Ok(versions)
}

/// Asynchronous version of `versions!`. View the documentation of `versions!`
/// for full usage.
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::versions_async;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = versions_async!().await {
///     if versions.max_version > current_version {
///         println!("Go get a new version!");
///     }
/// }
/// # }
/// ```
#[macro_export]
macro_rules! versions_async {
    () => {
        $crate::versions_async!(
            crate_name = $crate::crate_name!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::r#async::get_versions($crate_name, $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::versions_async!(
            crate_name = $crate_name,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr $(,)?) => {
        $crate::versions_async!(crate_name = $crate_name, user_agent = $crate::user_agent!())
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::versions_async!(crate_name = $crate::crate_name!(), user_agent = $user_agent)
    };
}

mod max;
mod newest;
