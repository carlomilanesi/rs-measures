rs_measures::define_measure_3d! {}
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
    let m = Measure3d::<Metre, f32>::new(123., 456., 789.);
    assert_eq!(m.x, 123.);
    assert_eq!(m.y, 456.);
    assert_eq!(m.z, 789.);
}

#[test]
fn measure_3d_unary_minus() {
    let m = -Measure3d::<Metre, f32>::new(123., 456., -789.);
    assert_eq!(m.x, -123.);
    assert_eq!(m.y, -456.);
    assert_eq!(m.z, 789.);
}

#[test]
fn measures_3d_addition() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., 13.);
    let m2 = Measure3d::<Metre, f32>::new(7., -29., 50.);
    let m3 = m1 + m2;
    assert_eq!(m3.x, 19.);
    assert_eq!(m3.y, -26.);
    assert_eq!(m3.z, 63.);
}

#[test]
fn measures_3d_addition_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 3., 13.);
    m += Measure3d::<Metre, f32>::new(7., -29., 50.);
    assert_eq!(m.x, 19.);
    assert_eq!(m.y, -26.);
    assert_eq!(m.z, 63.);
}

#[test]
fn measures_3d_subtraction() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., 13.);
    let m2 = Measure3d::<Metre, f32>::new(7., -29., 50.);
    let m3 = m1 - m2;
    assert_eq!(m3.x, 5.);
    assert_eq!(m3.y, 32.);
    assert_eq!(m3.z, -37.);
}

#[test]
fn measures_3d_subtraction_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 3., 13.);
    m -= Measure3d::<Metre, f32>::new(7., -29., 50.);
    assert_eq!(m.x, 5.);
    assert_eq!(m.y, 32.);
    assert_eq!(m.z, -37.);
}

#[test]
fn measure_3d_scalar_multiplication() {
    let m = Measure3d::<Metre, f32>::new(12., 3., 13.) * 3.;
    assert_eq!(m.x, 36.);
    assert_eq!(m.y, 9.);
    assert_eq!(m.z, 39.);
}

#[test]
fn measure_3d_scalar_multiplication_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 3., 13.);
    m *= 3.;
    assert_eq!(m.x, 36.);
    assert_eq!(m.y, 9.);
    assert_eq!(m.z, 39.);
}

#[test]
fn measure_3d_scalar_division() {
    let m = Measure3d::<Metre, f32>::new(12., 3., -57.) / 3.;
    assert_eq!(m.x, 4.);
    assert_eq!(m.y, 1.);
    assert_eq!(m.z, -19.);
}

#[test]
fn measure_3d_scalar_division_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 3., -57.);
    m /= 3.;
    assert_eq!(m.x, 4.);
    assert_eq!(m.y, 1.);
    assert_eq!(m.z, -19.);
}

#[test]
fn measure_3d_equals() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m2 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m3 = Measure3d::<Metre, f32>::new(13., -29., 98.);
    assert!(m2 == m1);
    assert!(!(m3 == m1));
}

#[test]
fn measure_3d_differ() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m2 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m3 = Measure3d::<Metre, f32>::new(13., -29., 98.);
    assert!(!(m2 != m1));
    assert!(m3 != m1);
}

#[test]
fn measure_3d_is_equal_to_its_clone() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m2 = m1.clone();
    assert!(m2 == m1);
}

#[test]
fn measure_3d_is_equal_to_its_copy() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_3d_shown_in_metres() {
    let m1 = Measure3d::<Metre, f32>::new(12., 3., -57.);
    assert_eq!(m1.to_string(), "(12, 3, -57) m");
}

#[test]
fn measure_point_3d_squared_norm() {
    let mp = Measure3d::<Metre, f32>::new(3., 4., 6.);
    assert_eq!(mp.squared_norm(), 61.);
}

// MeasurePoint

#[test]
fn measure_point_3d_value() {
    let mp = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    assert_eq!(mp.x, 12.);
    assert_eq!(mp.y, 3.);
    assert_eq!(mp.z, 13.);
}

#[test]
fn measure_point_3d_addition_of_vector() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let m = Measure3d::<Metre, f32>::new(7., -29., 50.);
    let mp2 = mp1 + m;
    assert_eq!(mp2.x, 19.);
    assert_eq!(mp2.y, -26.);
    assert_eq!(mp2.z, 63.);
}

#[test]
fn measure_point_3d_addition_of_vector_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    mp += Measure3d::<Metre, f32>::new(7., -29., 50.);
    assert_eq!(mp.x, 19.);
    assert_eq!(mp.y, -26.);
    assert_eq!(mp.z, 63.);
}

#[test]
fn measure_point_3d_subtraction_of_vector() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let m = Measure3d::<Metre, f32>::new(7., -29., 50.);
    let mp2 = mp1 - m;
    assert_eq!(mp2.x, 5.);
    assert_eq!(mp2.y, 32.);
    assert_eq!(mp2.z, -37.);
}

#[test]
fn measure_point_3d_subtraction_of_vector_assignment() {
    let mut mp = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    mp -= Measure3d::<Metre, f32>::new(7., -29., 50.);
    assert_eq!(mp.x, 5.);
    assert_eq!(mp.y, 32.);
    assert_eq!(mp.z, -37.);
}

#[test]
fn measures_point_3d_subtraction() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<Metre, f32>::new(7., -29., 50.);
    let m = mp1 - mp2;
    assert_eq!(m.x, 5.);
    assert_eq!(m.y, 32.);
    assert_eq!(m.z, -37.);
}

#[test]
fn measure_point_3d_equals() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp3 = MeasurePoint3d::<Metre, f32>::new(13., -29., 50.);
    assert!(mp1 == mp2);
    assert!(!(mp1 == mp3));
}

#[test]
fn measure_point_3d_differs() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp2 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp3 = MeasurePoint3d::<Metre, f32>::new(13., -29., 50.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn measure_point_3d_is_equal_to_its_clone() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp2 = mp1.clone();
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_3d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_3d_shown_in_metres() {
    let mp = MeasurePoint3d::<Metre, f32>::new(12., 3., 13.);
    assert_eq!(mp.to_string(), "at (12, 3, 13) m");
}
