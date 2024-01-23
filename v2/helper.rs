use crate::{
    anomaly::newton_true_anomaly,
    constants::MARS_OBE,
    events::SupportedPlanets,
    perihelion::{find_perihelion_time, planet_perihelion},
};

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

/// Computes the angle (theta 0)
pub fn compute_ls(
    orbital_eccentricity: f64,
    day: f64,
    p: SupportedPlanets,
    orbital_period: f64,
    ls_of_perihelion: f64,
    max_ls: f64,
) -> f64 {
    let theta = newton_true_anomaly(orbital_eccentricity, day, p, orbital_period);
    // let theta = fixed_true_anomaly(orbital_eccentricity, day, p, orbital_period);
    let mut ls = theta - find_perihelion_time(ls_of_perihelion, max_ls);

    if ls < 0.0 {
        ls += 2.0 * std::f64::consts::PI;
    }

    if ls > 2.0 * std::f64::consts::PI {
        ls -= 2.0 * std::f64::consts::PI;
    }

    ls = ls.to_degrees();

    return ls;
}
