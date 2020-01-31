use super::*;
use anyhow::{Context, Result};
use semver::Version;

/// *__NOTE__ You probably want to use `max_version!`*
///
/// Compares the current crate version to the maximum version available on
/// [Crates.io].
///
/// # Returns
/// - `Ok(Some(version))` if the current version < max version
/// - `Ok(None) if current version >= max version
/// - `Err(_)` if there was a failure to get and compare the versions
///
/// # Example
///
/// ```rust,no_run
/// use check_latest::blocking::get_max_version;
///
/// let name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", name, version);
///
/// if let Ok(Some(version)) = get_max_version(name, version, &user_agent) {
///     println!("Go get version {}!", version);
/// }
/// ```
///
/// [Crates.io]: https://crates.io/
#[deprecated(since = "1", note = "Please use Versions struct")]
#[allow(deprecated)]
pub fn get_max_version(
    crate_name: &str,
    current_crate_version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let current_version = Version::parse(current_crate_version)
        .context("Couldn't parse current version")?;
    let max_version = get_versions(crate_name, user_agent)
        .context("Couldn't get max version")?
        .max_version;
    let max_version = if current_version < max_version {
        Some(max_version)
    } else {
        None
    };
    Ok(max_version)
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
/// use check_latest::blocking::get_max_minor_version;
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
#[deprecated(since = "1", note = "Please use Versions struct")]
pub fn get_max_minor_version(
    crate_name: &str,
    version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let versions = get_version_list(crate_name, user_agent)
        .context("Couldn't get versions list")?;
    let current_version = Version::parse(version)
        .context("Couldn't parse `version`")?;

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
/// use check_latest::blocking::get_max_patch;
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
#[deprecated(since = "1", note = "Please use Versions struct")]
pub fn get_max_patch(
    crate_name: &str,
    version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let versions = get_version_list(crate_name, user_agent)
        .context("Couldn't get versions list")?;
    let current_version = Version::parse(version)
        .context("Couldn't parse `version`")?;

    let max_patch = versions
        .into_iter()
        .filter(|v| v.major == current_version.major)
        .filter(|v| v.minor == current_version.minor)
        .max();

    Ok(max_patch)
}

/// Makes it easier to run `get_max_version`.
///
/// `max_version!()` will predict the `crate_name`, `current_crate_version`, and
/// `user_agent`.
/// `crate_name` will default to package name in your `Cargo.toml` file.
/// `current_crate_version` will default to the package version in your
/// `Cargo.toml` file.
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
/// use check_latest::max_version;
///
/// if let Ok(Some(version)) = max_version!() {
///     println!("Go get version {}!", version);
/// }
/// ```
///
/// ## Set Crate Name
///
/// ```rust,no_run
/// use check_latest::max_version;
///
/// let name = "my-renamed-crate";
///
/// if let Ok(Some(version)) = max_version!(crate_name = name) {
///     println!("Go get version {}!", version);
/// }
/// ```
///
/// ## Set Crate Version to Compare
///
/// ```rust,no_run
/// use check_latest::max_version;
///
/// let current_version = "1.2.3";
///
///
/// if let Ok(Some(version)) = max_version!(version = current_version) {
///     println!("Go get version {}!", version);
/// }
/// ```
///
/// ## Set User Agent
///
/// ```rust,no_run
/// use check_latest::max_version;
///
/// let user_agent = "My extra detailed user agent";
///
/// if let Ok(Some(version)) = max_version!(user_agent = user_agent) {
///     println!("Go get version {}!", version);
/// }
/// ```
///
/// ## Set All 3
///
/// ```rust,no_run
/// use check_latest::max_version;
///
/// let crate_name = "my-renamed-crate";
/// let current_version = "1.2.3";
/// let user_agent = "My extra detailed user agent";
///
/// let max_version = max_version!(
///     // These can be shuffled BTW
///     crate_name = crate_name,
///     version = current_version,
///     user_agent = user_agent,
/// );
///
/// if let Ok(Some(version)) = max_version {
///     println!("Go get version {}!", version);
/// }
/// ```
#[macro_export]
macro_rules! max_version {
    () => {
        $crate::max_version!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::blocking::get_max_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version, user_agent = $user_agent)
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version, user_agent = $user_agent)
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version, user_agent = $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version, user_agent = $user_agent)
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version, user_agent = $user_agent)
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_version!(version = $version, user_agent = $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, user_agent = $user_agent)
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version!(crate_name = $crate_name, version = $version)
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}

/// Makes it easier to run `get_max_minor_version`.
///
/// `max_minor_version!()` will predict the `crate_name`,
/// `current_crate_version`, and `user_agent`.
/// `crate_name` will default to package name in your `Cargo.toml` file.
/// `current_crate_version` will default to the package version in your
/// `Cargo.toml` file.
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
/// use check_latest::max_minor_version;
///
/// if let Ok(Some(version)) = max_minor_version!() {
///     println!("Minor version has been updated to {}!", version);
/// }
/// ```
///
/// ## Set Crate Name
///
/// ```rust,no_run
/// use check_latest::max_minor_version;
///
/// let name = "my-renamed-crate";
///
/// if let Ok(Some(version)) = max_minor_version!(crate_name = name) {
///     println!("Minor version has been updated to {}!", version);
/// }
/// ```
///
/// ## Set Crate Version to Compare
///
/// ```rust,no_run
/// use check_latest::max_minor_version;
///
/// let current_version = "1.2.3";
///
///
/// if let Ok(Some(version)) = max_minor_version!(version = current_version) {
///     println!("Minor version has been updated to {}!", version);
/// }
/// ```
///
/// ## Set User Agent
///
/// ```rust,no_run
/// use check_latest::max_minor_version;
///
/// let user_agent = "My extra detailed user agent";
///
/// if let Ok(Some(version)) = max_minor_version!(user_agent = user_agent) {
///     println!("Minor version has been updated to {}!", version);
/// }
/// ```
///
/// ## Set All 3
///
/// ```rust,no_run
/// use check_latest::max_minor_version;
///
/// let crate_name = "my-renamed-crate";
/// let current_version = "1.2.3";
/// let user_agent = "My extra detailed user agent";
///
/// let max_minor_version = max_minor_version!(
///     // These can be shuffled BTW
///     crate_name = crate_name,
///     version = current_version,
///     user_agent = user_agent,
/// );
///
/// if let Ok(Some(version)) = max_minor_version {
///     println!("Minor version has been updated to {}!", version);
/// }
/// ```
#[macro_export]
macro_rules! max_minor_version {
    () => {
        $crate::max_minor_version!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::blocking::get_max_minor_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}

/// Makes it easier to run `get_max_patch`.
///
/// `max_patch!()` will predict the `crate_name`,
/// `current_crate_version`, and `user_agent`.
/// `crate_name` will default to package name in your `Cargo.toml` file.
/// `current_crate_version` will default to the package version in your
/// `Cargo.toml` file.
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
/// use check_latest::max_patch;
///
/// if let Ok(Some(version)) = max_patch!() {
///     println!("A new patch has been released: {}!", version);
/// }
/// ```
///
/// ## Set Crate Name
///
/// ```rust,no_run
/// use check_latest::max_patch;
///
/// let name = "my-renamed-crate";
///
/// if let Ok(Some(version)) = max_patch!(crate_name = name) {
///     println!("A new patch has been released: {}!", version);
/// }
/// ```
///
/// ## Set Crate Version to Compare
///
/// ```rust,no_run
/// use check_latest::max_patch;
///
/// let current_version = "1.2.3";
///
///
/// if let Ok(Some(version)) = max_patch!(version = current_version) {
///     println!("A new patch has been released: {}!", version);
/// }
/// ```
///
/// ## Set User Agent
///
/// ```rust,no_run
/// use check_latest::max_patch;
///
/// let user_agent = "My extra detailed user agent";
///
/// if let Ok(Some(version)) = max_patch!(user_agent = user_agent) {
///     println!("A new patch has been released: {}!", version);
/// }
/// ```
///
/// ## Set All 3
///
/// ```rust,no_run
/// use check_latest::max_patch;
///
/// let crate_name = "my-renamed-crate";
/// let current_version = "1.2.3";
/// let user_agent = "My extra detailed user agent";
///
/// let max_patch = max_patch!(
///     // These can be shuffled BTW
///     crate_name = crate_name,
///     version = current_version,
///     user_agent = user_agent,
/// );
///
/// if let Ok(Some(version)) = max_patch {
///     println!("A new patch has been released: {}!", version);
/// }
/// ```
#[macro_export]
macro_rules! max_patch {
    () => {
        $crate::max_patch!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::blocking::get_max_patch($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}
