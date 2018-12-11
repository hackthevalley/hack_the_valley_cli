use reqwest::Client;

// Get API version
fn api_ver(url: &str) -> Result<(), Box<::std::error::Error>> {
    let body = reqwest::get(&format!("{}{}", url, "/version"))?.text()?;
    println!("{:?}", body);
    Ok(())
}
