use super::{centuries_j2000, normalize_deg};

/// Apparent ecliptic longitude of the Sun, degrees (Meeus, low order).
pub fn sun_ecliptic_long(jd: f64) -> f64 {
    let t = centuries_j2000(jd);
    let l0 = normalize_deg(280.4664567 + 36000.76982779 * t + 0.0003032028 * t * t);
    let m = normalize_deg(357.5291092 + 35999.0502909 * t - 0.0001536 * t * t);
    let mr = m.to_radians();
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * mr.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * mr).sin()
        + 0.000289 * (3.0 * mr).sin();
    let true_long = normalize_deg(l0 + c);
    let omega = 125.04 - 1934.136 * t;
    normalize_deg(true_long - 0.00569 - 0.00478 * omega.to_radians().sin())
}
