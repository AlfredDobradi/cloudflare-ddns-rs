use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use std::{fs::File, error::Error, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct CfResponse {
    success: bool,
    errors: Vec<String>,
    result: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    id: String,
    zone_id: String,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ZoneConfig {
    zone_id: String,
    records: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    cf_api_key: String,
    records_to_update: HashMap<String, ZoneConfig>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config("./config.json")?;

    let public_ip = get_public_ip()?;

    println!("Public IP: {}", public_ip);

    config.clone().records_to_update.into_iter().for_each(|item| {
        println!("Getting records for zone {}", item.1.zone_id);
        match get_records_for_zone(item.1.zone_id, config.cf_api_key.clone()) {
            Ok(r) => update_records(config.clone(), r, public_ip.clone()),
            Err(_) => eprintln!("error"),
        };
    });
    


    Ok(())
}

fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = File::open(path)?;

    let config: Config = serde_json::from_reader(config_file)?;

    Ok(config)
}

fn get_public_ip() -> Result<String, Box<dyn Error>> {
    #[derive(Deserialize)]
    struct IP {
        origin: String
    }
    
    let resp: IP = reqwest::blocking::get("https://httpbin.org/ip")?
        .json()?;
    

    Ok(resp.origin)
}

fn get_records_for_zone(zone_id: String, api_key: String) -> Result<Vec<Record>, Box<dyn Error>> {
    let url = format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records",
                               zone_id = zone_id);
    
    println!("URL: {}", url);
    println!("API Key: {}", api_key);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.append(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.append(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let resp: CfResponse = client.get(url)
        .headers(headers)
        .send()?
        .json()?;

    Ok(resp.result)
}

fn update_records(config: Config, records: Vec<Record>, ip: String) {
    records.into_iter().for_each(|record| {
        if record.kind != "A" {
            return
        }

        let zone_config = config.records_to_update.get(&record.zone_id);
        match zone_config {
            None => {
                eprintln!("Zone not found in config: {}", record.zone_id);
            },
            Some(zone) => {
                if !zone.records.contains(&record.name) {
                    return
                }
            }
        };

        if record.content == ip {
            println!("Record {} is up to date", record.name);
            return
        }

        let url = format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}",
                                zone_id = record.zone_id,
                                record_id = record.id);

        let mut headers: HeaderMap = HeaderMap::new();
        headers.append(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.append(AUTHORIZATION, format!("Bearer {}", config.cf_api_key).parse().unwrap());

        let mut patch = record.clone();
        patch.content = ip.clone();

        let client = reqwest::blocking::Client::new();
        let resp = client.put(url)
            .headers(headers)
            .json(&patch)
            .send();

        match resp {
            Ok(_) => println!("Successfully updated record {}", record.name),
            Err(e) => eprintln!("Failed to update record {}: {}", record.name, e),
        }
    });
}