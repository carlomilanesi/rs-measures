rs_measures::define_measure_1d! {}

pub struct P1;

pub struct U1;
impl MeasurementUnit for U1 {
    type Property = P1;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u1";
}
impl VectorMeasurementUnit for U1 {}

pub struct P2;

pub struct U2;
impl MeasurementUnit for U2 {
    type Property = P2;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u2";
}
impl VectorMeasurementUnit for U2 {}

pub struct P3;

pub struct U3;
impl MeasurementUnit for U3 {
    type Property = P3;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u3";
}

units_relation::define_units_relation! { U1:4 == U2 * U3 }

fn main() {}
