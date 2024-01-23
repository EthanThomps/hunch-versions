//! points out the positon of a body in keplerian orbit
//! Anomaly references
//! http://www.jgiesen.de/kepler/kepler.html
//! https://en.wikipedia.org/wiki/Kepler_orbit
//! https://en.wikipedia.org/wiki/Kepler%27s_equation
//! https://en.wikipedia.org/wiki/Newton%27s_method
//! https://en.wikipedia.org/wiki/True_anomaly

use crate::{
    constants,
    events::SupportedPlanets,
    helper::eccentricity,
    perihelion::{find_perihelion_time, perihelion_elapse, planet_perihelion},
};

/// https://en.wikipedia.org/wiki/Mean_anomaly
/// (in radians)
pub fn mean_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    return (constants::RIC * perihelion_elapse(day, p, orbital_period)).abs();
}

/// https://en.wikipedia.org/wiki/Mean_anomaly
/// (in radians)
pub fn old_mean_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    return constants::RIC * perihelion_elapse(day, p, orbital_period);
}

/// e1 = E = Eccentric Anomaly
/// e = Orbital Eccentricity
pub fn mean_anomaly_proof(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    let e1 = fixed_eccentric_anomaly(day, p, orbital_period);
    // let e1 = newton_eccentric_anomaly(day, p, orbital_period);
    let e = eccentricity(p);
    return e1 - e * e1.sin();
}

/// This method is the simplist one
/// https://www3.nd.edu/~zxu2/acms40390F12/Lec-2.2.pdf
pub fn fixed_eccentric_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    let mut k = 1.0;
    let n = constants::RIC;
    let m = mean_anomaly(day, p, orbital_period);
    let e = eccentricity(p);

    // E = M
    let mut e1 = m;

    // for k = 1 to n
    while k < n {
        // E = M + e * sin E
        e1 = m + e * e1.sin(); // E = M + e * sin E

        // println!(
        // "\nLoop: Eccentric Anomaly: {:?}\nLoop: Mean Anomaly: {:?}\n",
        // e1, m
        // );

        // next k
        k += 1.0;
    }

    // println!(
    // "Mean Anomaly: {:?}\nOrbital Eccentricity: {:?}\nEccentric Anomaly: {:?}",
    // m, e, e1
    // );

    // return E
    return e1;
}

// Uses newtons int
/// BUG! the newtwon true anomaly is not accurate
pub fn newton_eccentric_anomaly(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    let m = mean_anomaly(day, p, orbital_period);
    let m1 = old_mean_anomaly(day, p, orbital_period);
    let e = eccentricity(p);
    let mut needle = 1.0;

    let mut e1 = m + e * m.sin();

    // goes until needle converges to limit
    while needle > 1.0e-7 {
        // INFO! where En is the needle
        // INFO! En - e * sin(En) - M(t) / 1 - e * cos(En)
        // INFO! En = -(Eccentric Anomaly - eccentricity * sin(Eccentric Anomaly) - Mean Anomaly / 1 - eccentricity * cos(Eccentric Anomaly))
        needle = -((e1 - e * e1.sin() - m) / (1.0 - e * e1.cos()));
        e1 = e1 + needle;
    }

    if m1 < 0.0 {
        e1 = -e1;
    }

    println!("\nNewton Eccentric Anomaly: {:?}", e1);

    return 0.0;
}

/// v = 2 arctan (sqrt(1+e/1-e) * tan E/2)
/// where v = true_anomaly
/// where e = orbital eccentricity
/// where E = eccentric anomaly
/// Uses a fixed iteration (not precise)
pub fn fixed_true_anomaly(
    orbital_eccentricity: f64,
    day: f64,
    p: SupportedPlanets,
    orbital_period: f64,
) -> f64 {
    let e = orbital_eccentricity;
    let e1 = fixed_eccentric_anomaly(day, p, orbital_period);
    let theta = 2.0 * ((1.0 + e / 1.0 - e).sqrt() * (e1 / 2.0).tan()).atan();

    println!("Fixed True Anomaly: {:?}\n", theta);

    return theta;
}

/// v = 2 arctan (sqrt(1+e/1-e) * tan E/2)
/// where v = true_anomaly
/// where e = orbital eccentricity
/// where E = eccentric anomaly
/// Uses newton iterations (precise)
pub fn newton_true_anomaly(
    orbital_eccentricity: f64,
    day: f64,
    p: SupportedPlanets,
    orbital_period: f64,
) -> f64 {
    let e = orbital_eccentricity;
    let e1 = newton_eccentric_anomaly(day, p, orbital_period);
    let theta = 2.0 * ((1.0 + e / 1.0 - e).sqrt() * (e1 / 2.0).tan()).atan();

    println!("Newton True Anomaly: {:?}\n", theta);

    return theta;
}
