use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use super::{deg_to_rad, normalize_deg, rad_to_deg};
use crate::types::PanchaangError;

pub fn jd_from_datetime(dt: DateTime<Utc>) -> f64 {
    let secs = dt.timestamp() as f64 + f64::from(dt.timestamp_subsec_micros()) / 1_000_000.0;
    2440587.5 + secs / 86400.0
}

pub fn datetime_from_jd(jd: f64) -> DateTime<Utc> {
    let secs = (jd - 2440587.5) * 86400.0;
    let s = secs.trunc() as i64;
    let nsec = (secs.fract() * 1_000_000_000.0).round() as u32;
    Utc.timestamp_opt(s, nsec).single().unwrap_or_else(|| {
        DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        )
    })
}

/// NOAA sunrise and sunset JD for the UTC calendar date of `date_midnight_utc_jd`.
pub fn sunrise_sunset_jd(
    date_midnight_utc_jd: f64,
    longitude: f64,
    latitude: f64,
) -> Result<(f64, f64), PanchaangError> {
    // NOAA uses longitude positive west.
    let lw = -longitude;
    let n = (date_midnight_utc_jd - 2451545.0 - 0.0009 - lw / 360.0).round();
    let j_star = 2451545.0 + 0.0009 + lw / 360.0 + n;
    let m = normalize_deg(357.5291 + 0.98560028 * (j_star - 2451545.0));
    let c = 1.9148 * deg_to_rad(m).sin()
        + 0.0200 * deg_to_rad(2.0 * m).sin()
        + 0.0003 * deg_to_rad(3.0 * m).sin();
    let lambda = normalize_deg(m + 102.9372 + c + 180.0);
    let j_transit = j_star + 0.0053 * deg_to_rad(m).sin() - 0.0069 * deg_to_rad(2.0 * lambda).sin();
    let delta = (deg_to_rad(23.4397).sin() * deg_to_rad(lambda).sin()).asin();
    let lat = deg_to_rad(latitude);
    let cos_omega =
        (deg_to_rad(-0.83).sin() - lat.sin() * delta.sin()) / (lat.cos() * delta.cos());
    if cos_omega.abs() > 1.0 {
        return Err(PanchaangError::PolarDayNight);
    }
    let omega = rad_to_deg(cos_omega.acos());
    Ok((j_transit - omega / 360.0, j_transit + omega / 360.0))
}
