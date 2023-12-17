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

define_units_relation::define_units_relation! { U1:2 * U2 == U3:2 }

fn main() {
    let u3: Measure2d<U3> = Measure2d::<U1>::new(6., 7.) * Measure::<U2>::new(4.);
    assert_eq!(u3.x, 24.);
    assert_eq!(u3.y, 28.);
}