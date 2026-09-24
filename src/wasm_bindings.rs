use wasm_bindgen::prelude::*;
use crate::panchaang;
use crate::types::{PanchangInput, PanchangToGregorianQuery, Paksha};

#[wasm_bindgen]
pub fn calculate_panchaang_js(dt_iso: &str, lat: f64, lon: f64) -> Result<JsValue, JsValue> {
    let dt = chrono::DateTime::parse_from_rfc3339(dt_iso)
        .map_err(|e| JsValue::from_str(&format!("invalid datetime: {}", e)))?
        .with_timezone(&chrono::Utc);
    let input = PanchangInput {
        date_time: dt,
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    let out = panchaang::forward::calculate_panchaang(&input)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn find_gregorian_date_js(
    samvat_year: i32,
    lunar_month: u8,
    paksha: &str,
    tithi: u8,
    lat: f64,
    lon: f64,
) -> Result<JsValue, JsValue> {
    let pak = match paksha.to_lowercase().as_str() {
        "shukla" => Paksha::Shukla,
        "krishna" => Paksha::Krishna,
        _ => return Err(JsValue::from_str("invalid paksha")),
    };
    let query = PanchangToGregorianQuery {
        samvat_year,
        lunar_month,
        is_purnimanta: false,
        paksha: pak,
        tithi,
        latitude: lat,
        longitude: lon,
        timezone_offset_hours: 5.5,
    };
    let out = panchaang::reverse::find_gregorian_date(&query)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}
