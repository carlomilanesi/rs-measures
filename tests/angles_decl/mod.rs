use rs_measures::{
    angle::Angle,
    traits::{AngleMeasurementUnit, MeasurementUnit},
};
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy)]
pub struct Turn;
impl MeasurementUnit for Turn {
    type Quantity = Angle;
    const RATIO: f64 = TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Turn {
    const TURN_FRACTION: f64 = 1.;
}

#[derive(Debug, Clone, Copy)]
pub struct Gradian;
impl MeasurementUnit for Gradian {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const TURN_FRACTION: f64 = 400.;
}

#[derive(Debug, Clone, Copy)]
pub struct Degree;
impl MeasurementUnit for Degree {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}
