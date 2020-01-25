use super::*;

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
/// use check_latest::get_max_version;
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
pub fn get_max_version(crate_name: &str, current_crate_version: &str, user_agent: &str) -> Result<Option<Version>, Error> {
    let current_version = Version::parse(current_crate_version).map_err(|_| "Couldn't parse current version")?;
    let max_version = get_versions(crate_name, user_agent)?.max_version;
    let max_version = if current_version < max_version {
        Some(max_version)
    } else {
        None
    };
    Ok(max_version)
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
        $crate::max_version!(crate_name = $crate::crate_name!(), version = $crate::crate_version!(), user_agent = $crate::user_agent!())
    };
    // All 3 specified {{{
    (crate_name = $crate_name:expr, version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };
    (version = $version:expr, crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };
    (version = $version:expr, user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };
    (user_agent = $user_agent:expr, version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $user_agent)
    };

    (version = $version:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::get_max_version($crate::crate_name!(), $version, $user_agent)
    };
    (user_agent = $user_agent:expr, version = $version:expr $(,)?) => {
        $crate::get_max_version($crate::crate_name!(), $version, $user_agent)
    };
    (crate_name = $crate_name:expr, user_agent = $user_agent:expr $(,)?) => {
        $crate::get_max_version($crate_name, $crate::crate_version!(), $user_agent)
    };
    (user_agent = $user_agent:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::get_max_version($crate_name, $crate::crate_version!(), $user_agent)
    };
    (crate_name = $crate_name:expr, version = $version:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $crate::user_agent!())
    };
    (version = $version:expr, crate_name = $crate_name:expr $(,)?) => {
        $crate::get_max_version($crate_name, $version, $crate::user_agent!())
    };

    (crate_name = $crate_name:expr $(,)?) => {
        $crate::get_max_version($crate_name, $crate::crate_version!(), $crate::user_agent!())
    };
    (version = $version:expr $(,)?) => {
        $crate::get_max_version($crate::crate_name!(), $version, $crate::user_agent!())
    };
    (user_agent = $user_agent:expr $(,)?) => {
        $crate::get_max_version($crate::crate_name!(), $crate::crate_version!(), $user_agent)
    };

    () => {
        $crate::get_max_version($crate::crate_name!(), $crate::crate_version!(), $crate::user_agent!())
    };
}
