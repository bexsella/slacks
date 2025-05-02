/// This module contains the various mathematical functions and structures used
/// throughout the project

#[inline]
pub fn hav(f: f64) -> f64 {
    (1.0 - f.cos()) / 2.0
}
