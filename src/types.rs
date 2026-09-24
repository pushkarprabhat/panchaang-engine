use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Waxing (Shukla) or waning (Krishna) half of the lunar month.
/// Independent of Amanta/Purnimanta and of Vikrama/Shaka.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Paksha {
    Shukla,
    Krishna,
}

/// When the *month name* changes. Does not change tithi or paksha.
///
/// Amanta: month ends at Amavasya. Gujarati / Maharashtrian / most southern
/// lunar calendars. Default for this engine.
///
/// Purnimanta: month ends at Purnima. Common in North Indian printed
/// panchangs. During Krishna paksha the month *name* is one month ahead of
/// Amanta (Amanta Ashadha Krishna = Purnimanta Shravan Krishna). That is why
/// “Shravan starts” on different civil dates in Ahmedabad vs a Purnimanta book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonthSystem {
    Amanta,
    Purnimanta,
}

/// Year-numbering only. Does not change tithi, paksha, or month system.
/// Vikrama ≈ Gregorian + 57. Shaka ≈ Gregorian − 78 (Shaka 1948 ≈ 2026 CE).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    Vikrama,
    Shaka,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchangToGregorianQuery {
    pub samvat_year: i32,
    pub lunar_month: u8,
    /// false = Amanta (default). true = Purnimanta month names.
    pub is_purnimanta: bool,
    pub paksha: Paksha,
    pub tithi: u8,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_offset_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchangInput {
    pub date_time: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_meters: Option<f64>,
    pub ayanamsa_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchaangOutput {
    pub tithi_number: u8,
    pub paksha: Paksha,
    pub nakshatra_index: u8,
    pub yoga_index: u8,
    pub karana_index: u8,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub tithi_start: DateTime<Utc>,
    pub tithi_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GregorianMatch {
    pub date_time_start: DateTime<Utc>,
    pub date_time_end: DateTime<Utc>,
    pub sunrise_at_tithi: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanchaangError {
    InvalidLatitude(f64),
    InvalidLongitude(f64),
    InvalidTimezone(f64),
    InvalidTithi(u8),
    InvalidLunarMonth(u8),
    PolarDayNight,
    CalculationError(String),
}

impl fmt::Display for PanchaangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanchaangError::InvalidLatitude(v) => write!(f, "invalid latitude: {}", v),
            PanchaangError::InvalidLongitude(v) => write!(f, "invalid longitude: {}", v),
            PanchaangError::InvalidTimezone(v) => write!(f, "invalid timezone offset: {}", v),
            PanchaangError::InvalidTithi(v) => write!(f, "invalid tithi: {}", v),
            PanchaangError::InvalidLunarMonth(v) => write!(f, "invalid lunar month: {}", v),
            PanchaangError::PolarDayNight => {
                write!(f, "polar day/night: sunrise/sunset unavailable")
            }
            PanchaangError::CalculationError(s) => write!(f, "calculation error: {}", s),
        }
    }
}

impl std::error::Error for PanchaangError {}

impl From<String> for PanchaangError {
    fn from(s: String) -> Self {
        PanchaangError::CalculationError(s)
    }
}

impl From<&str> for PanchaangError {
    fn from(s: &str) -> Self {
        PanchaangError::CalculationError(s.to_string())
    }
}

pub fn validate_coords(lat: f64, lon: f64) -> Result<(), PanchaangError> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(PanchaangError::InvalidLatitude(lat));
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(PanchaangError::InvalidLongitude(lon));
    }
    Ok(())
}

pub fn validate_timezone(hours: f64) -> Result<(), PanchaangError> {
    if !(-12.0..=14.0).contains(&hours) {
        return Err(PanchaangError::InvalidTimezone(hours));
    }
    Ok(())
}

pub fn validate_tithi(tithi: u8) -> Result<(), PanchaangError> {
    if !(1..=15).contains(&tithi) {
        return Err(PanchaangError::InvalidTithi(tithi));
    }
    Ok(())
}
