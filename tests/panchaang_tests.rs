use chrono::{TimeZone, Utc};

use panchaang_engine::ephemeris;
use panchaang_engine::panchaang::{self, tithi_at};
use panchaang_engine::types::{
    PanchaangError, PanchangInput, PanchangToGregorianQuery, Paksha,
};

fn input(y: i32, m: u32, d: u32, lat: f64, lon: f64) -> PanchangInput {
    PanchangInput {
        date_time: Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap(),
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    }
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
    let out = panchaang::forward::calculate_panchaang(&input(2023, 11, 12, 28.6139, 77.2090))
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
        latitude: 28.6139,
        longitude: 77.2090,
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
    let gp = panchaang::forward::calculate_panchaang(&input(2023, 3, 22, 19.0760, 72.8777))
        .expect("gudi");
    assert_eq!(gp.paksha, Paksha::Shukla);
    assert_eq!(gp.tithi_number, 1);

    let cn = panchaang::forward::calculate_panchaang(&input(2024, 4, 9, 28.6139, 77.2090))
        .expect("navratri");
    assert_eq!(cn.paksha, Paksha::Shukla);
    assert_eq!(cn.tithi_number, 1);

    let dw = panchaang::forward::calculate_panchaang(&input(2023, 11, 12, 28.6139, 77.2090))
        .expect("diwali morning");
    assert_eq!(dw.paksha, Paksha::Krishna);
    assert_eq!(dw.tithi_number, 14);
}

#[test]
fn reverse_finds_shukla_pratipada() {
    let q = PanchangToGregorianQuery {
        samvat_year: 2080,
        lunar_month: 1,
        is_purnimanta: false,
        paksha: Paksha::Shukla,
        tithi: 1,
        latitude: 19.0760,
        longitude: 72.8777,
        timezone_offset_hours: 5.5,
    };
    let hits = panchaang::reverse::find_gregorian_date(&q).expect("reverse");
    assert!(!hits.is_empty());
    for h in &hits {
        assert!(h.date_time_start < h.date_time_end);
    }
}
