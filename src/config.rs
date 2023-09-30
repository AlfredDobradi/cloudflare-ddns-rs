use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{error::Error, fs::File};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneConfig {
    pub zone_id: String,
    pub records: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub cf_api_key: String,
    pub records_to_update: HashMap<String, ZoneConfig>,
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = File::open(path)?;

    let config: Config = serde_json::from_reader(config_file)?;

    Ok(config)
}