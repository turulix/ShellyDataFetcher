use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ShellyResponse {
    pub wifi_sta: WifiStats,
    pub cloud: CloudStats,
    pub mqtt: MqttStats,
    pub time: String,
    pub unixtime: u128,
    pub serial: u64,
    pub has_update: bool,
    pub mac: String,
    pub cfg_changed_cnt: u64,
    pub actions_stats: ActionStats,
    pub relays: Vec<Relay>,
    pub meters: Vec<Meter>,
    pub temperature: f32,
    pub overtemperature: bool,
    pub tmp: TempStats,
    pub update: Update,
    pub ram_total: i64,
    pub ram_free: i64,
    pub fs_size: i64,
    pub fs_free: i64,
    pub uptime: i64,
}

#[derive(Deserialize)]
pub struct WifiStats {
    pub connected: bool,
    pub rssi: i16,
    pub ssid: String,
    pub ip: String,
}

#[derive(Deserialize)]
pub struct CloudStats {
    pub enabled: bool,
    pub connected: bool,
}

#[derive(Deserialize)]
pub struct MqttStats {
    pub connected: bool,
}

#[derive(Deserialize)]
pub struct ActionStats {
    pub skipped: i32,
}

#[derive(Deserialize)]
pub struct Relay {
    pub ison: bool,
    pub has_timer: bool,
    pub timer_started: i32,
    pub timer_duration: i32,
    pub timer_remaining: i32,
    pub overpower: bool,
    pub source: String,
}

#[derive(Deserialize)]
pub struct Meter {
    pub power: f32,
    pub overpower: f32,
    pub is_valid: bool,
    pub timestamp: u64,
    pub counters: Vec<f32>,
    pub total: f32,
}

#[derive(Deserialize)]
pub struct TempStats {
    #[serde(rename = "tC")]
    pub t_c: f32,
    #[serde(rename = "tF")]
    pub t_f: f32,
    pub is_valid: bool,
}
#[derive(Deserialize)]
pub struct Update {
    pub status: String,
    pub has_update: bool,
    pub new_version: String,
    pub old_version: String,
}
