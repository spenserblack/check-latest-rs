#[tokio::main]
async fn main() {
    if let Ok(Some(version)) = check_latest::check_max_async!().await {
        println!("A new version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_minor_async!().await {
        println!("A new minor version has been released: {}", version);
    }
    if let Ok(Some(version)) = check_latest::check_patch_async!().await {
        println!("A new patch has been released: {}", version);
    }
}
