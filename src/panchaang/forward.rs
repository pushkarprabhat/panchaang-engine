
use chrono::{DateTime, Utc};
use crate::types::{PanchangInput, Paksha, PanchaangError};

pub struct PanchaangOutput {
	pub tithi_number: u8,
	pub paksha: Paksha,
	pub nakshatra_index: u8, // 1..27
	pub yoga_index: u8,      // 1..27
	pub karana_index: u8,    // 1..11
	pub sunrise: DateTime<Utc>,
	pub sunset: DateTime<Utc>,
}

fn jd_from_datetime(dt: DateTime<Utc>) -> f64 {
	let secs = dt.timestamp() as f64 + (dt.timestamp_subsec_micros() as f64) / 1_000_000f64;
	2440587.5 + secs / 86400.0
}

fn normalize_deg(mut d: f64) -> f64 { d = d % 360.0; if d < 0.0 { d += 360.0 } d }

fn sun_mean_long(jd: f64) -> f64 {
	#[cfg(feature = "ephem")]
	{
		let ang = crate::panchaang::reverse::sun_ecliptic_long(jd);
		return normalize_deg(ang);
	}
	#[cfg(not(feature = "ephem"))]
	{
		let d = jd - 2451545.0;
		normalize_deg(280.46061837 + 0.98564736629 * d)
	}
}

fn sun_true_long(jd: f64) -> f64 {
	#[cfg(feature = "ephem")]
	{
		crate::panchaang::reverse::sun_ecliptic_long(jd)
	}
	#[cfg(not(feature = "ephem"))]
	{
		let d = jd - 2451545.0;
		let m = normalize_deg(357.52911 + 0.98560028 * d);
		let mrad = m.to_radians();
		let c = 1.914602 * mrad.sin() + 0.019993 * (2.0 * mrad).sin() + 0.000289 * (3.0 * mrad).sin();
		normalize_deg(sun_mean_long(jd) + c)
	}
}

fn moon_mean_long(jd: f64) -> f64 {
	#[cfg(feature = "ephem")]
	{
		crate::panchaang::reverse::moon_ecliptic_long(jd)
	}
	#[cfg(not(feature = "ephem"))]
	{
		let d = jd - 2451545.0;
		normalize_deg(218.316 + 13.176396 * d)
	}
}

// Simple sunrise/sunset estimation using previous helper (in reverse.rs), approximate
fn approx_sunrise_sunset(jd_midnight: f64, lon: f64, lat: f64) -> Result<(f64,f64), PanchaangError> {
	// reuse NOAA-based method: compute sunrise JD and estimate sunset as opposite
	let rise = crate::panchaang::reverse::sunrise_jd_for_date(jd_midnight, lon, lat)?;
	let set = rise + 0.5; // approximate: 12 hours later
	Ok((rise, set))
}

pub fn calculate_panchaang(input: &PanchangInput) -> Result<PanchaangOutput, PanchaangError> {
	// Input validation
	if !( -90.0..=90.0 ).contains(&input.latitude) {
		return Err(PanchaangError::InvalidLatitude(input.latitude));
	}
	if !( -180.0..=180.0 ).contains(&input.longitude) {
		return Err(PanchaangError::InvalidLongitude(input.longitude));
	}
	// 1. JD
	let jd = jd_from_datetime(input.date_time);

	// 2. Sun and Moon mean/true longitudes
	let _sun_mean = sun_mean_long(jd);
	let sun_true = sun_true_long(jd);
	let moon_mean = moon_mean_long(jd);

	// 3. Tithi calculation
	let diff = normalize_deg(moon_mean - sun_true);
	let tithi_idx = (diff / 12.0).floor() as u8 + 1; // 1..30
	let (paksha, t_num) = if diff < 180.0 {
		(Paksha::Shukla, tithi_idx.min(15))
	} else {
		(Paksha::Krishna, (tithi_idx - 15).max(1))
	};

	// Nakshatra: moon longitude / (360/27)
	let nak = ((normalize_deg(moon_mean) / (360.0 / 27.0)).floor() as i32 + 1) as u8;

	// Yoga: (Sun lon + Moon lon) normalized then divided into 27 parts
	let yoga = (((normalize_deg(sun_true + moon_mean)) / (360.0 / 27.0)).floor() as i32 + 1) as u8;

	// Karana: each tithi has two karanas; simplified mapping use tithi index
	let kar = ((tithi_idx as u8 - 1) % 11) + 1;

	// 4. Sunrise/Sunset
	let date = input.date_time.date_naive();
	let midnight = date.and_hms_opt(0, 0, 0).ok_or_else(|| "invalid date".to_string())?;
	let jd_mid = jd_from_datetime(DateTime::from_naive_utc_and_offset(midnight, Utc));
	let (rise_jd, set_jd) = match approx_sunrise_sunset(jd_mid, input.longitude, input.latitude) {
		Ok((r, s)) => (r, s),
		Err(e) => return Err(e),
	};

	Ok(PanchaangOutput {
		tithi_number: t_num,
		paksha,
		nakshatra_index: nak,
		yoga_index: yoga,
		karana_index: kar,
		sunrise: crate::panchaang::reverse::datetime_from_jd(rise_jd),
		sunset: crate::panchaang::reverse::datetime_from_jd(set_jd),
	})
}
