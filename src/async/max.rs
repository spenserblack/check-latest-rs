use super::*;
use anyhow::{Context, Result};
use semver::Version;

/// *__NOTE__ You probably want to use `max_version_async!`*
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
/// # async fn run() {
/// use check_latest::r#async::get_max_version;
///
/// let name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", name, version);
///
/// if let Ok(Some(version)) = get_max_version(name, version, &user_agent).await {
///     println!("Go get version {}!", version);
/// }
/// # }
/// ```
///
/// [Crates.io]: https://crates.io/
pub async fn get_max_version(
    crate_name: &str,
    current_crate_version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let versions = get_version_list(crate_name, user_agent)
        .await
        .context("Couldn't get versions list")?;
    let current_version = Version::parse(current_crate_version)
        .context("Couldn't parse current version")?;
    let max_version = versions
        .into_iter()
        .max()
        .filter(|v| v > &current_version);
    Ok(max_version)
}

/// Gets the largest minor version available with the same major version.
///
/// *__NOTE__ You probably want to use `max_minor_version_async!`*
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
/// # async fn run() {
/// use check_latest::r#async::get_max_minor_version;
///
/// let crate_name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", crate_name, version);
///
/// let result = get_max_minor_version(crate_name, version, &user_agent);
///
/// if let Ok(Some(higher_minor_version)) = result.await {
///     println!("A new minor version is available: {}", higher_minor_version);
/// }
/// # }
/// ```
///
/// [Crates.io]: https://crates.io/
pub async fn get_max_minor_version(
    crate_name: &str,
    version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let versions = get_version_list(crate_name, user_agent)
        .await
        .context("Couldn't get version list")?;
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
/// *__NOTE__ You probably want to use `max_patch_async!`*
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
/// # async fn run() {
/// use check_latest::r#async::get_max_patch;
///
/// let crate_name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", crate_name, version);
///
/// let result = get_max_patch(crate_name, version, &user_agent);
///
/// if let Ok(Some(higher_patch)) = result.await {
///     println!("A new patch has been released: {}", higher_patch);
/// }
/// # }
/// ```
///
/// [Crates.io]: https://crates.io/
pub async fn get_max_patch(
    crate_name: &str,
    version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let versions = get_version_list(crate_name, user_agent)
        .await
        .context("Couldn't get version list")?;
    let current_version = Version::parse(version)
        .context("Couldn't parse `version`")?;

    let max_patch = versions
        .into_iter()
        .filter(|v| v.major == current_version.major)
        .filter(|v| v.minor == current_version.minor)
        .max();

    Ok(max_patch)
}

/// Asynchronous version of `max_version!` View the documentation of
/// `max_version!` for more details.
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::max_version_async;
///
/// if let Ok(Some(version)) = max_version_async!().await {
///     println!("Go get version {}!", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! max_version_async {
    () => {
        $crate::max_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::r#async::get_max_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}

/// Asynchronous version of `max_minor_version!` View the documentation of
/// `max_minor_version!` for more information.
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::max_minor_version_async;
///
/// if let Ok(Some(version)) = max_minor_version_async!().await {
///     println!("Minor version has been updated to {}!", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! max_minor_version_async {
    () => {
        $crate::max_minor_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::r#async::get_max_minor_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_minor_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}

/// Asynchronous version of `max_patch!` View the documentation of
/// `max_patch!` for more information.
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::max_patch_async;
///
/// if let Ok(Some(version)) = max_patch_async!().await {
///     println!("A new patch has been released: {}!", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! max_patch_async {
    () => {
        $crate::max_patch_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::r#async::get_max_patch($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::max_patch_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}
