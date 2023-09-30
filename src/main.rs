use std::error::Error;

mod record;
mod config;
mod ip;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::read_config("./config.json")?;

    let public_ip = ip::get_public_ip()?;

    println!("Public IP: {}", public_ip);

    config.clone().records_to_update.into_iter().for_each(|item| {
        let zone_id = item.1.zone_id.clone();
        println!("Getting records for zone {}", &zone_id);
        match record::get_records_for_zone(&config, &zone_id, &public_ip) {
            Ok(r) => record::update_records(config.clone(), r, &public_ip),
            Err(e) => eprintln!("error: {}", e),
        };
    });
    
    Ok(())
}