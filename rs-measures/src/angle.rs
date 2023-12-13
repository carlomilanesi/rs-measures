use crate::traits::{AngleMeasurementUnit, MeasurementUnit};

pub struct Angle;

pub struct Radian;
impl MeasurementUnit for Radian {
    type Property = Angle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}
impl AngleMeasurementUnit for Radian {
    const TURN_FRACTION: f64 = std::f64::consts::TAU;
}
