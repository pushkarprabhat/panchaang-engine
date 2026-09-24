//! In-tree Sun/Moon/ayanamsa/rise-set. No third-party astronomy crate.

mod ayanamsa;
mod moon;
mod rise;
mod sun;

pub use ayanamsa::lahiri_ayanamsa;
pub use moon::moon_ecliptic_long;
pub use rise::{sunrise_sunset_jd, datetime_from_jd, jd_from_datetime};
pub use sun::sun_ecliptic_long;

pub fn normalize_deg(mut d: f64) -> f64 {
    d %= 360.0;
    if d < 0.0 {
        d += 360.0;
    }
    d
}

pub fn deg_to_rad(d: f64) -> f64 {
    d.to_radians()
}

pub fn rad_to_deg(r: f64) -> f64 {
    r.to_degrees()
}

pub fn centuries_j2000(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

/// Tropical elongation Moon − Sun, 0..360.
pub fn elongation_deg(jd: f64) -> f64 {
    normalize_deg(moon_ecliptic_long(jd) - sun_ecliptic_long(jd))
}

/// Sidereal ecliptic longitudes (Lahiri).
pub fn sidereal_sun_moon(jd: f64) -> (f64, f64) {
    let aya = lahiri_ayanamsa(jd);
    (
        normalize_deg(sun_ecliptic_long(jd) - aya),
        normalize_deg(moon_ecliptic_long(jd) - aya),
    )
}
