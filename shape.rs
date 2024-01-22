/// gets the area calculations of a body in ellipse
pub fn area_in_ellipse(eccentricity: f64, semiminor_axis: f64, semimajor_axis: f64) -> f64 {
    let k = eccentricity;
    let major = semimajor_axis;
    let minor = semiminor_axis;
    let pi = std::f64::consts::PI;

    let a = pi * major * minor;
    println!("1st {a}");

    let a = pi * major.powf(2.0_f64) * (1.0 - k.powf(2.0_f64)).sqrt();
    println!("2nd {a}");

    return a;
}

pub fn planet_ellipse_area() {}
