use crate::{constants::MARS_OBE, events::SupportedPlanets, perihelion::planet_perihelion};

/// @param {SupportedPlanets} p
///  Used for special numbers for unique planets
/// @param {f64} day
///  The current poition of the planet
/// @param {f64} orbital_period
///  The time it takes for the body to complete 1 orbit (year)
///
pub fn mean_motion(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    return (day - planet_perihelion(p).0) / orbital_period;
}

pub fn eccentricity(p: SupportedPlanets) -> f64 {
    match p {
        SupportedPlanets::Mars => MARS_OBE,
        _ => todo!(),
    }
}

/// https://astronomy.stackexchange.com/questions/12252/inclination-in-keplers-laws
/// https://astronomy.swin.edu.au/cosmos/O/orbital+inclination
pub fn tilt() {}

/// https://sites.astro.caltech.edu/~fdai/obliquity.html
pub fn obliquity() {}
