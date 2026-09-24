//! Named places are a convenience index. The engine itself only needs lat/lon.

pub mod cities;

pub use cities::{lookup_city, City};
