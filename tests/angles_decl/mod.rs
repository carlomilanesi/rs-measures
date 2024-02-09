use rs_measures::{
    angle::Angle,
    traits::{AngleMeasurementUnit, MeasurementUnit},
};
use std::f64::consts::TAU;

pub struct Cycle;
impl MeasurementUnit for Cycle {
    type Property = Angle;
    const RATIO: f64 = TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Cycle {
    const CYCLE_FRACTION: f64 = 1.;
}

pub struct Gradian;
impl MeasurementUnit for Gradian {
    type Property = Angle;
    const RATIO: f64 = TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const CYCLE_FRACTION: f64 = 400.;
}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}
