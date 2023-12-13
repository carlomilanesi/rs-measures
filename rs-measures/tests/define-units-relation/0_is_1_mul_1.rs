rs_measures::define_measure_1d! {}

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

define_units_relation::define_units_relation! { 1 == U1 * U2 }

fn main() {
    let f: f64 = Measure::<U1>::new(6.) * Measure::<U2>::new(4.);
    assert_eq!(f, 24.);
}
