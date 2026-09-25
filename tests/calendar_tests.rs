use chrono::Datelike;
use panchaang_engine::calendar::month_cells;
use panchaang_engine::reminders::{next_tithi_days, ReminderKind};
use panchaang_engine::types::Paksha;

#[test]
fn ahmedabad_month_has_all_days() {
    let days = month_cells(2026, 9, 23.0225, 72.5714).expect("month");
    assert_eq!(days.len(), 30);
    assert_eq!(days[0].date, "2026-09-01");
    assert!(days[0].tithi_number >= 1 && days[0].tithi_number <= 15);
}

#[test]
fn next_shukla_pratipada_found() {
    let from = chrono::NaiveDate::from_ymd_opt(2026, 9, 25).unwrap();
    let hits = next_tithi_days(from, 23.0225, 72.5714, Paksha::Shukla, 1, 40).expect("hits");
    assert!(!hits.is_empty());
    assert_eq!(hits[0].paksha, Paksha::Shukla);
    assert_eq!(hits[0].tithi_number, 1);
    let _ = ReminderKind::Janma;
}
