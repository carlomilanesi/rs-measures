use rs_measures::{define_measure1d, define_measure3d};
define_measure1d! {}
define_measure3d! {}
struct Length;
struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

// Measure3d

#[test]
fn measure_3d_values() {
    let m = Measure3d::<f32, Metre>::new(123., 456., 789.);
    assert_eq!(123., m.x);
    assert_eq!(456., m.y);
    assert_eq!(789., m.z);
}

#[test]
fn measure_3d_unary_minus() {
    let m = -Measure3d::<f32, Metre>::new(123., 456., -789.);
    assert_eq!(-123., m.x);
    assert_eq!(-456., m.y);
    assert_eq!(789., m.z);
}

#[test]
fn measures_3d_addition() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., 13.);
    let m2 = Measure3d::<f32, Metre>::new(7., -29., 50.);
    let m3 = m1 + m2;
    assert_eq!(19., m3.x);
    assert_eq!(-26., m3.y);
    assert_eq!(63., m3.z);
}

#[test]
fn measures_3d_addition_assignment() {
    let mut m = Measure3d::<f32, Metre>::new(12., 3., 13.);
    m += Measure3d::<f32, Metre>::new(7., -29., 50.);
    assert_eq!(19., m.x);
    assert_eq!(-26., m.y);
    assert_eq!(63., m.z);
}

#[test]
fn measures_3d_subtraction() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., 13.);
    let m2 = Measure3d::<f32, Metre>::new(7., -29., 50.);
    let m3 = m1 - m2;
    assert_eq!(5., m3.x);
    assert_eq!(32., m3.y);
    assert_eq!(-37., m3.z);
}

#[test]
fn measures_3d_subtraction_assignment() {
    let mut m = Measure3d::<f32, Metre>::new(12., 3., 13.);
    m -= Measure3d::<f32, Metre>::new(7., -29., 50.);
    assert_eq!(5., m.x);
    assert_eq!(32., m.y);
    assert_eq!(-37., m.z);
}

#[test]
fn measure_3d_scalar_multiplication() {
    let m = Measure3d::<f32, Metre>::new(12., 3., 13.) * 3.;
    assert_eq!(36., m.x);
    assert_eq!(9., m.y);
    assert_eq!(39., m.z);
}

#[test]
fn measure_3d_scalar_multiplication_assignment() {
    let mut m = Measure3d::<f32, Metre>::new(12., 3., 13.);
    m *= 3.;
    assert_eq!(36., m.x);
    assert_eq!(9., m.y);
    assert_eq!(39., m.z);
}

#[test]
fn measure_3d_scalar_division() {
    let m = Measure3d::<f32, Metre>::new(12., 3., -57.) / 3.;
    assert_eq!(4., m.x);
    assert_eq!(1., m.y);
    assert_eq!(-19., m.z);
}

#[test]
fn measure_3d_scalar_division_assignment() {
    let mut m = Measure3d::<f32, Metre>::new(12., 3., -57.);
    m /= 3.;
    assert_eq!(4., m.x);
    assert_eq!(1., m.y);
    assert_eq!(-19., m.z);
}

#[test]
fn measure_3d_equals() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m2 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m3 = Measure3d::<f32, Metre>::new(13., -29., 98.);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
}

#[test]
fn measure_3d_differ() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m2 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m3 = Measure3d::<f32, Metre>::new(13., -29., 98.);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
}

#[test]
fn measure_3d_is_equal_to_its_clone() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m2 = m1.clone();
    assert!(m1 == m2);
}

#[test]
fn measure_3d_is_equal_to_its_copy() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn measure_3d_shown_in_metres() {
    let m1 = Measure3d::<f32, Metre>::new(12., 3., -57.);
    assert_eq!("(12, 3, -57) m", m1.to_string());
}

#[test]
fn measure_point_3d_squared_norm() {
    let mp = Measure3d::<f32, Metre>::new(3., 4., 6.);
    assert_eq!(61., mp.squared_norm());
}

// MeasurePoint

#[test]
fn measure_point_3d_value() {
    let mp = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    assert_eq!(12., mp.x);
    assert_eq!(3., mp.y);
    assert_eq!(13., mp.z);
}

#[test]
fn measure_point_3d_addition_of_vector() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let m = Measure3d::<f32, Metre>::new(7., -29., 50.);
    let mp2 = mp1 + m;
    assert_eq!(19., mp2.x);
    assert_eq!(-26., mp2.y);
    assert_eq!(63., mp2.z);
}

#[test]
fn measure_point_3d_addition_of_vector_assignment() {
    let mut mp = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    mp += Measure3d::<f32, Metre>::new(7., -29., 50.);
    assert_eq!(19., mp.x);
    assert_eq!(-26., mp.y);
    assert_eq!(63., mp.z);
}

#[test]
fn measure_point_3d_subtraction_of_vector() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let m = Measure3d::<f32, Metre>::new(7., -29., 50.);
    let mp2 = mp1 - m;
    assert_eq!(5., mp2.x);
    assert_eq!(32., mp2.y);
    assert_eq!(-37., mp2.z);
}

#[test]
fn measure_point_3d_subtraction_of_vector_assignment() {
    let mut mp = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    mp -= Measure3d::<f32, Metre>::new(7., -29., 50.);
    assert_eq!(5., mp.x);
    assert_eq!(32., mp.y);
    assert_eq!(-37., mp.z);
}

#[test]
fn measures_point_3d_subtraction() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<f32, Metre>::new(7., -29., 50.);
    let m = mp1 - mp2;
    assert_eq!(5., m.x);
    assert_eq!(32., m.y);
    assert_eq!(-37., m.z);
}

#[test]
fn measure_point_3d_equals() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp3 = MeasurePoint3d::<f32, Metre>::new(13., -29., 50.);
    assert!(mp1 == mp2);
    assert!(!(mp1 == mp3));
}

#[test]
fn measure_point_3d_differs() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp3 = MeasurePoint3d::<f32, Metre>::new(13., -29., 50.);
    assert!(!(mp1 != mp2));
    assert!(mp1 != mp3);
}

#[test]
fn measure_point_3d_is_equal_to_its_clone() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp2 = mp1.clone();
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_3d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_3d_shown_in_metres() {
    let mp = MeasurePoint3d::<f32, Metre>::new(12., 3., 13.);
    assert_eq!("at (12, 3, 13) m", mp.to_string());
}
