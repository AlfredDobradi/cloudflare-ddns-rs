use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use serde::{Serialize, Deserialize};
use crate::config::Config;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CfResponse {
    pub success: bool,
    pub errors: Vec<String>,
    pub result: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub zone_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub content: String,
}

pub fn get_records_for_zone(config: &Config, zone_id: &String, ip: &String) -> Result<Vec<Record>, Box<dyn Error>> {
    let url = format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records",
                               zone_id = zone_id);
    
    let api_key = config.cf_api_key.clone();

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

    let mut filtered: Vec<Record> = Vec::new();
    resp.result.into_iter().for_each(|item| {
        if record_filter(config, &item, ip) {
            filtered.push(item);
        }
    });

    Ok(filtered)
}

pub fn update_records(config: Config, records: Vec<Record>, ip: &str) {
    records.into_iter().for_each(|record| {
        let url = format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}",
                                zone_id = record.zone_id,
                                record_id = record.id);

        let mut headers: HeaderMap = HeaderMap::new();
        headers.append(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.append(AUTHORIZATION, format!("Bearer {}", config.cf_api_key).parse().unwrap());

        let record_name = record.name.clone();
        let mut patch = record;
        patch.content = ip.to_owned();

        let client = reqwest::blocking::Client::new();
        let resp = client.put(url)
            .headers(headers)
            .json(&patch)
            .send();

        match resp {
            Ok(_) => println!("Successfully updated record {}", record_name),
            Err(e) => eprintln!("Failed to update record {}: {}", record_name, e),
        }
    });
}

fn record_filter(config: &Config, record: &Record, new_ip: &String) -> bool {
    if record.kind != "A" {
        return false
    }

    let zone_config = config.records_to_update.get(&record.zone_id);
    match zone_config {
        None => {
            return false;
        },
        Some(zone) => {
            if !zone.records.contains(&record.name) {
                return false;
            }
        }
    };

    let old_ip = record.content.clone();
    if &old_ip == new_ip {
        println!("Record {} is up to date", record.name);
        return false;
    }

    true
}