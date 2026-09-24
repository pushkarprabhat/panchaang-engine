use panchaang_engine::geo::cities::lookup_city;
use panchaang_engine::panchaang::forward::calculate_panchaang;
use panchaang_engine::types::PanchangInput;
use chrono::{TimeZone, Utc};

#[test]
fn finds_ahmedabad_and_ujjain() {
    let a = lookup_city("ahmedabad", Some("India"), None);
    assert_eq!(a.len(), 1);
    assert!((a[0].latitude - 23.0225).abs() < 0.01);
    let u = lookup_city("Ujjain", None, None);
    assert_eq!(u[0].province, "Madhya Pradesh");
}

#[test]
fn raw_latlon_does_not_need_a_named_city() {
    // A village that is not in the gazetteer still has a panchang.
    let input = PanchangInput {
        date_time: Utc.with_ymd_and_hms(2026, 9, 24, 0, 0, 0).unwrap(),
        latitude: 21.7051,
        longitude: 72.9959,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    let out = calculate_panchaang(&input).expect("bharuch coords");
    assert!(out.tithi_number >= 1 && out.tithi_number <= 15);
    assert!(out.sunrise < out.sunset);
}
