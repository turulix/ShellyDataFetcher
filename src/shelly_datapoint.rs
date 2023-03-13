use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable)]
pub struct ShellyDatapoint {
    pub time: DateTime<Utc>,
    pub temperature: f32,
    pub uptime: i64,
    pub ram_free: i64,
    pub fs_free: i64,
    pub power: f32,
    pub power_total: f32,
    #[influxdb(tag)] pub ip: String,
    #[influxdb(tag)] pub mac: String,
    #[influxdb(tag)] pub serial: String,
    #[influxdb(tag)] pub ison: bool,
}
