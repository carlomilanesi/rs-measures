rs_measures::define_measure_2d! {}

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

pub struct P3;

pub struct U3;
impl MeasurementUnit for U3 {
    type Property = P3;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u3";
}
impl VectorMeasurementUnit for U3 {}

units_relation::define_units_relation! { U1:2 == U2 * U3:2 }

fn main() {
    let u1: Measure2d<U1> = Measure::<U2>::new(6.) * Measure2d::<U3>::new(7., 8.);
    assert_eq!(u1.x, 42.);
    assert_eq!(u1.y, 48.);
}
