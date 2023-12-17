rs_measures::define_measure_2d!{}

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

define_units_relation::define_units_relation! { U1:2 X U1:2 == U2 }

fn main() {
    let _: Measure<U2> = Measure2d::<U1>::new(1.2, 1.3).cross_product(Measure2d::<U1>::new(2.3, 2.4));
}
