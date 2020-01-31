use super::*;
use anyhow::{Context, Result};
use semver::Version;

/// *__NOTE__ You probably want to use `newest_version_async!`*
///
/// Compares the current crate version to the newest version available on
/// [Crates.io].
///
/// Please note that the newest version is *not* always the maximum version.
/// A patch may have been released for an old major version that is still being
/// maintained, for example.
///
/// # Returns
/// - `Ok(Some(version))` if the current version < newest version
/// - `Ok(None) if current version >= newest version
/// - `Err(_)` if there was a failure to get and compare the versions
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::r#async::get_newest_version;
///
/// let name = "my-awesome-crate-bin";
/// let version = "1.0.0";
/// let user_agent = format!("{}/{}", name, version);
///
/// if let Ok(Some(version)) = get_newest_version(name, version, &user_agent).await {
///     println!("Go get version {}!", version);
/// }
/// # }
/// ```
///
/// [Crates.io]: https://crates.io/
#[deprecated(since = "1", note = "Please use Versions struct")]
pub async fn get_newest_version(
    crate_name: &str,
    current_crate_version: &str,
    user_agent: &str,
) -> Result<Option<Version>> {
    let current_version = Version::parse(current_crate_version)
        .context("Couldn't parse current version")?;
    let newest_version = get_versions(crate_name, user_agent)
        .await
        .context("Couldn't get newest version")?
        .newest_version;
    let newest_version = if current_version < newest_version {
        Some(newest_version)
    } else {
        None
    };
    Ok(newest_version)
}

/// Asynchronous version of `newest_version!`.
/// View the documentation of `newest_versions!` for full usage.
///
/// # Example
///
/// ```rust,no_run
/// # async fn run() {
/// use check_latest::newest_version_async;
///
/// if let Ok(Some(version)) = newest_version_async!().await {
///     println!("Go get version {}!", version);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! newest_version_async {
    () => {
        $crate::newest_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::r#async::get_newest_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $user_agent,
        )
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate_name,
            version = $crate::crate_version!(),
            user_agent = $crate::user_agent!(),
        )
    };
    (version = $version:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate::crate_name!(),
            version = $version,
            user_agent = $crate::user_agent!(),
        )
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::newest_version_async!(
            crate_name = $crate::crate_name!(),
            version = $crate::crate_version!(),
            user_agent = $user_agent,
        )
    };
}
