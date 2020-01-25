use semver::Version;
pub use max::*;
pub use newest::*;

#[derive(Debug)]
pub struct Versions {
    /// The maximum version.
    pub max_version: Version,
    /// The newest version. Not necessarily that maximum version.
    pub newest_version: Version,
}

pub type Error = String;

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
/// use check_latest::get_versions;
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
pub fn get_versions(crate_name: &str, user_agent: &str) -> Result<Versions, Error> {
    let url = format!("https://crates.io/api/v1/crates/{crate_name}", crate_name = crate_name);
    let response: serde_json::Value = reqwest::blocking::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .map_err(|_| "Couldn't build client")?
        .get(&url)
        .send()
        .map_err(|_| "Couldn't request crate info")?
        .json()
        .map_err(|_| "Couldn't parse response to JSON")?;

    let crate_data = response
        .get("crate")
        .ok_or("Unexpected JSON format")?;
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
    let newest_version = Version::parse(newest_version).map_err(|_| "Couldn't parse newest version")?;
    let versions = Versions {
        max_version,
        newest_version,
    };
    Ok(versions)
}

fn get_version_list(crate_name: &str, user_agent: &str) -> Result<Vec<Version>, Error> {
    let url = format!("https://crates.io/api/v1/crates/{crate_name}", crate_name = crate_name);
    let response: serde_json::Value = reqwest::blocking::Client::builder()
        .user_agent(format!("{}/{}", crate_name, user_agent))
        .build()
        .map_err(|_| "Couldn't build client")?
        .get(&url)
        .send()
        .map_err(|_| "Couldn't request crate info")?
        .json()
        .map_err(|_| "Couldn't parse response to JSON")?;
    let versions = response
        .get("versions")
        .ok_or("Version list not found")?
        .as_array()
        .ok_or("Couldn't parse version list as array")?;
    let versions = versions.into_iter()
        .filter_map(|v| v.as_str())
        .map(|v| Version::parse(v))
        .filter_map(|v| v.ok())
        .collect();
    Ok(versions)
}

/// Gets the largest minor version available with the same major version.
///
/// *__NOTE__ You probably want to use `max_minor_version!`*
///
/// - `crate_name`: The crate that the version should be checked for.
/// - `version`: The version to be compared against.
/// - `user_agent`: without a proper User-Agent, the request to the [Crates.io]
/// API will result in the following response, which we won't be able to parse
/// into crate versions.
///
/// # Returns
///
/// If `version` is `x.y.z` and the max available minor version is `x.b.c`.
///
/// - `Ok(Some(version))` if `y.z` < `b.c`
/// - `Ok(None) if `y.z` >= `b.c`
/// - `Err(_)` if there was a failure to get and compare the versions
///
/// # Example
///
/// ```rust,no_run
/// use check_latest::get_max_minor_version;
///
/// let crate_name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", crate_name, version);
///
/// let result = get_max_minor_version(crate_name, version, &user_agent);
///
/// if let Ok(Some(higher_minor_version)) = result {
///     println!("A new minor version is available: {}", higher_minor_version);
/// }
/// ```
///
/// [Crates.io]: https://crates.io/
pub fn get_max_minor_version(crate_name: &str, version: &str, user_agent: &str) -> Result<Option<Version>, Error> {
    let versions = get_version_list(crate_name, user_agent)?;
    let current_version = Version::parse(version).map_err(|_| "Couldn't parse `version`")?;

    let max_minor_version = versions
        .into_iter()
        .filter(|v| v.major == current_version.major)
        .max();

    Ok(max_minor_version)
}

/// Gets the largest patch available with the same major and minor version.
///
/// *__NOTE__ You probably want to use `max_patch!`*
///
/// - `crate_name`: The crate that the version should be checked for.
/// - `version`: The version to be compared against.
/// - `user_agent`: without a proper User-Agent, the request to the [Crates.io]
/// API will result in the following response, which we won't be able to parse
/// into crate versions.
///
/// # Returns
///
/// If `version` is `x.y.z` and the max available patch is `x.y.c`.
///
/// - `Ok(Some(version))` if `z` < `c`
/// - `Ok(None) if `z` >= `c`
/// - `Err(_)` if there was a failure to get and compare the versions
///
/// # Example
///
/// ```rust,no_run
/// use check_latest::get_max_patch;
///
/// let crate_name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", crate_name, version);
///
/// let result = get_max_patch(crate_name, version, &user_agent);
///
/// if let Ok(Some(higher_patch)) = result {
///     println!("A new patch has been released: {}", higher_patch);
/// }
/// ```
///
/// [Crates.io]: https://crates.io/
pub fn get_max_patch(crate_name: &str, version: &str, user_agent: &str) -> Result<Option<Version>, Error> {
    let versions = get_version_list(crate_name, user_agent)?;
    let current_version = Version::parse(version).map_err(|_| "Couldn't parse `version`")?;

    let max_patch = versions
        .into_iter()
        .filter(|v| v.major == current_version.major)
        .filter(|v| v.minor == current_version.minor)
        .max();

    Ok(max_patch)
}

#[doc(hidden)]
#[macro_export]
macro_rules! crate_name {
    () => (env!("CARGO_PKG_NAME"));
}

#[doc(hidden)]
#[macro_export]
macro_rules! crate_version {
    () => (env!("CARGO_PKG_VERSION"));
}

#[doc(hidden)]
#[macro_export]
macro_rules! user_agent {
    () => (concat!($crate::crate_name!(), "/", $crate::crate_version!()));
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
        $crate::versions!(crate_name = $crate::crate_name!(), user_agent = $crate::user_agent!())
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::get_versions($crate_name, $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::get_versions($crate_name, $user_agent)
    };
    (crate_name = $crate_name:expr $(,)?) => {
        $crate::versions!(crate_name = $crate_name, user_agent = $crate::user_agent!())
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::versions!(crate_name = $crate::crate_name!(), user_agent = $user_agent)
    };
}

mod max;
mod newest;
