use wasm_bindgen::prelude::*;
use crate::types::{PanchangInput, PanchaangError};
use crate::panchaang;

#[wasm_bindgen]
pub struct WasmPanchangError {
    pub message: String,
}

impl From<PanchaangError> for WasmPanchangError {
    fn from(e: PanchaangError) -> Self {
        WasmPanchangError { message: e.to_string() }
    }
}

#[wasm_bindgen]
pub fn calculate_panchaang_js(dt_iso: &str, lat: f64, lon: f64) -> Result<JsValue, JsValue> {
    let dt = match chrono::DateTime::parse_from_rfc3339(dt_iso) {
        Ok(d) => d.with_timezone(&chrono::Utc),
        Err(e) => return Err(JsValue::from_str(&format!("invalid datetime: {}", e))),
    };
    let input = PanchangInput { date_time: dt, latitude: lat, longitude: lon, elevation_meters: None, ayanamsa_id: 1 };
    match panchaang::forward::calculate_panchaang(&input) {
        Ok(out) => Ok(JsValue::from_serde(&out).map_err(|e| JsValue::from_str(&e.to_string()))?),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn find_gregorian_date_js(samvat_year: i32, lunar_month: u8, paksha: &str, tithi: u8, lat: f64, lon: f64) -> Result<JsValue, JsValue> {
    // Minimal wrapper: convert paksha string
    let pak = match paksha.to_lowercase().as_str() {
        "shukla" => crate::types::Paksha::Shukla,
        "krishna" => crate::types::Paksha::Krishna,
        _ => return Err(JsValue::from_str("invalid paksha")),
    };
    let query = crate::panchaang::reverse::PanchangToGregorianQuery {
        samvat_year,
        lunar_month,
        is_purnimanta: false,
        paksha: pak,
        tithi,
        latitude: lat,
        longitude: lon,
        timezone_offset_hours: 0.0,
    };
    match crate::panchaang::reverse::find_gregorian_date(&query) {
        Ok(v) => Ok(JsValue::from_serde(&v).map_err(|e| JsValue::from_str(&e.to_string()))?),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
