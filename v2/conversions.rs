use crate::{
    constants::{EARTH_DAY, MARS_DAY, MARS_YEAR},
    events::SupportedPlanets,
    julian::{days_since_j2000, jd2greg},
    planets::mars::compute_mars_ls,
};

/// https://github.com/ethanAthompson/superzone/blob/main/rust_solar/src/shared/datetime.rs
/// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
/// https://www.mars.asu.edu/christensen/docs/clancy_jgr_intercomparison_2000.pdf
/// https://www.lpl.arizona.edu/~shane/publications/piqueux_etal_icarus_2015.pdf
/// https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2006JE002770
pub fn julian_to_mars(julian_date: f64) -> String {
    // Julian date where the mars year is 12
    // https://www.onlineconversion.com/julian_date.htm
    let julian_date_ref = 2442765.66667;

    //// Julian Date TEST!
    // jd2greg(julian_date);
    // jd2greg(julian_date_ref);
    // days_since_j2000(1975, 12, 19, 4.0);
    ////

    let mut martian_year = 12.0; // inital year

    // the sol since jd2000 or sjd2000
    let mut sol = (julian_date - julian_date_ref) * EARTH_DAY / MARS_DAY;
    println!("sol:{sol}");

    while sol >= MARS_YEAR {
        sol -= MARS_YEAR;
        martian_year += 1.0;

        // println!("sol >= MY {sol}, {martian_year}");
    }

    while sol < 0.0 {
        sol += MARS_YEAR;
        martian_year -= 1.0;
        // println!("sol < 0.0 {sol}, {martian_year}");
    }

    let ls = compute_mars_ls(sol);

    let avg_ls = 30.0;
    let month = (1.0 + (ls / avg_ls)).round();
    let year = martian_year;
    let day = 1.0 + sol.floor();

    return format!(
        "Year: {:?}\nMonth: {:?}\nSol: {:?}\nSolar Longitude: {:?}\n",
        year,
        month,
        day,
        ls.trunc()
    );
}
