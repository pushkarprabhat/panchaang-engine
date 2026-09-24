use chrono::{Duration, NaiveDate, TimeZone, Utc};

use crate::ephemeris;
use crate::types::{
    validate_coords, validate_timezone, validate_tithi, GregorianMatch, PanchaangError,
    PanchangToGregorianQuery,
};

use super::{tithi_at, tithi_bounds_jd};

pub fn sunrise_jd_for_date(
    date_midnight_utc_jd: f64,
    longitude: f64,
    latitude: f64,
) -> Result<f64, PanchaangError> {
    let (rise, _) = ephemeris::sunrise_sunset_jd(date_midnight_utc_jd, longitude, latitude)?;
    Ok(rise)
}

pub fn find_gregorian_date(
    query: &PanchangToGregorianQuery,
) -> Result<Vec<GregorianMatch>, PanchaangError> {
    validate_coords(query.latitude, query.longitude)?;
    validate_timezone(query.timezone_offset_hours)?;
    validate_tithi(query.tithi)?;
    if !(1..=12).contains(&query.lunar_month) {
        return Err(PanchaangError::InvalidLunarMonth(query.lunar_month));
    }

    let approx_greg_year = query
        .samvat_year
        .checked_sub(57)
        .ok_or_else(|| PanchaangError::CalculationError("samvat_year out of range".into()))?;

    let mut month = (((query.lunar_month as i32) + 1) % 12) + 1;
    if query.is_purnimanta {
        month = month % 12 + 1;
    }
    let center = NaiveDate::from_ymd_opt(approx_greg_year, month as u32, 15)
        .ok_or_else(|| PanchaangError::from("invalid approximate date"))?;

    let mut matches = Vec::new();
    for d_off in -2i64..=33 {
        let date = center + Duration::days(d_off);
        let midnight = date.and_hms_opt(0, 0, 0).unwrap();
        let local = Utc.from_utc_datetime(&midnight)
            - Duration::minutes((query.timezone_offset_hours * 60.0) as i64);
        let jd_mid = ephemeris::jd_from_datetime(local);

        let jd_rise = sunrise_jd_for_date(jd_mid, query.longitude, query.latitude)?;
        let (paksha, t_num) = tithi_at(jd_rise);
        if paksha == query.paksha && t_num == query.tithi {
            let (start_jd, end_jd) = tithi_bounds_jd(jd_rise);
            matches.push(GregorianMatch {
                date_time_start: ephemeris::datetime_from_jd(start_jd),
                date_time_end: ephemeris::datetime_from_jd(end_jd),
                sunrise_at_tithi: ephemeris::datetime_from_jd(jd_rise),
            });
        }
    }
    Ok(matches)
}

pub fn sun_ecliptic_long(jd: f64) -> f64 {
    ephemeris::sun_ecliptic_long(jd)
}
pub fn moon_ecliptic_long(jd: f64) -> f64 {
    ephemeris::moon_ecliptic_long(jd)
}
pub fn datetime_from_jd(jd: f64) -> chrono::DateTime<Utc> {
    ephemeris::datetime_from_jd(jd)
}
