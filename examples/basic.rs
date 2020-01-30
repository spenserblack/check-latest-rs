use check_latest::max_version;

fn main() {
    // We're setting the version to check against so that we can safely assume
    // that this example will *always* find a higher version.
    let version = "0.0.1";

    if let Ok(Some(version)) = max_version!(version = version) {
        println!("A new version has been released: {}", version);
    }
}