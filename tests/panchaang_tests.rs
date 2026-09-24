use chrono::{TimeZone, Utc};

use panchaang_engine::ephemeris;
use panchaang_engine::panchaang::{self, tithi_at};
use panchaang_engine::types::{
    PanchaangError, PanchangInput, PanchangToGregorianQuery, Paksha,
};

const DELHI: (f64, f64) = (28.6139, 77.2090);
const MUMBAI: (f64, f64) = (19.0760, 72.8777);
const UJJAIN: (f64, f64) = (23.1765, 75.7849);
const JAIPUR: (f64, f64) = (26.9124, 75.7873);

fn input(y: i32, m: u32, d: u32, lat: f64, lon: f64) -> PanchangInput {
    PanchangInput {
        date_time: Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap(),
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    }
}

fn assert_sunrise_tithi(y: i32, mo: u32, d: u32, lat: f64, lon: f64, paksha: Paksha, tithi: u8) {
    let out = panchaang::forward::calculate_panchaang(&input(y, mo, d, lat, lon))
        .expect("panchang");
    assert_eq!(out.paksha, paksha);
    assert_eq!(out.tithi_number, tithi);
    assert!(out.sunrise < out.sunset);
}

#[test]
fn elongation_maps_paksha() {
    let jd = ephemeris::jd_from_datetime(Utc.with_ymd_and_hms(2000, 1, 6, 18, 0, 0).unwrap());
    let (paksha, tithi) = tithi_at(jd);
    assert!(tithi >= 1 && tithi <= 15);
    assert!(paksha == Paksha::Shukla || paksha == Paksha::Krishna);
}

#[test]
fn sunrise_before_sunset_delhi() {
    let out = panchaang::forward::calculate_panchaang(&input(2023, 11, 12, DELHI.0, DELHI.1))
        .expect("delhi");
    assert!(out.sunrise < out.sunset);
    assert!(out.tithi_start < out.tithi_end);
    assert!(out.tithi_number >= 1 && out.tithi_number <= 15);
}

#[test]
fn invalid_latitude() {
    let mut inp = input(2026, 1, 1, 95.0, 0.0);
    inp.latitude = 95.0;
    let out = panchaang::forward::calculate_panchaang(&inp);
    assert!(matches!(out, Err(PanchaangError::InvalidLatitude(v)) if v == 95.0));
}

#[test]
fn invalid_tithi_rejected() {
    let q = PanchangToGregorianQuery {
        samvat_year: 2080,
        lunar_month: 8,
        is_purnimanta: false,
        paksha: Paksha::Shukla,
        tithi: 20,
        latitude: DELHI.0,
        longitude: DELHI.1,
        timezone_offset_hours: 5.5,
    };
    let res = panchaang::reverse::find_gregorian_date(&q);
    assert!(matches!(res, Err(PanchaangError::InvalidTithi(20))));
}

#[test]
fn polar_sunrise_errors() {
    let out = panchaang::forward::calculate_panchaang(&input(2023, 6, 21, 89.0, 0.0));
    assert!(matches!(out, Err(PanchaangError::PolarDayNight)));
}

#[test]
fn gold_sunrise_tithi() {
    let cities = [MUMBAI, DELHI, UJJAIN, JAIPUR];
    for (lat, lon) in cities {
        // Gudi Padwa / Ugadi 2023-03-22: Shukla Pratipada at sunrise
        assert_sunrise_tithi(2023, 3, 22, lat, lon, Paksha::Shukla, 1);
        // Chaitra Navratri day 1 2024-04-09
        assert_sunrise_tithi(2024, 4, 9, lat, lon, Paksha::Shukla, 1);
        // Diwali morning 2023-11-12: still Krishna 14 at sunrise
        assert_sunrise_tithi(2023, 11, 12, lat, lon, Paksha::Krishna, 14);
    }
}

#[test]
fn reverse_finds_shukla_pratipada() {
    let q = PanchangToGregorianQuery {
        samvat_year: 2080,
        lunar_month: 1,
        is_purnimanta: false,
        paksha: Paksha::Shukla,
        tithi: 1,
        latitude: MUMBAI.0,
        longitude: MUMBAI.1,
        timezone_offset_hours: 5.5,
    };
    let hits = panchaang::reverse::find_gregorian_date(&q).expect("reverse");
    assert!(!hits.is_empty());
    for h in &hits {
        assert!(h.date_time_start < h.date_time_end);
    }
}
