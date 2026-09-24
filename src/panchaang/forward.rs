use chrono::{DateTime, Utc};

use crate::ephemeris;
use crate::types::{validate_coords, PanchaangError, PanchaangOutput, PanchangInput};

use super::{nakshatra_yoga_karana, tithi_at, tithi_bounds_jd};

pub fn calculate_panchaang(input: &PanchangInput) -> Result<PanchaangOutput, PanchaangError> {
    validate_coords(input.latitude, input.longitude)?;

    let date = input.date_time.date_naive();
    let midnight = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| PanchaangError::from("invalid date"))?;
    let jd_mid = ephemeris::jd_from_datetime(DateTime::from_naive_utc_and_offset(midnight, Utc));
    let (rise_jd, set_jd) = ephemeris::sunrise_sunset_jd(jd_mid, input.longitude, input.latitude)?;

    let (paksha, t_num) = tithi_at(rise_jd);
    let (nak, yoga, kar) = nakshatra_yoga_karana(rise_jd);
    let (start_jd, end_jd) = tithi_bounds_jd(rise_jd);

    let _ = (input.ayanamsa_id, input.elevation_meters);

    Ok(PanchaangOutput {
        tithi_number: t_num,
        paksha,
        nakshatra_index: nak,
        yoga_index: yoga,
        karana_index: kar,
        sunrise: ephemeris::datetime_from_jd(rise_jd),
        sunset: ephemeris::datetime_from_jd(set_jd),
        tithi_start: ephemeris::datetime_from_jd(start_jd),
        tithi_end: ephemeris::datetime_from_jd(end_jd),
    })
}
