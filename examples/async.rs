use check_latest::max_version_async;

#[tokio::main]
async fn main() {
    // We're setting the version to check against so that we can safely assume
    // that this example will *always* find a higher version.
    let version = "0.0.1";
    // Semver works too!
    let semver_version = semver::Version::parse(version).unwrap();

    if let Ok(Some(version)) = check_latest::check_max_async!(version).await {
        println!("A new version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_minor_async!(&semver_version).await {
        println!("A new minor version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_patch_async!("0.2.0").await {
        println!("A new patch has been released: {}", version);
    }
}
