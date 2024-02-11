rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: false,
        with_directions: false,
        with_2d: true,
        with_3d: false,
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

rs_measures::define_units_relationship! { U1:2 == U2 * U3:2 }

fn main() {
    let u1: Measure2d<U1> = Measure::<U2>::new(6.) * Measure2d::<U3>::new(7., 8.);
    assert_eq!(u1.x, 42.);
    assert_eq!(u1.y, 48.);
}
