fn main() {
    // We're setting the version to check against so that we can safely assume
    // that this example will *always* find a higher version.
    let version = "0.0.1";
    // Semver works too!
    let semver_version = semver::Version::parse(version).unwrap();

    if let Ok(Some(version)) = check_latest::check_max!(version) {
        println!("A new version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_minor!(&semver_version) {
        println!("A new minor version has been released: {}", version);
    }
}
