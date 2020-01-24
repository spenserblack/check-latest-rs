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
/// `user_agent`: crates.io requires a good user-agent. An example user-agent is
/// `"my-awesome-crate-bin/1.0.0"`, although crates.io prefers contact info.
///
/// # Example
///
/// ```ignore
/// # async fn run() {
/// if let Ok(_newer) = get_latest_version("my-awesome-crate-bin", "1.0.0") {
///     println!("There's a new version!");
/// }
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
