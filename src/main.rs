use clap::Parser;
use error::ApplicationError;

mod command;
mod config;
mod ip;
mod record;
mod error;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let args = command::Args::parse();
    let config = config::read_config(&args.config)?;

    let public_ip = ip::get_public_ip().await?;

    println!("Public IP: {}", public_ip);

    // let mut threads =
    let records = config.records_to_update.clone();
    for item in records {
        let zone_id = item.1.zone_id.clone();
        println!("Getting records for zone {}", &zone_id);
        match record::get_records_for_zone(&config, &zone_id, &public_ip).await {
            Ok(r) => {
                record::update_records(config.clone(), r, &public_ip).await;
            }
            Err(e) => eprintln!("error: {}", e),
        };
    }

    Ok(())
}
