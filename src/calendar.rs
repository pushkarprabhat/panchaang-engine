use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use serde::Serialize;

use crate::panchaang::forward::calculate_panchaang;
use crate::types::{PanchaangError, PanchaangOutput, PanchangInput, Paksha};

#[derive(Debug, Clone, Serialize)]
pub struct DayCell {
    pub date: String,
    pub tithi_number: u8,
    pub paksha: Paksha,
    pub sunrise: String,
    pub sunset: String,
}

fn day_input(date: NaiveDate, lat: f64, lon: f64) -> PanchangInput {
    let midnight = date.and_hms_opt(0, 0, 0).unwrap();
    PanchangInput {
        date_time: Utc.from_utc_datetime(&midnight),
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    }
}

pub fn panchang_on(date: NaiveDate, lat: f64, lon: f64) -> Result<PanchaangOutput, PanchaangError> {
    calculate_panchaang(&day_input(date, lat, lon))
}

pub fn month_cells(year: i32, month: u32, lat: f64, lon: f64) -> Result<Vec<DayCell>, PanchaangError> {
    let start = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| PanchaangError::from("invalid month"))?;
    let mut days = Vec::new();
    let mut d = start;
    while d.month() == month {
        let out = panchang_on(d, lat, lon)?;
        days.push(DayCell {
            date: d.to_string(),
            tithi_number: out.tithi_number,
            paksha: out.paksha,
            sunrise: out.sunrise.to_rfc3339(),
            sunset: out.sunset.to_rfc3339(),
        });
        d = d.succ_opt().unwrap();
    }
    Ok(days)
}
