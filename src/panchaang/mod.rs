pub mod forward;
pub mod reverse;

pub use forward::*;
pub use reverse::*;

use crate::ephemeris;
use crate::types::Paksha;

pub fn tithi_at(jd: f64) -> (Paksha, u8) {
    let diff = ephemeris::elongation_deg(jd);
    let t_idx = (diff / 12.0).floor() as i32 + 1;
    if diff < 180.0 {
        (Paksha::Shukla, t_idx.min(15) as u8)
    } else {
        (Paksha::Krishna, (t_idx - 15).max(1).min(15) as u8)
    }
}

pub fn tithi_index_30(jd: f64) -> i32 {
    let diff = ephemeris::elongation_deg(jd);
    (diff / 12.0).floor() as i32 + 1
}

pub fn nakshatra_yoga_karana(jd: f64) -> (u8, u8, u8) {
    let (sun_s, moon_s) = ephemeris::sidereal_sun_moon(jd);
    let nak = ((moon_s / (360.0 / 27.0)).floor() as i32 + 1).clamp(1, 27) as u8;
    let yoga_span = ephemeris::normalize_deg(sun_s + moon_s);
    let yoga = ((yoga_span / (360.0 / 27.0)).floor() as i32 + 1).clamp(1, 27) as u8;
    let elong = ephemeris::elongation_deg(jd);
    let kar = ((elong / 6.0).floor() as i32).rem_euclid(60) + 1;
    (nak, yoga, kar as u8)
}

pub fn find_tithi_index_crossing(mut left: f64, mut right: f64, target: i32) -> f64 {
    let tol_days = 0.001 / 86400.0;
    for _ in 0..200 {
        if right - left <= tol_days {
            break;
        }
        let mid = 0.5 * (left + right);
        if tithi_index_30(mid) >= target {
            right = mid;
        } else {
            left = mid;
        }
    }
    0.5 * (left + right)
}

pub fn tithi_bounds_jd(probe: f64) -> (f64, f64) {
    let idx = tithi_index_30(probe);
    let start = find_tithi_index_crossing(probe - 2.0, probe, idx);
    let end = find_tithi_index_crossing(probe, probe + 2.0, idx + 1);
    (start, end)
}
