use serde::Deserialize;

use crate::error;

#[derive(Deserialize)]
struct IP {
    origin: String,
}

pub async fn get_public_ip() -> Result<String, error::ApplicationError> {
    let resp: IP = reqwest::get("https://httpbin.org/ip").await?.json().await?;

    Ok(resp.origin)
}
