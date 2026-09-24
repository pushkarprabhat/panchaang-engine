use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Paksha {
    Shukla,
    Krishna,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonthSystem {
    Amanta,
    Purnimanta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchangToGregorianQuery {
    pub samvat_year: i32,
    pub lunar_month: u8,
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
pub struct PanchangQuery {
    pub target_samvat: i32,
    pub month_name: String,
    pub paksha: Paksha,
    pub tithi_number: u8,
    pub latitude: f64,
    pub longitude: f64,
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
            PanchaangError::PolarDayNight => write!(f, "polar day/night: sunrise/sunset unavailable"),
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
