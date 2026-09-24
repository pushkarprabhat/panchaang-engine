use super::centuries_j2000;

/// Lahiri (Chitrapaksha) ayanamsa, degrees.
/// Linear model around J2000 (≈ 23.85°) at the precessional rate.
pub fn lahiri_ayanamsa(jd: f64) -> f64 {
    let t = centuries_j2000(jd);
    23.852931 + 1.396341 * t + 0.000308 * t * t
}
