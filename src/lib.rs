use semver::Version;
// const VERSION: &str = env!("CARGO_PKG_VERSION");
// const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

// TODO Make macro

#[derive(Debug)]
pub struct Versions {
    /// The maximum version.
    pub max_version: Version,
    /// The newest version. Not necessarily that maximum version.
    pub newest_version: Version,
}

/// *__NOTE__ You probably want to use `max_version!`*
///
/// `crate_name`: The crate that the version should be checked for.
///
/// `user_agent`: without a proper User-Agent, the request to the Crates.io API
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
pub fn get_versions(crate_name: &str, user_agent: &str) -> Result<Versions, String> {
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
