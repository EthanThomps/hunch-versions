use crate::{events::SupportedPlanets, helper::mean_motion};

/// The purpose of this function is to find the perihelion date of a celstial body
///
///
/// @param {f64} start_of_month
///  The starting sol or whatever name for the day is of that perihelion month
///
/// @param {f64} end_of_month
///  The ending sol or whatever name for that end of day for the perihelion month
///
/// @param {f64} starting_ls
///  The begininng ls (ell sub ess) of that month
///
/// @param {f64} ending_ls
///  The ending ls (ell sub ess) of that month
///
/// @param {f64} ls_of_perihelion
///  The ls that occurs at the perihelion
///
/// Find the reference date where the perihelion started
pub fn find_perihelion_date(
    start_of_month: f64,
    end_of_month: f64,
    starting_ls: f64,
    ending_ls: f64,
    ls_of_perihelion: f64,
) -> f64 {
    let avg_days = end_of_month - start_of_month;
    let avg_ls = ending_ls - starting_ls;
    let ls_until_perihelion = ls_of_perihelion - starting_ls;

    // println!("{},{},{}", avg_days, avg_ls, ls_until_perihelion);
    let perihelion_day = avg_days / avg_ls;
    // println!(
    //     "{:?}",
    //     (perihelion_day * ls_until_perihelion) + start_of_month
    // );
    (perihelion_day * ls_until_perihelion) + start_of_month
}

/// Mars? = 1.90258341759902
/// https://www.desmos.com/scientific
///  2.0 * pi * (1.0 - 251/360)
pub fn find_perihelion_time(ls_of_perihelion: f64, max_ls: f64) -> f64 {
    2.0 * std::f64::consts::PI * (1.0 - ls_of_perihelion / max_ls)
}

/// The time since the perihelion
/// @returns  (t - t0)
pub fn perihelion_elapse(day: f64, p: SupportedPlanets, orbital_period: f64) -> f64 {
    return mean_motion(day, p, orbital_period) - mean_motion(day, p, orbital_period).round();
}

pub fn planet_perihelion(p: SupportedPlanets) -> (f64, f64) {
    // let planet = match p {
    match p {
        SupportedPlanets::Mars => (
            find_perihelion_date(468.5, 514.6, 240.0, 270.0, 251.0),
            find_perihelion_time(251.0, 360.0),
        ),
        _ => todo!(),
    }

    // println!(
    //     "Perihelion -> Planet: {:?}, Date: {:?}, Time: {:?}",
    //     p, planet.0, planet.1
    // );
}
