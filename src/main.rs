#![feature(int_roundings)]

mod shelly_response;
mod shelly_datapoint;
mod sun_datapoints;

use std::fmt::{Display};
use std::process;
use std::time::{Duration, SystemTime};
use chrono::DateTime;
use influxdb::{Client};

use crate::shelly_response::ShellyResponse;
use influxdb::InfluxDbWriteable;

use tokio;
use crate::sun_datapoints::SunDatapoint;

#[tokio::main]
async fn main() {
    let influx_host = std::env::var("INFLUX_HOST").expect("INFLUX_HOST is missing from Environment Variables.");
    let influx_db = std::env::var("INFLUX_DB").expect("INFLUX_DB is missing from Environment Variables.");
    let influx_username = std::env::var("INFLUX_USERNAME").expect("INFLUX_USERNAME is missing from Environment Variables.");
    let influx_password = std::env::var("INFLUX_PASSWORD").expect("INFLUX_PASSWORD is missing from Environment Variables.");

    let shelly_ips = std::env::var("SHELLY_IPS").unwrap_or("".to_string());
    let shelly_ips: Vec<&str> = shelly_ips.split(",").collect();

    let track_sun: bool = std::env::var("TRACK_SUN").unwrap_or("false".to_string()).to_lowercase().eq("true");
    let latitude: f32 = std::env::var("LAT").unwrap_or("0.0".to_string()).parse().unwrap();
    let longitude: f32 = std::env::var("LONG").unwrap_or("0.0".to_string()).parse().unwrap();

    if shelly_ips.is_empty()
    {
        log("No Shelly IPs found in Environment Variables. Please add SHELLY_IPS with a comma separated list of IPs.");
        return;
    }

    if track_sun
    {
        log(format!("Tracking Sun at LAT: {}, LONG: {}", latitude, longitude));
    } else {
        log("Sun tracking not enabled");
    }

    let http_client = reqwest::ClientBuilder::new().timeout(Duration::from_secs(10)).build().unwrap();

    let client = Client::new(influx_host, influx_db)
        .with_auth(influx_username, influx_password)
        .with_http_client(http_client.clone());

    loop {
        let mut points = Vec::new();
        for ip in &shelly_ips
        {
            let response = http_client.get(format!("{}/status", ip)).send().await;
            let unwrapped_response: ShellyResponse = match response {
                Ok(x) => match x.json().await {
                    Ok(y) => y,
                    Err(e) => {
                        log(format!("Error parsing Shelly data from {}: {}", ip, e));
                        continue;
                    }
                },
                Err(_) => {
                    log(format!("Error getting Shelly data from {}", ip));
                    continue;
                }
            };

            let shelly = shelly_datapoint::ShellyDatapoint {
                time: chrono::Utc::now(),
                temperature: unwrapped_response.temperature,
                uptime: unwrapped_response.uptime,
                ram_free: unwrapped_response.ram_free,
                fs_free: unwrapped_response.fs_free,
                power: unwrapped_response.meters[0].power,
                power_total: unwrapped_response.meters[0].total,
                ip: unwrapped_response.wifi_sta.ip,
                mac: unwrapped_response.mac,
                serial: unwrapped_response.serial.to_string(),
                ison: unwrapped_response.relays[0].ison,
            }.into_query("shelly");
            points.push(shelly);

            if track_sun {
                let sun = SunDatapoint::new(latitude, longitude).into_query("sun_position");
                points.push(sun);
            }
        }
        let res = client.query(&points).await;
        match res {
            Ok(y) => log(format!("Successfully wrote {} Points: {}", &points.len(), y)),
            Err(e) => {
                log(format!("Error writing data to InfluxDB: {}", e));
                process::exit(1);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn log<T: Display>(message: T) {
    let system_time = SystemTime::now();
    let datetime: DateTime<chrono::Local> = system_time.into();
    println!("[{}] {}", datetime.format("%d-%m-%Y %H:%M:%S"), message);
}
