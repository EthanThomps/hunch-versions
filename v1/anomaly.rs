//! Anomaly references
//! http://www.jgiesen.de/kepler/kepler.html
//! https://en.wikipedia.org/wiki/Kepler%27s_equation
//! https://en.wikipedia.org/wiki/Newton%27s_method
//! https://en.wikipedia.org/wiki/True_anomaly

use crate::{
    constants, events::SupportedPlanets, helper::eccentricity, perihelion::perihelion_elapse,
};

/// https://en.wikipedia.org/wiki/Mean_anomaly
/// (in radians)
pub fn mean_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    return (constants::RIC * perihelion_elapse(day, p, orbital_period)).abs();
}

/// e1 = E = Eccentric Anomaly
/// e = Orbital Eccentricity
pub fn mean_anomaly_proof(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    let e1 = eccentric_anomaly(day, p, orbital_period);
    let e = eccentricity(p);
    return e1 - e * e1.sin();
}

/// This method is the simplist one
pub fn fixed_eccentric_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    let m = mean_anomaly(day, p, orbital_period);
    let e = eccentricity(p);
    println!("Mean Anomaly: {:?}, Eccentricity: {:?}", m, e);
    let n = constants::RIC;
    let mut e1 = m; // E = M
    let mut k = 1.0;

    loop {
        e1 = m + e * e1.sin(); // E = M + e * sin E
        k += 1.0; // next k

        if k > n {
            break;
        }
    }

    return e1; // return E
}

/// This method uses newtons iterations
///
///
///
pub fn newton_eccentric_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    0.0
}

pub fn fixed_true_anomaly() {}
pub fn newton_true_anomaly() {}

/// Where ls is computed
pub fn get_ls(day: f64) -> f64 {
    day
}
