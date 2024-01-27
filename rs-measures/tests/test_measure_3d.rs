use std::f64::consts::TAU;
rs_measures::define_1d_2d_3d! {}

mod test_utils;

struct Length;
impl VectorProperty for Length {}

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

struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

#[test]
fn measure_3d_default() {
    let m: Measure3d<Metre, f32> = Measure3d::default();
    assert_eq!(m.x, 0.);
    assert_eq!(m.y, 0.);
    assert_eq!(m.z, 0.);
    let m = Measure3d::<Metre>::default();
    assert_eq!(m.x, 0.);
    assert_eq!(m.y, 0.);
    assert_eq!(m.z, 0.);
}

#[test]
fn measure_3d_new() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new(12., 23., 34.);
    assert_eq!(m.x, 12.);
    assert_eq!(m.y, 23.);
    assert_eq!(m.z, 34.);
}

#[test]
fn measure_3d_xyz_functions() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let mx: Measure<Metre, f32> = m.x();
    let my: Measure<Metre, f32> = m.y();
    let mz: Measure<Metre, f32> = m.z();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
    assert_eq!(mz.value, 34.);
}

#[test]
fn measure_3d_convert() {
    let m1: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2: Measure3d<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.x, 12.);
    assert_eq!(m1.y, 23.);
    assert_eq!(m1.z, 34.);
    assert_eq!(m2.x, 12000.);
    assert_eq!(m2.y, 23000.);
    assert_eq!(m2.z, 34000.);
}

#[test]
fn measure_2d_lossless_into_32_to_32() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_2d_lossless_into_32_to_64() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_3d_lossless_into_64_to_64() {
    let m1 = Measure3d::<Metre, f64>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_3d_lossy_into_32_to_32() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_2d_lossy_into_32_to_64() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_3d_lossy_into_64_to_32() {
    let m1 = Measure3d::<Metre, f64>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_3d_lossy_into_64_to_64() {
    let m1 = Measure3d::<Metre, f64>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.x, 12.);
    assert_eq!(m2.y, 23.);
    assert_eq!(m2.z, 34.);
}

#[test]
fn measure_3d_squared_norm_positive() {
    let m = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let n: f32 = m.squared_norm();
    assert_eq!(n, 12. * 12. + 23. * 23. + 34. * 34.);
}

#[test]
fn measure_3d_squared_norm_negative() {
    let m = Measure3d::<Metre, f64>::new(-12., -23., -34.);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 12. * 12. + 23. * 23. + 34. * 34.);
}

#[test]
fn measure_3d_squared_norm_zero() {
    let m = Measure3d::<Metre, f64>::new(0., 0., 0.);
    let n: f64 = m.squared_norm();
    assert_eq!(n, 0.);
}

#[test]
fn measure_3d_normalized_positive() {
    let m1 = Measure3d::<Metre, f64>::new(12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_32!(m2.squared_norm(), 1.);
    assert_eq!(m1.x.signum(), m2.x.signum());
    assert_eq!(m1.y.signum(), m2.y.signum());
    assert_eq!(m1.z.signum(), m2.z.signum());
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
    assert_eq_64!(m1.z / m1.x, m2.z / m2.x);
}

#[test]
fn measure_3d_normalized_x_negative() {
    let m1 = Measure3d::<Metre, f64>::new(-12., 23., 34.);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.x.signum(), m2.x.signum());
    assert_eq!(m1.y.signum(), m2.y.signum());
    assert_eq!(m1.z.signum(), m2.z.signum());
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
    assert_eq_64!(m1.z / m1.x, m2.z / m2.x);
}

#[test]
fn measure_3d_normalized_y_negative() {
    let m1 = Measure3d::<Metre, f64>::new(12., -23., 34.);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.x.signum(), m2.x.signum());
    assert_eq!(m1.y.signum(), m2.y.signum());
    assert_eq!(m1.z.signum(), m2.z.signum());
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
    assert_eq_64!(m1.z / m1.x, m2.z / m2.x);
}

#[test]
fn measure_3d_normalized_z_negative() {
    let m1 = Measure3d::<Metre, f64>::new(12., 23., -34.);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.x.signum(), m2.x.signum());
    assert_eq!(m1.y.signum(), m2.y.signum());
    assert_eq!(m1.z.signum(), m2.z.signum());
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
}

#[test]
fn measure_3d_normalized_xyz_negative() {
    let m1 = Measure3d::<Metre, f64>::new(-12., -23., -34.);
    let m2: Measure3d<Metre, f64> = m1.normalized();
    assert_eq_64!(m2.squared_norm(), 1.);
    assert_eq!(m1.x.signum(), m2.x.signum());
    assert_eq!(m1.y.signum(), m2.y.signum());
    assert_eq!(m1.z.signum(), m2.z.signum());
    assert_eq_64!(m1.y / m1.x, m2.y / m2.x);
    assert_eq_64!(m1.z / m1.x, m2.z / m2.x);
}

#[test]
fn measure_3d_normalized_zero() {
    let m1 = Measure3d::<Metre, f32>::new(0., 0., 0.);
    let m2: Measure3d<Metre, f32> = m1.normalized();
    assert!(m2.x.is_nan());
    assert!(m2.y.is_nan());
    assert!(m2.z.is_nan());
}

#[test]
fn measure_3d_unary_minus() {
    let m = -Measure3d::<Metre, f32>::new(12., 23., 34.);
    assert_eq!(m.x, -12.);
    assert_eq!(m.y, -23.);
    assert_eq!(m.z, -34.);
}

#[test]
fn measure_3d_addition() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(45., -56., 67.);
    let m3 = m1 + m2;
    assert_eq!(m1.x, 12.);
    assert_eq!(m1.y, 23.);
    assert_eq!(m1.z, 34.);
    assert_eq!(m2.x, 45.);
    assert_eq!(m2.y, -56.);
    assert_eq!(m2.z, 67.);
    assert_eq!(m3.x, 12. + 45.);
    assert_eq!(m3.y, 23. + -56.);
    assert_eq!(m3.z, 34. + 67.);
}

#[test]
fn measure_3d_addition_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(45., -56., 67.);
    m1 += m2;
    assert_eq!(m1.x, 12. + 45.);
    assert_eq!(m1.y, 23. + -56.);
    assert_eq!(m1.z, 34. + 67.);
    assert_eq!(m2.x, 45.);
    assert_eq!(m2.y, -56.);
    assert_eq!(m2.z, 67.);
}

#[test]
fn measure_3d_subtraction() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(45., -56., 67.);
    let m3 = m1 - m2;
    assert_eq!(m1.x, 12.);
    assert_eq!(m1.y, 23.);
    assert_eq!(m1.z, 34.);
    assert_eq!(m2.x, 45.);
    assert_eq!(m2.y, -56.);
    assert_eq!(m2.z, 67.);
    assert_eq!(m3.x, 12. - 45.);
    assert_eq!(m3.y, 23. - -56.);
    assert_eq!(m3.z, 34. - 67.);
}

#[test]
fn measure_3d_subtraction_assignment() {
    let mut m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(45., -56., 67.);
    m1 -= m2;
    assert_eq!(m1.x, 12. - 45.);
    assert_eq!(m1.y, 23. - -56.);
    assert_eq!(m1.z, 34. - 67.);
    assert_eq!(m2.x, 45.);
    assert_eq!(m2.y, -56.);
    assert_eq!(m2.z, 67.);
}

#[test]
fn measure_3d_scalar_multiplication_after_32() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new(12., 23., 34.) * 3.;
    assert_eq!(m.x, 12. * 3.);
    assert_eq!(m.y, 23. * 3.);
    assert_eq!(m.z, 34. * 3.);
}

#[test]
fn measure_3d_scalar_multiplication_after_64() {
    let m: Measure3d<Metre, f64> = Measure3d::<Metre, f64>::new(12., 23., 34.) * 3.;
    assert_eq!(m.x, 12. * 3.);
    assert_eq!(m.y, 23. * 3.);
    assert_eq!(m.z, 34. * 3.);
}

#[test]
fn measure_3d_scalar_multiplication_before_32() {
    let m: Measure3d<Metre, f32> = 3. * Measure3d::<Metre, f32>::new(12., 23., 34.);
    assert_eq!(m.x, 12. * 3.);
    assert_eq!(m.y, 23. * 3.);
    assert_eq!(m.z, 34. * 3.);
}

#[test]
fn measure_3d_scalar_multiplication_before_64() {
    let m: Measure3d<Metre, f64> = 3. * Measure3d::<Metre, f64>::new(12., 23., 34.);
    assert_eq!(m.x, 12. * 3.);
    assert_eq!(m.y, 23. * 3.);
    assert_eq!(m.z, 34. * 3.);
}

#[test]
fn measure_3d_scalar_multiplication_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 23., 34.);
    m *= 3.;
    assert_eq!(m.x, 12. * 3.);
    assert_eq!(m.y, 23. * 3.);
    assert_eq!(m.z, 34. * 3.);
}

#[test]
fn measure_3d_scalar_division() {
    let m: Measure3d<Metre, f32> = Measure3d::<Metre, f32>::new(12., 23., 34.) / 3.;
    assert_eq_32!(m.x, 12. / 3.);
    assert_eq_32!(m.y, 23. / 3.);
    assert_eq_32!(m.z, 34. / 3.);
}

#[test]
fn measure_3d_scalar_division_assignment() {
    let mut m = Measure3d::<Metre, f32>::new(12., 23., 34.);
    m /= 3.;
    assert_eq_32!(m.x, 12. / 3.);
    assert_eq_32!(m.y, 23. / 3.);
    assert_eq_32!(m.z, 34. / 3.);
}

#[test]
fn measure_3d_equals() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m3 = Measure3d::<Metre, f32>::new(12.1, 23., 34.);
    let m4 = Measure3d::<Metre, f32>::new(12., 23.2, 34.);
    let m5 = Measure3d::<Metre, f32>::new(12., 23., 34.3);
    let m6 = Measure3d::<Metre, f32>::new(12.1, 23.2, 34.3);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
    assert!(!(m1 == m4));
    assert!(!(m1 == m5));
    assert!(!(m1 == m6));
}

#[test]
fn measure_3d_differs() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m3 = Measure3d::<Metre, f32>::new(12.1, 23., 34.);
    let m4 = Measure3d::<Metre, f32>::new(12., 23.2, 34.);
    let m5 = Measure3d::<Metre, f32>::new(12., 23., 34.3);
    let m6 = Measure3d::<Metre, f32>::new(12.1, 23.2, 34.3);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
    assert!(m1 != m4);
    assert!(m1 != m5);
    assert!(m1 != m6);
}

#[test]
fn measure_3d_is_equal_to_its_clone() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = m1.clone();
    assert!(m2 == m1);
}

#[test]
fn measure_3d_is_equal_to_its_copy() {
    let m1 = Measure3d::<Metre, f32>::new(12., 23., 34.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_3d_formatting_in_metres() {
    let m = Measure3d::<Metre, f32>::new(12.25, 23.50, 34.75);
    assert_eq!(format!("{}", m), "(12.25, 23.5, 34.75) m");
}

#[test]
fn measure_3d_formatting_in_metres_one_fractional_digit() {
    let m = Measure3d::<Metre, f32>::new(12.25, 23.50, 34.75);
    assert_eq!(format!("{:.1}", m), "(12.2, 23.5, 34.8) m");
}

#[test]
fn measure_3d_formatting_for_debug_in_metres() {
    let m = Measure3d::<Metre, f32>::new(12.25, 23.50, 34.75);
    assert_eq!(format!("{:?}", m), "(12.25, 23.5, 34.75) m");
}

#[test]
fn measure_3d_formatting_for_debug_in_metres_one_fractional_digit() {
    let m = Measure3d::<Metre, f32>::new(12.25, 23.50, 34.75);
    assert_eq!(format!("{:.1?}", m), "(12.2, 23.5, 34.8) m");
}
