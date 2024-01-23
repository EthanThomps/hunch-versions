use crate::anomaly::{fixed_true_anomaly, newton_true_anomaly};
use crate::constants::{self, MARS_OBE, MARS_YEAR};
use crate::conversions::julian_to_mars;
use crate::events::SupportedPlanets;
use crate::perihelion::find_perihelion_time;

/// https://www.onlineconversion.com/julian_date.htm
pub fn sol562() {
    let date = julian_to_mars(2460332.5);
    println!("{date}");
}

pub fn compute_mars_ls(day: f64) -> f64 {
    let theta = newton_true_anomaly(MARS_OBE, day, SupportedPlanets::Mars, MARS_YEAR);

    // let theta = fixed_true_anomaly(orbital_eccentricity, day, p, orbital_period);
    let mut ls = theta - find_perihelion_time(251.0, 360.0);
    // let mut ls = -1.1374007382953426 - find_perihelion_time(251.0, 360.0);

    if ls < 0.0 {
        ls += constants::RIC;
    }

    if ls > 2.0 * std::f64::consts::PI {
        ls -= constants::RIC;
    }

    ls = ls.to_degrees();
    println!("Ls deg: {ls}");

    return ls;
}
