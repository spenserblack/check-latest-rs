fn main() {
    if let Ok(Some(version)) = check_latest::check_max!() {
        println!("A new version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_minor!() {
        println!("A new minor version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_patch!() {
        println!("A new patch has been released: {}", version);
    }
}
