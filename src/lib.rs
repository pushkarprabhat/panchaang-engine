use chrono::{DateTime, Duration, NaiveDate, Utc};

const TITHI_ARC_DEGREES: f64 = 12.0;
const BOUNDARY_SCAN_LIMIT_HOURS: usize = 24 * 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarEra {
    VikramSamvat,
    ShakaSamvat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarMonth {
    Chaitra = 0,
    Vaishakha = 1,
    Jyeshtha = 2,
    Ashadha = 3,
    Shravana = 4,
    Bhadrapada = 5,
    Ashwin = 6,
    Kartika = 7,
    Margashirsha = 8,
    Pausha = 9,
    Magha = 10,
    Phalguna = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InverseSearchInput {
    pub era: CalendarEra,
    pub samvat_year: i32,
    pub lunar_month: LunarMonth,
    pub tithi: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TithiInterval {
    pub date: NaiveDate,
    pub sunrise_utc: DateTime<Utc>,
    pub start_utc: DateTime<Utc>,
    pub end_utc: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InverseSearchError {
    InvalidTithi(u8),
    BoundaryNotFound,
}

pub trait AstronomyEngine {
    fn sun_moon_longitudes(&self, at_utc: DateTime<Utc>) -> (f64, f64);
    fn sunrise_utc(&self, date: NaiveDate) -> DateTime<Utc>;
}

/// Estimates a 30-day Gregorian search window for an inverse Panchang lookup.
///
/// This is an approximation helper intended to seed a precise astronomical search.
/// It assumes:
/// - Era offsets of Vikram = Gregorian + 57 and Shaka = Gregorian - 78
/// - Chaitra is approximately aligned around March 21
/// - Each lunar month advances by an approximate 29-day offset
pub fn estimate_search_window(input: InverseSearchInput) -> (NaiveDate, NaiveDate) {
    let gregorian_year = match input.era {
        CalendarEra::VikramSamvat => input.samvat_year - 57,
        CalendarEra::ShakaSamvat => input.samvat_year + 78,
    };

    let base = NaiveDate::from_ymd_opt(gregorian_year, 3, 21).expect("valid base date");
    let anchor = base + Duration::days(input.lunar_month as i64 * 29);
    let start = anchor - Duration::days(15);
    let end = start + Duration::days(29);
    (start, end)
}

pub fn inverse_search_tithi<E: AstronomyEngine>(
    engine: &E,
    input: InverseSearchInput,
) -> Result<Vec<TithiInterval>, InverseSearchError> {
    if !(1..=30).contains(&input.tithi) {
        return Err(InverseSearchError::InvalidTithi(input.tithi));
    }
    let (start, end) = estimate_search_window(input);
    let mut out = Vec::new();
    let mut day = start;

    while day <= end {
        let sunrise = engine.sunrise_utc(day);
        if tithi_at(engine, sunrise) == input.tithi {
            let (start_utc, end_utc) = tithi_bounds_utc(engine, sunrise, input.tithi)?;
            out.push(TithiInterval {
                date: day,
                sunrise_utc: sunrise,
                start_utc,
                end_utc,
            });
        }
        day += Duration::days(1);
    }
    Ok(out)
}

fn tithi_bounds_utc<E: AstronomyEngine>(
    engine: &E,
    probe: DateTime<Utc>,
    target_tithi: u8,
) -> Result<(DateTime<Utc>, DateTime<Utc>), InverseSearchError> {
    let step = Duration::hours(1);
    let mut left_in = probe;
    let mut right_in = probe;
    let mut left_out = probe - step;
    let mut right_out = probe + step;
    let mut left_steps = 0usize;
    let mut right_steps = 0usize;

    while tithi_at(engine, left_out) == target_tithi {
        left_in = left_out;
        left_out -= step;
        left_steps += 1;
        if left_steps >= BOUNDARY_SCAN_LIMIT_HOURS {
            return Err(InverseSearchError::BoundaryNotFound);
        }
    }
    while tithi_at(engine, right_out) == target_tithi {
        right_in = right_out;
        right_out += step;
        right_steps += 1;
        if right_steps >= BOUNDARY_SCAN_LIMIT_HOURS {
            return Err(InverseSearchError::BoundaryNotFound);
        }
    }

    Ok((
        bisect_boundary_start(engine, left_out, left_in, target_tithi),
        bisect_boundary_end(engine, right_in, right_out, target_tithi),
    ))
}

fn bisect_boundary_start<E: AstronomyEngine>(
    engine: &E,
    mut not_target: DateTime<Utc>,
    mut target: DateTime<Utc>,
    expected_tithi: u8,
) -> DateTime<Utc> {
    while (target - not_target) > Duration::seconds(1) {
        let mid = not_target + (target - not_target) / 2;
        if tithi_at(engine, mid) == expected_tithi {
            target = mid;
        } else {
            not_target = mid;
        }
    }
    target
}

fn bisect_boundary_end<E: AstronomyEngine>(
    engine: &E,
    mut target: DateTime<Utc>,
    mut not_target: DateTime<Utc>,
    expected_tithi: u8,
) -> DateTime<Utc> {
    while (not_target - target) > Duration::seconds(1) {
        let mid = target + (not_target - target) / 2;
        if tithi_at(engine, mid) == expected_tithi {
            target = mid;
        } else {
            not_target = mid;
        }
    }
    not_target
}

fn tithi_at<E: AstronomyEngine>(engine: &E, at_utc: DateTime<Utc>) -> u8 {
    let (sun, moon) = engine.sun_moon_longitudes(at_utc);
    tithi_from_longitudes(sun, moon)
}

fn tithi_from_longitudes(sun: f64, moon: f64) -> u8 {
    let mut angular_distance = moon - sun;
    angular_distance %= 360.0;
    if angular_distance < 0.0 {
        angular_distance += 360.0;
    }
    let tithi = (angular_distance / TITHI_ARC_DEGREES).floor() as u8 + 1;
    tithi.min(30)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, TimeZone, Timelike};

    struct MockAstronomy {
        epoch: DateTime<Utc>,
        moon_rate_deg_per_hour: f64,
    }

    impl AstronomyEngine for MockAstronomy {
        fn sun_moon_longitudes(&self, at_utc: DateTime<Utc>) -> (f64, f64) {
            let seconds = (at_utc - self.epoch).num_seconds() as f64;
            let hours = seconds / 3600.0;
            (0.0, hours * self.moon_rate_deg_per_hour)
        }

        fn sunrise_utc(&self, date: NaiveDate) -> DateTime<Utc> {
            Utc.from_utc_datetime(&date.and_time(NaiveTime::from_hms_opt(6, 0, 0).unwrap()))
        }
    }

    #[test]
    fn estimates_thirty_day_window() {
        let input = InverseSearchInput {
            era: CalendarEra::ShakaSamvat,
            samvat_year: 1948,
            lunar_month: LunarMonth::Chaitra,
            tithi: 2,
        };
        let (start, end) = estimate_search_window(input);
        assert_eq!(end.signed_duration_since(start).num_days(), 29);
        assert!(start <= NaiveDate::from_ymd_opt(2026, 3, 21).unwrap());
    }

    #[test]
    fn finds_tithi_and_precise_utc_bounds_at_sunrise() {
        let engine = MockAstronomy {
            epoch: Utc.with_ymd_and_hms(2026, 3, 20, 0, 0, 0).unwrap(),
            moon_rate_deg_per_hour: 0.5,
        };
        let input = InverseSearchInput {
            era: CalendarEra::ShakaSamvat,
            samvat_year: 1948,
            lunar_month: LunarMonth::Chaitra,
            tithi: 2,
        };

        let matches = inverse_search_tithi(&engine, input).unwrap();
        let target = matches
            .iter()
            .find(|m| m.date == NaiveDate::from_ymd_opt(2026, 3, 21).unwrap())
            .unwrap();

        assert_eq!(target.sunrise_utc.hour(), 6);
        assert_eq!(target.start_utc, Utc.with_ymd_and_hms(2026, 3, 21, 0, 0, 0).unwrap());
        assert_eq!(target.end_utc, Utc.with_ymd_and_hms(2026, 3, 22, 0, 0, 0).unwrap());
    }

    #[test]
    fn finds_bounds_when_transition_is_between_hourly_probes() {
        let engine = MockAstronomy {
            epoch: Utc.with_ymd_and_hms(2026, 3, 20, 0, 30, 0).unwrap(),
            moon_rate_deg_per_hour: 0.5,
        };
        let input = InverseSearchInput {
            era: CalendarEra::ShakaSamvat,
            samvat_year: 1948,
            lunar_month: LunarMonth::Chaitra,
            tithi: 2,
        };

        let matches = inverse_search_tithi(&engine, input).unwrap();
        let target = matches
            .iter()
            .find(|m| m.date == NaiveDate::from_ymd_opt(2026, 3, 21).unwrap())
            .unwrap();

        assert_eq!(target.start_utc, Utc.with_ymd_and_hms(2026, 3, 21, 0, 30, 0).unwrap());
        assert_eq!(target.end_utc, Utc.with_ymd_and_hms(2026, 3, 22, 0, 30, 0).unwrap());
    }
}
