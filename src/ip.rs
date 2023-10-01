use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct IP {
    origin: String
}

pub async fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let resp: IP = reqwest::get("https://httpbin.org/ip").await?
        .json()
        .await?;
    
    Ok(resp.origin)
}