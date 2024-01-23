/// 0-360 (degrees) | 0-2pi (radians)
use std::f64::consts::PI;

/// Mars day in seconds
pub const MARS_DAY: f64 = 88_775.245;

/// Mars year in days
pub const MARS_YEAR: f64 = 668.6;

/// Mars orbital eccentricity
pub const MARS_OBE: f64 = 0.0934;

/// Earth year in days
pub const EARTH_YEAR: f64 = 365.24;

/// Earth day in seconds
pub const EARTH_DAY: f64 = 86400.0;

/// Radians in a circle
pub const RIC: f64 = 2.0 * PI;

/// The julian date from January 1st 2000 at noon
pub const JD2NOON: f64 = 2451545.0;
