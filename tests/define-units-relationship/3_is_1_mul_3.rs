rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: false,
        with_directions: false,
        with_2d: false,
        with_3d: true,
        with_transformations: false,
        with_uncertainty: None,
    }
}

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

rs_measures::define_units_relationship! { U1:3 == U2 * U3:3 }

fn main() {
    let u1: Measure3d<U1> = Measure::<U2>::new(3.) * Measure3d::<U3>::new(6., -2., 7.);
    assert_eq!(u1.x, 18.);
    assert_eq!(u1.y, -6.);
    assert_eq!(u1.z, 21.);
}
