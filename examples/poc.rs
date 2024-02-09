use rs_measures::traits::{MeasurementUnit, VectorProperty};

pub struct Length;
impl VectorProperty for Length {}

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Velocity;
impl VectorProperty for Velocity {}

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Acceleration;
impl VectorProperty for Acceleration {}

pub struct MetrePerSquareSecond;
impl MeasurementUnit for MetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

pub struct Force;
impl VectorProperty for Force {}

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

pub struct ElectricalResistance;

pub struct Ohm;
impl MeasurementUnit for Ohm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}";
}

pub struct ElectricalConductance;

pub struct Siemens;
impl MeasurementUnit for Siemens {
    type Property = ElectricalConductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S";
}

pub struct Torque;
impl VectorProperty for Torque {}

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}

fn main() {
    rs_measures::define_1d_2d_3d! {}
    rs_measures::define_units_relationship! {Metre == MetrePerSecond * Second}
    rs_measures::define_units_relationship! {Metre:2 == Second * MetrePerSecond:2}
    rs_measures::define_units_relationship! {MetrePerSecond:2 == MetrePerSquareSecond:2 * Second}
    rs_measures::define_units_relationship! {Metre:3 == Second * MetrePerSecond:3}
    rs_measures::define_units_relationship! {MetrePerSecond:3 == MetrePerSquareSecond:3 * Second}
    rs_measures::define_units_relationship! {Joule == Newton:2 * Metre:2}
    rs_measures::define_units_relationship! {Joule == Newton:3 * Metre:3}
    rs_measures::define_units_relationship! { Ohm == 1 / Siemens }
    rs_measures::define_units_relationship! { NewtonMetre == Newton:2 X Metre:2}
    rs_measures::define_units_relationship! { NewtonMetre:3 == Newton:3 X Metre:3}
    let length = Measure::<Metre>::new(100.);
    let duration = Measure::<Second>::new(9.8);
    println!("{}", length / duration);
    let length2 = Measure2d::<Metre>::new(100., 120.);
    println!("{}", length2 / duration);
    let length3 = Measure3d::<Metre>::new(100., 120., 150.);
    println!("{}", length3 / duration);
}
