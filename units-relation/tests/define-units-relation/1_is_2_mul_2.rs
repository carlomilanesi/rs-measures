rs_measures::define_measure_2d! {}

pub struct P1;

pub struct U1;
impl MeasurementUnit for U1 {
    type Property = P1;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u1";
}

pub struct P2;
impl VectorProperty for P2 {}

pub struct U2;
impl MeasurementUnit for U2 {
    type Property = P2;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u2";
}

pub struct P3;
impl VectorProperty for P3 {}

pub struct U3;
impl MeasurementUnit for U3 {
    type Property = P3;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u3";
}

units_relation::define_units_relation! { U1 == U2:2 * U3:2 }

fn main() {
    let u1: Measure<U1> = Measure2d::<U2>::new(6., 4.) * Measure2d::<U3>::new(7., 8.);
    assert_eq!(u1.value, 74.);
}
