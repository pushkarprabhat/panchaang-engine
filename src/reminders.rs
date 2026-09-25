//! Personal tithi reminders (janma / anniversary / death).
//! Yearly accuracy needs lunar month; without it we only find the next
//! matching paksha+tithi (about once a month).

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::calendar::panchang_on;
use crate::types::{PanchaangError, Paksha};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderKind {
    Janma,
    Anniversary,
    Death,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TithiReminder {
    pub name: String,
    pub kind: ReminderKind,
    pub paksha: Paksha,
    pub tithi: u8,
    /// 1..=12 when known (Chaitra=1). Needed for a once-a-year alert.
    pub lunar_month: Option<u8>,
    pub city: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReminderHit {
    pub date: String,
    pub tithi_number: u8,
    pub paksha: Paksha,
}

pub fn next_tithi_days(
    from: NaiveDate,
    lat: f64,
    lon: f64,
    paksha: Paksha,
    tithi: u8,
    horizon_days: i64,
) -> Result<Vec<ReminderHit>, PanchaangError> {
    let mut hits = Vec::new();
    for i in 0..=horizon_days {
        let d = from + Duration::days(i);
        let out = panchang_on(d, lat, lon)?;
        if out.paksha == paksha && out.tithi_number == tithi {
            hits.push(ReminderHit {
                date: d.to_string(),
                tithi_number: out.tithi_number,
                paksha: out.paksha,
            });
        }
    }
    Ok(hits)
}
