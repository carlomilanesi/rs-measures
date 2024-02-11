rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: false,
        with_directions: false,
        with_2d: false,
        with_3d: false,
        with_transformations: false,
        with_uncertainty: None,
    }
}

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

rs_measures::define_units_relationship! { U1 == 1 / U2 }

fn main() {
    let u1: Measure<U1> = 24. / Measure::<U2>::new(6.);
    assert_eq!(u1.value, 4.);
}