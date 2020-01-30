//! Enabled with the `blocking` feature
//!
//! ```rust,no_run
//! use check_latest::*;
//!
//! if let Ok(Some(version)) = max_version!() {
//!     println!("We've released a new version: {}!", version);
//! }
//! ```

use anyhow::{Context, Result};
use crate::{build_url, CratesioResponse, Versions};
pub use max::*;
pub use newest::*;
use semver::Version;

/// *__NOTE__ You probably want to use `versions!`*
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
/// use check_latest::blocking::get_versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = get_versions("my-awesome-crate-bin", "my-awesome-crate-bin/1.0.0") {
///     if versions.max_version > current_version {
///         println!("Go get the new version!");
///     }
/// }
/// ```
///
/// [Crates.io]: https://crates.io/
pub fn get_versions(crate_name: &str, user_agent: &str) -> Result<Versions> {
    let url = build_url(crate_name);
    let response: CratesioResponse = reqwest::blocking::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .context("Couldn't build client")?
        .get(&url)
        .send()
        .context("Couldn't request crate info")?
        .json()
        .context("Couldn't read as JSON")?;

    Ok(response.versions)
}

fn get_version_list(crate_name: &str, user_agent: &str) -> Result<Vec<Version>> {
    let url = build_url(crate_name);
    let response: CratesioResponse = reqwest::blocking::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .context("Couldn't build client")?
        .get(&url)
        .send()
        .context("Couldn't request crate info")?
        .json()
        .context("Couldn't read as JSON")?;
    let versions = response.all_versions;
    let versions = versions
        .into_iter()
        .map(|v| v.version)
        .collect();
    Ok(versions)
}

/// Makes it easier to run `get_versions`.
///
/// `versions!()` will predict the `crate_name` and `user_agent`. `crate_name`
/// will default to package name in your `Cargo.toml` file.
/// `user_agent` will default to the name of your crate as defined in
/// `Cargo.toml` followed by a `/` and the version of your package as defined
/// in your `Cargo.toml` file (e.g. `my-crate-name/1.0.0`).
///
/// If you do not want these defaults to be used, you can set your own values.
/// See the examples below.
///
/// # Examples
///
/// ## Use Defaults
///
/// ```rust,no_run
/// use check_latest::versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = versions!() {
///     if versions.max_version > current_version {
///         println!("Go get a new version!");
///     }
/// }
/// ```
///
/// ## Set Crate Name
///
/// ```rust,no_run
/// use check_latest::versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = versions!(crate_name = "my-renamed-crate") {
///     if versions.max_version > current_version {
///         println!("Go get a new version!");
///     }
/// }
/// ```
///
/// ## Set User Agent
///
/// ```rust,no_run
/// use check_latest::versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// if let Ok(versions) = versions!(user_agent = "my extra detailed user agent") {
///     if versions.max_version > current_version {
///         println!("Go get a new version!");
///     }
/// }
/// ```
///
/// ## Set Both
///
/// ```rust,no_run
/// use check_latest::versions;
/// use semver::Version;
///
/// let current_version = Version::parse("1.0.0").unwrap();
///
/// let crate_name = "my-renamed-crate";
/// let user_agent = "my extra detailed user agent";
///
/// // This is reversible BTW
/// let versions = versions!(crate_name = crate_name, user_agent = user_agent);
///
/// if let Ok(versions) = versions {
///     if versions.max_version > current_version {
///         println!("Go get a new version!");
///     }
/// }
/// ```
#[macro_export]
macro_rules! versions {
    () => {
        $crate::versions!(
            crate_name = $crate::crate_name!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::blocking::get_versions($crate_name, $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::versions!(crate_name = $crate_name, user_agent = $user_agent)
    };
    (crate_name = $crate_name:expr $(,)?) => {
        $crate::versions!(
            crate_name = $crate_name,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::versions!(
            crate_name = $crate::crate_name!(),
            user_agent = $user_agent,
        )
    };
}

mod max;
mod newest;
