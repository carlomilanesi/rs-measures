use rs_measures::{define_measure1d, define_measure2d};
define_measure1d! {}
define_measure2d! {}
struct Length;
struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

// Measure2d

#[test]
fn measure_2d_values() {
    let m = Measure2d::<f32, Metre>::new(123., 456.);
    assert_eq!(123., m.x);
    assert_eq!(456., m.y);
}

#[test]
fn measure_2d_unary_minus() {
    let m = -Measure2d::<f32, Metre>::new(123., 456.);
    assert_eq!(-123., m.x);
    assert_eq!(-456., m.y);
}

#[test]
fn measures_2d_addition() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(7., -29.);
    let m3 = m1 + m2;
    assert_eq!(19., m3.x);
    assert_eq!(-26., m3.y);
}

#[test]
fn measures_2d_addition_assignment() {
    let mut m = Measure2d::<f32, Metre>::new(12., 3.);
    m += Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(19., m.x);
    assert_eq!(-26., m.y);
}

#[test]
fn measures_2d_subtraction() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(7., -29.);
    let m3 = m1 - m2;
    assert_eq!(5., m3.x);
    assert_eq!(32., m3.y);
}

#[test]
fn measures_2d_subtraction_assignment() {
    let mut m = Measure2d::<f32, Metre>::new(12., 3.);
    m -= Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(5., m.x);
    assert_eq!(32., m.y);
}

#[test]
fn measure_2d_scalar_multiplication() {
    let m = Measure2d::<f32, Metre>::new(12., 3.) * 3.;
    assert_eq!(36., m.x);
    assert_eq!(9., m.y);
}

#[test]
fn measure_2d_scalar_multiplication_assignment() {
    let mut m = Measure2d::<f32, Metre>::new(12., 3.);
    m *= 3.;
    assert_eq!(36., m.x);
    assert_eq!(9., m.y);
}

#[test]
fn measure_2d_scalar_division() {
    let m = Measure2d::<f32, Metre>::new(12., 3.) / 3.;
    assert_eq!(4., m.x);
    assert_eq!(1., m.y);
}

#[test]
fn measure_2d_scalar_division_assignment() {
    let mut m = Measure2d::<f32, Metre>::new(12., 3.);
    m /= 3.;
    assert_eq!(4., m.x);
    assert_eq!(1., m.y);
}

#[test]
fn measure_2d_equals() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(12., 3.);
    let m3 = Measure2d::<f32, Metre>::new(13., -29.);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
}

#[test]
fn measure_2d_differ() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(12., 3.);
    let m3 = Measure2d::<f32, Metre>::new(13., -29.);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
}

#[test]
fn measure_2d_is_equal_to_its_clone() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = m1.clone();
    assert!(m1 == m2);
}

#[test]
fn measure_2d_is_equal_to_its_copy() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn measure_2d_shown_in_metres() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    assert_eq!("(12, 3) m", m1.to_string());
}

#[test]
fn measure_point_2d_squared_norm() {
    let mp = Measure2d::<f32, Metre>::new(3., 4.);
    assert_eq!(25., mp.squared_norm());
}

#[test]
fn measure_point_2d_unsigned_direction() {
    let mp = Measure2d::<f32, Metre>::new(3., 4.);
    let dir = mp.unsigned_direction();
    assert!((0.927295_f32 - dir.value).abs() < 1e-6);
}

#[test]
fn measure_point_2d_signed_direction() {
    let mp = Measure2d::<f32, Metre>::new(3., 4.);
    let dir = mp.signed_direction();
    assert!((0.927295_f32 - dir.value).abs() < 1e-6);
}

// MeasurePoint

#[test]
fn measure_point_2d_value() {
    let mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    assert_eq!(12., mp.x);
    assert_eq!(3., mp.y);
}

#[test]
fn measure_point_2d_addition_of_vector() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let m = Measure2d::<f32, Metre>::new(7., -29.);
    let mp2 = mp1 + m;
    assert_eq!(19., mp2.x);
    assert_eq!(-26., mp2.y);
}

#[test]
fn measure_point_2d_addition_of_vector_assignment() {
    let mut mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    mp += Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(19., mp.x);
    assert_eq!(-26., mp.y);
}

#[test]
fn measure_point_2d_subtraction_of_vector() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let m = Measure2d::<f32, Metre>::new(7., -29.);
    let mp2 = mp1 - m;
    assert_eq!(5., mp2.x);
    assert_eq!(32., mp2.y);
}

#[test]
fn measure_point_2d_subtraction_of_vector_assignment() {
    let mut mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    mp -= Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(5., mp.x);
    assert_eq!(32., mp.y);
}

#[test]
fn measures_point_2d_subtraction() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(7., -29.);
    let m = mp1 - mp2;
    assert_eq!(5., m.x);
    assert_eq!(32., m.y);
}

#[test]
fn measure_point_2d_equals() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp3 = MeasurePoint2d::<f32, Metre>::new(13., -29.);
    assert!(mp1 == mp2);
    assert!(!(mp1 == mp3));
}

#[test]
fn measure_point_2d_differs() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp3 = MeasurePoint2d::<f32, Metre>::new(13., -29.);
    assert!(!(mp1 != mp2));
    assert!(mp1 != mp3);
}

#[test]
fn measure_point_2d_is_equal_to_its_clone() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = mp1.clone();
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_2d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_2d_shown_in_metres() {
    let mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    assert_eq!("at (12, 3) m", mp.to_string());
}
