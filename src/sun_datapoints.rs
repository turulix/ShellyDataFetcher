use chrono::{Datelike, DateTime, Timelike, Utc};
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable)]
pub struct SunDatapoint {
    pub time: DateTime<Utc>,
    pub azimuth: f64,
    pub elevation: f64,
    #[influxdb(tag)] pub lat: f32,
    #[influxdb(tag)] pub long: f32,
}

impl SunDatapoint {
    pub fn new(lat: f32, long: f32) -> Self {
        let date = Utc::now();
        let year = date.year() as f64;
        let month = date.month() as f64;
        let day = date.day() as f64;
        let hour = date.hour() as f64;
        let minute = date.minute() as f64;
        let second = date.second() as f64;

        let greenwichtime = hour + minute / 60_f64 + second / 3600_f64;

        let rlat = lat.to_radians() as f64;
        let rlon = long.to_radians() as f64;

        // Days from J2000, accurate from 1901 to 2099
        let daynum = (367_f64 * year
            - 7_f64 * ((year + ((month + 9_f64) / 12_f64).floor()) / 4_f64).floor()
            + ((275_f64 * month) / 9_f64).floor()
            + day
            - 730531.5_f64
            + greenwichtime / 24_f64
        );

        // Mean longitude of the Sun
        let mean_long = daynum * 0.01720279239_f64 + 4.894967873_f64;
        let mean_anom = daynum * 0.01720197034_f64 + 6.240040768_f64;

        // Ecliptic longitude of the sun
        let eclip_long = (
            mean_long
                + 0.03342305518 * f64::sin(mean_anom)
                + 0.0003490658504 * f64::sin(2_f64 * mean_anom)
        );

        let obliquity = 0.4090877234 - 0.000000006981317008 * daynum;

        let rasc = f64::atan2(f64::cos(obliquity) * f64::sin(eclip_long), f64::cos(eclip_long));
        let decl = f64::asin(f64::sin(obliquity) * f64::sin(eclip_long));

        let sidereal = 4.894961213_f64 + 6.300388099 * daynum as f64 + rlon;

        let hour_ang = sidereal - rasc;
        let elevation = f64::asin(f64::sin(decl) * f64::sin(rlat) + f64::cos(decl) * f64::cos(rlat) * f64::cos(hour_ang));
        let azimuth = f64::atan2(
            -f64::cos(decl) * f64::cos(rlat) * f64::sin(hour_ang),
            f64::sin(decl) - f64::sin(rlat) * f64::sin(elevation),
        );

        let azimuth = into_range(azimuth.to_degrees(), 0.0, 360.0);
        let mut elevation = into_range(elevation.to_degrees(), -180.0, 180.0);

        let targ = (elevation + (10.3 / (elevation + 5.11))).to_radians();
        elevation += (1.02 / f64::tan(targ)) / 60.0;


        return SunDatapoint {
            time: date,
            azimuth,
            elevation,
            lat: lat as f32,
            long: long as f32,
        };
    }
}

fn into_range(x: f64, range_min: f64, range_max: f64) -> f64 {
    let shiftedx = x - range_min;
    let delta = range_max - range_min;
    return (((shiftedx % delta) + delta) % delta) + range_min;
}
