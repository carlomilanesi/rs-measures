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

pub struct U2;
impl MeasurementUnit for U2 {
    type Property = P2;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u2";
}
impl VectorMeasurementUnit for U2 {}

units_relation::define_units_relation! { U1 == U2:2 X U2:2 }

fn main() {
    let u1: Measure<U1> = Measure2d::<U2>::new(6., 5.).cross_product(Measure2d::<U2>::new(7., 4.));
    assert_eq!(u1.value, -11.);
}
