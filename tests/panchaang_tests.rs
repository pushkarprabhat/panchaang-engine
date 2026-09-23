use chrono::{Utc, TimeZone};

use panchaang_engine::{types, panchaang};

#[test]
fn forward_and_reverse_smoke() {
    // Forward: Gregorian -> Panchang
    let input = types::PanchangInput {
        date_time: Utc.with_ymd_and_hms(2026, 9, 23, 0, 0, 0).unwrap(),
        latitude: 28.6139,
        longitude: 77.2090,
        elevation_meters: None,
        ayanamsa_id: 1,
    };

    let out = panchaang::forward::calculate_panchaang(&input).expect("forward failed");
    assert!(out.tithi_number >= 1 && out.tithi_number <= 15);
    assert!(out.nakshatra_index >= 1 && out.nakshatra_index <= 27);
    assert!(out.yoga_index >= 1 && out.yoga_index <= 27);

    // Reverse: Panchang -> Gregorian (smoke, just ensure it runs)
    let query = panchaang::reverse::PanchangToGregorianQuery {
        samvat_year: 2083,
        lunar_month: 8,
        is_purnimanta: false,
        paksha: types::Paksha::Shukla,
        tithi: 1,
        latitude: 28.6139,
        longitude: 77.2090,
        timezone_offset_hours: 5.5,
    };

    let res = panchaang::reverse::find_gregorian_date(&query);
    assert!(res.is_ok());
}
