use semver::Version;

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
