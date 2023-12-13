rs_measures::define_measure_3d!{}

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

define_units_relation::define_units_relation! { U1:3 X U1:3 == U2:3 }

fn main() {
    let _: Measure3d<U2> = Measure3d::<U1>::new(1.2, 1.3, 1.4).cross_product(Measure3d::<U1>::new(2.3, 2.4, 2.5));
}
