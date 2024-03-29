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

struct Length;

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 0.001;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

#[test]
fn measure_default() {
    let m: Measure<Metre, f32> = Measure::default();
    assert_eq!(m.value, 0.);
    let m = Measure::<Metre>::default();
    assert_eq!(m.value, 0.);
}

#[test]
fn measure_new() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    assert_eq!(m.value, 12.);
}

#[test]
fn measure_convert() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.value, 12.);
    assert_eq!(m2.value, 12000.);
}

#[test]
fn measure_lossless_into_32_to_32() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossless_into_32_to_64() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossless_into_64_to_64() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossy_into_32_to_32() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossy_into_32_to_64() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossy_into_64_to_32() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_lossy_into_64_to_64() {
    let m1 = Measure::<Metre, f64>::new(12.);
    let m2: Measure<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_squared_norm_positive() {
    let m = Measure::<Metre, f32>::new(12.);
    let n: f32 = m.squared_norm();
    assert_eq!(n, 12. * 12.);
}

#[test]
fn measure_squared_norm_negative() {
    let m = Measure::<Metre, f64>::new(-12.);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 12. * 12.);
}

#[test]
fn measure_squared_norm_zero() {
    let m = Measure::<Metre, f64>::new(0.);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 0.);
}

#[test]
fn measure_normalized_positive() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, 1.);
}

#[test]
fn measure_normalized_negative() {
    let m1 = Measure::<Metre, f64>::new(-12.);
    let m2: Measure<Metre, f64> = m1.normalized();
    assert_eq!(m2.value, -1.);
}

#[test]
fn measure_normalized_positive_zero() {
    let m1 = Measure::<Metre, f32>::new(0.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, 1.);
}

#[test]
fn measure_normalized_negative_zero() {
    let m1 = Measure::<Metre, f32>::new(-0.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, -1.);
}

#[test]
fn measure_unary_minus() {
    let m: Measure<Metre, f32> = -Measure::<Metre, f32>::new(12.);
    assert_eq!(m.value, -12.);
}

#[test]
fn measure_addition() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(7.);
    let m3: Measure<Metre, f32> = m1 + m2;
    assert_eq!(m3.value, 19.);
}

#[test]
fn measure_addition_assignment() {
    let mut m = Measure::<Metre, f32>::new(12.);
    m += Measure::<Metre, f32>::new(7.);
    assert_eq!(m.value, 19.);
}

#[test]
fn measure_subtraction() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(7.);
    let m3: Measure<Metre, f32> = m1 - m2;
    assert_eq!(m3.value, 5.);
}

#[test]
fn measure_subtraction_assignment() {
    let mut m = Measure::<Metre, f32>::new(12.);
    m -= Measure::<Metre, f32>::new(7.);
    assert_eq!(m.value, 5.);
}

#[test]
fn measure_scalar_multiplication_after_32() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.) * 3.;
    assert_eq!(m.value, 36.);
}

#[test]
fn measure_scalar_multiplication_after_64() {
    let m: Measure<Metre, f64> = Measure::<Metre, f64>::new(12.) * 3.;
    assert_eq!(m.value, 36.);
}

#[test]
fn measure_scalar_multiplication_before_32() {
    let m: Measure<Metre, f32> = 3. * Measure::<Metre, f32>::new(12.);
    assert_eq!(m.value, 36.);
}

#[test]
fn measure_scalar_multiplication_before_64() {
    let m: Measure<Metre, f64> = 3. * Measure::<Metre, f64>::new(12.);
    assert_eq!(m.value, 36.);
}

#[test]
fn measure_scalar_multiplication_assignment() {
    let mut m = Measure::<Metre, f32>::new(12.);
    m *= 3.;
    assert_eq!(m.value, 36.);
}

#[test]
fn measure_scalar_division() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.) / 3.;
    assert_eq!(m.value, 4.);
}

#[test]
fn measure_scalar_division_assignment() {
    let mut m = Measure::<Metre, f32>::new(12.);
    m /= 3.;
    assert_eq!(m.value, 4.);
}

#[test]
fn measure_measure_division() {
    let m1: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = Measure::<Metre, f32>::new(3.);
    let n: f32 = m1 / m2;
    assert_eq!(n, 4.);
}

#[test]
fn measure_equals() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
}

#[test]
fn measure_differ() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
}

#[test]
fn measure_partial_cmp() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    let m4 = Measure::<Metre, f32>::new(f32::NAN);
    use std::cmp::Ordering;
    assert_eq!(m1.partial_cmp(&m2), Some(Ordering::Equal));
    assert_eq!(m1.partial_cmp(&m3), Some(Ordering::Less));
    assert_eq!(m3.partial_cmp(&m1), Some(Ordering::Greater));
    assert_eq!(m1.partial_cmp(&m4), None);
}

#[test]
fn measure_is_equal_to_its_copy() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_formatting_in_metres() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{}", m), "12.25 m");
}

#[test]
fn measure_formatting_in_metres_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1}", m), "12.2 m");
}

#[test]
fn measure_formatting_for_debug_in_metres() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:?}", m), "12.25 m");
}

#[test]
fn measure_formatting_for_debug_in_metres_one_fractional_digit() {
    let m = Measure::<Metre, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", m), "12.2 m");
}
