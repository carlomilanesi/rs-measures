rs_measures::define_measure_3d! {}

pub struct P1;

pub struct U1;
impl MeasurementUnit for U1 {
    type Property = P1;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u1";
}

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
impl VectorMeasurementUnit for U3 {}

define_units_relation::define_units_relation! { U1 * U2:3 == U3:3 }

fn main() {
    let u3: Measure3d<U3> = Measure::<U1>::new(6.) * Measure3d::<U2>::new(4., 5., 7.);
    assert_eq!(u3.x, 24.);
    assert_eq!(u3.y, 30.);
    assert_eq!(u3.z, 42.);
}