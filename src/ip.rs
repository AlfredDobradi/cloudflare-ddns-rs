use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct IP {
    origin: String
}

pub fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let resp: IP = reqwest::blocking::get("https://httpbin.org/ip")?
        .json()?;
    
    Ok(resp.origin)
}