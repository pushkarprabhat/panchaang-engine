use chrono::{DateTime, Datelike, NaiveDate, Utc, TimeZone};
use std::f64::consts::PI;

use crate::types::Paksha;
#[cfg(feature = "ephem")]
use siderust::{time::try_jd_f64, ephemeris::{Vsop87Ephemeris, DynEphemeris}};

/// Input parameters for Panchang to Gregorian conversion
pub struct PanchangToGregorianQuery {
    pub samvat_year: i32,          // e.g., 2081 (Vikram Samvat)
    pub lunar_month: u8,           // 1 to 12 (e.g., Chaitra = 1, Vaisakha = 2)
    pub is_purnimanta: bool,       // true if Purnimanta system, false if Amanta
    pub paksha: Paksha,            // Shukla (waxing) or Krishna (waning)
    pub tithi: u8,                 // 1 to 15 (15 = Purnima/Amavasya depending on Paksha)
    pub latitude: f64,             // e.g., 23.0225
    pub longitude: f64,            // e.g., 72.5714
    pub timezone_offset_hours: f64 // e.g., +5.5 for IST
}

/// Output struct representing the calculated Gregorian date range
pub struct GregorianMatch {
    pub date_time_start: DateTime<Utc>,
    pub date_time_end: DateTime<Utc>,
    pub sunrise_at_tithi: DateTime<Utc>,
}

pub fn jd_from_datetime(dt: DateTime<Utc>) -> f64 {
    // Julian day for Unix epoch 1970-01-01T00:00:00Z is 2440587.5
    let secs = dt.timestamp() as f64 + (dt.timestamp_subsec_micros() as f64) / 1_000_000f64;
    2440587.5 + secs / 86400.0
}

pub fn datetime_from_jd(jd: f64) -> DateTime<Utc> {
    let secs = (jd - 2440587.5) * 86400.0;
    let s = secs.trunc() as i64;
    let frac = secs.fract();
    let nsec = (frac * 1_000_000_000f64).round() as u32;
    if let Some(dt) = Utc.timestamp_opt(s, nsec).single() {
        dt
    } else {
        DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            Utc,
        )
    }
}

fn deg_to_rad(d: f64) -> f64 { d * PI / 180.0 }
fn rad_to_deg(r: f64) -> f64 { r * 180.0 / PI }

fn normalize_deg(mut d: f64) -> f64 {
    d %= 360.0;
    if d < 0.0 { d += 360.0 }
    d
}

// Approximate Sun ecliptic longitude (degrees) using mean longitude + simple correction
pub fn sun_ecliptic_long(jd: f64) -> f64 {
    #[cfg(feature = "ephem")]
    {
        if let Ok(jd_obj) = try_jd_f64(jd) {
            let eph = Vsop87Ephemeris::default();
            if let Ok(pos) = eph.try_sun_barycentric(jd_obj) {
                // transform to ecliptic of date and extract longitude (degrees)
                let sph = pos.to_spherical();
                let lon = sph.azimuth.value();
                return normalize_deg(lon);
            }
        }
        // fallback to simple approximation if siderust failed
        let d = jd - 2451545.0;
        let l0 = 280.46061837 + 0.98564736629 * d; // mean lon
        let m = normalize_deg(357.52911 + 0.98560028 * d);
        let mrad = deg_to_rad(m);
        let c = 1.914602 * mrad.sin() + 0.019993 * (2.0 * mrad).sin() + 0.000289 * (3.0 * mrad).sin();
        return normalize_deg(l0 + c);
    }
    #[cfg(not(feature = "ephem"))]
    {
        let d = jd - 2451545.0;
        let l0 = 280.46061837 + 0.98564736629 * d; // mean lon
        let m = normalize_deg(357.52911 + 0.98560028 * d);
        let mrad = deg_to_rad(m);
        let c = 1.914602 * mrad.sin() + 0.019993 * (2.0 * mrad).sin() + 0.000289 * (3.0 * mrad).sin();
        normalize_deg(l0 + c)
    }
}

// Approximate Moon ecliptic longitude (degrees) using mean longitude
pub fn moon_ecliptic_long(jd: f64) -> f64 {
    #[cfg(feature = "ephem")]
    {
        if let Ok(jd_obj) = try_jd_f64(jd) {
            let eph = Vsop87Ephemeris::default();
            if let Ok(pos) = eph.try_moon_geocentric(jd_obj) {
                let sph = pos.to_spherical();
                let lon = sph.azimuth.value();
                return normalize_deg(lon);
            }
        }
        // fallback approximation
        let d = jd - 2451545.0;
        let lm = 218.316 + 13.176396 * d; // mean lunar longitude
        return normalize_deg(lm);
    }
    #[cfg(not(feature = "ephem"))]
    {
        let d = jd - 2451545.0;
        let lm = 218.316 + 13.176396 * d; // mean lunar longitude
        normalize_deg(lm)
    }
}

// Lahiri Ayanamsa: approximate sidereal offset (degrees) for given JD
// Implementation: compute tropical longitude of Aries (mean equinox) vs sidereal reference using siderust nutation/precession
#[cfg(feature = "ephem")]
pub fn lahiri_ayanamsa(jd: f64) -> f64 {
    // Use approximate formula based on Lahiri constants: Ayanamsa ~ 24° 02' 30" (circa 2000) and slowly changing.
    // For higher precision, compute the difference between mean tropical and sidereal longitudes of fixed star (0 Aries).
    // Here we compute using siderust sidereal time correction: use IAU model via astro precession to get delta psi.
    // Fallback: use a common approximation: lahiri (2000) = 23.8569 deg
    _ = jd; // currently unused; placeholder for future precise implementation
    23.8569
}

// Compute tithi info at given JD: returns (paksha, tithi_number 1..15)
fn tithi_at(jd: f64) -> (Paksha, u8) {
    let sun = sun_ecliptic_long(jd);
    let moon = moon_ecliptic_long(jd);
    let diff = normalize_deg(moon - sun);
    let t_idx = (diff / 12.0).floor() as i32 + 1; // 1..30
    if diff < 180.0 {
        let t = ((t_idx) as i32).min(15) as u8;
        (Paksha::Shukla, t)
    } else {
        let t = ((t_idx - 15) as i32).max(1).min(15) as u8;
        (Paksha::Krishna, t)
    }
}

// Compute approximate sunrise JD for a given date (UTC day) and location using NOAA algorithm
pub fn sunrise_jd_for_date(date_midnight_utc_jd: f64, longitude: f64, latitude: f64) -> Option<f64> {
    // NOAA algorithm (approx)
    let lon = longitude;
    let n = (date_midnight_utc_jd - 2451545.0 - 0.0009 - lon / 360.0).round();
    let j_star = 2451545.0 + 0.0009 + (lon / 360.0) + n;
    let m = normalize_deg(357.5291 + 0.98560028 * (j_star - 2451545.0));
    let c = 1.9148 * deg_to_rad(m).sin() + 0.0200 * deg_to_rad(2.0 * m).sin() + 0.0003 * deg_to_rad(3.0 * m).sin();
    let lambda = normalize_deg(m + 102.9372 + c + 180.0);
    let j_transit = j_star + 0.0053 * deg_to_rad(m).sin() - 0.0069 * deg_to_rad(2.0 * lambda).sin();
    let obliq = deg_to_rad(23.4397);
    let lambda_rad = deg_to_rad(lambda);
    let delta = (obliq.sin() * lambda_rad.sin()).asin();
    let lat_rad = deg_to_rad(latitude);
    let cos_omega = (deg_to_rad(-0.83).sin() - lat_rad.sin() * delta.sin()) / (lat_rad.cos() * delta.cos());
    if cos_omega.abs() > 1.0 { return None; }
    let omega = rad_to_deg(cos_omega.acos());
    let _j_set = j_transit + omega / 360.0;
    let j_rise = j_transit - omega / 360.0;
    Some(j_rise)
}

// Find tithi boundary near a target JD using bisection where tithi index changes
fn find_tithi_boundary(mut left: f64, mut right: f64, target_t_idx: i32) -> f64 {
    for _ in 0..60 {
        let mid = 0.5 * (left + right);
        let sun = sun_ecliptic_long(mid);
        let moon = moon_ecliptic_long(mid);
        let diff = normalize_deg(moon - sun);
        let t_idx = (diff / 12.0).floor() as i32 + 1;
        if t_idx == target_t_idx {
            right = mid;
        } else {
            left = mid;
        }
    }
    0.5 * (left + right)
}


// Prompt for Copilot:
// Implement a function `find_gregorian_date(query: &PanchangToGregorianQuery) -> Result<Vec<GregorianMatch>, String>`
// 1. Calculate the approximate Gregorian year corresponding to the target Samvat year.
// 2. Define a 30-day search window centered around the target lunar month.
// 3. Increment through the search window day-by-day at local sunrise for the given latitude/longitude.
// 4. For each day, calculate the Sun and Moon positions to determine the active Tithi and Paksha.
// 5. If the active Tithi and Paksha match the query parameters, compute the exact start and end timestamps for that Tithi.
// 6. Return all matching Gregorian timestamp windows.

pub fn find_gregorian_date(query: &PanchangToGregorianQuery) -> Result<Vec<GregorianMatch>, String> {
    // 1. Approximate Gregorian year: Vikram Samvat is roughly +57 years ahead of Gregorian
    let approx_greg_year = query.samvat_year - 57;

    // 2. Define a 30-day search window centered around lunar month midpoint.
    // We'll approximate month -> Gregorian month by using approx_greg_year and lunar_month.
    let approx_month = (((query.lunar_month as i32 - 1) + 2) % 12) + 1; // rough mapping
    let center_date = NaiveDate::from_ymd_opt(approx_greg_year, approx_month as u32, 15)
        .ok_or_else(|| "invalid approximate date".to_string())?;

    let mut matches = Vec::new();

    // search +/-15 days
    for d_off in -15..=15 {
        let date = center_date + chrono::Duration::days(d_off.into());
        let midnight = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let jd_midnight = jd_from_datetime(DateTime::from_naive_utc_and_offset(midnight, Utc));

        // 3. Compute local sunrise JD for this date
        if let Some(jd_rise) = sunrise_jd_for_date(jd_midnight, query.longitude, query.latitude) {
            // 4. Compute tithi at sunrise
            let (paksha, t_num) = tithi_at(jd_rise);
            if paksha == query.paksha && t_num == query.tithi {
                // 5. Find start and end of this tithi (determine tithi index target)
                let sun = sun_ecliptic_long(jd_rise);
                let moon = moon_ecliptic_long(jd_rise);
                let diff = normalize_deg(moon - sun);
                let t_idx = (diff / 12.0).floor() as i32 + 1; // 1..30

                // find left boundary
                let left = jd_rise - 2.0; // two days before
                let right = jd_rise + 2.0; // two days after
                let start_jd = find_tithi_boundary(left, jd_rise, t_idx);
                let end_jd = find_tithi_boundary(jd_rise, right, t_idx + 1);

                matches.push(GregorianMatch {
                    date_time_start: datetime_from_jd(start_jd),
                    date_time_end: datetime_from_jd(end_jd),
                    sunrise_at_tithi: datetime_from_jd(jd_rise),
                });
            }
        }
    }

    Ok(matches)
}