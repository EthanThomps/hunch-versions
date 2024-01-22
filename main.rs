use crate::{
    anomaly::{fixed_eccentric_anomaly, newton_eccentric_anomaly},
    events::SupportedPlanets,
    perihelion::planet_perihelion,
};

pub mod anomaly;
pub mod axis;
pub mod constants;
pub mod events;
pub mod helper;
pub mod perihelion;
pub mod shape;

fn main() {
    println!("Hello, world!");

    planet_perihelion(events::SupportedPlanets::Mars);

    // let (d, p, op) = (0.0, SupportedPlanets::Mars, 0.0);
    // let t1 = fixed_eccentric_anomaly(d, p, op);
    // let t2 = newton_eccentric_anomaly(d, p, op);
    // println!("Fixed -> {:?}, Newton -> {:?}", t1, t2);
}
