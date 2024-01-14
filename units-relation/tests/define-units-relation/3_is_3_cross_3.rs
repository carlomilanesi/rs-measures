rs_measures::define_measure_3d! {}

pub struct P1;
impl VectorProperty for P1 {}

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

units_relation::define_units_relation! { U1:3 == U2:3 X U3:3 }

fn main() {
    let u1: Measure3d<U1> =
        Measure3d::<U2>::new(6., -3., 5.).cross_product(Measure3d::<U3>::new(-2., 7., 8.));
    assert_eq!(u1.x, -59.);
    assert_eq!(u1.y, -58.);
    assert_eq!(u1.z, 36.);
}
