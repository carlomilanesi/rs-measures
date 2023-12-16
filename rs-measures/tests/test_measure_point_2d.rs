use std::f64::consts::TAU;
rs_measures::define_measure_2d! {}

struct Length;

#[derive(Debug)]
struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

#[derive(Debug)]
struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 0.001;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}
impl VectorMeasurementUnit for MilliMetre {}

#[derive(Debug)]
struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}

#[test]
fn measure_point_2d_value() {
    let m = MeasurePoint2d::<Metre, f32> {
        x: 12.3,
        y: 45.6,
        phantom: PhantomData,
    };
    assert_eq!(m.x, 12.3);
    assert_eq!(m.y, 45.6);
}

#[test]
fn measure_point_2d_xy_fields() {
    let m: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    assert_eq!(m.x, 12.);
    assert_eq!(m.y, 23.);
}

#[test]
fn measure_point_2d_xy_functions() {
    let m: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mx: MeasurePoint<Metre, f32> = m.x();
    let my: MeasurePoint<Metre, f32> = m.y();
    assert_eq!(mx.value, 12.);
    assert_eq!(my.value, 23.);
}

#[test]
fn measure_point_2d_convert() {
    let m1: MeasurePoint2d<Metre, f32> = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let m2: MeasurePoint2d<MilliMetre, f32> = m1.convert::<MilliMetre>();
    assert_eq!(m1.x, 12.);
    assert_eq!(m1.y, 23.);
    assert_eq!(m2.x, 12000.);
    assert_eq!(m2.y, 23000.);
}

#[test]
fn measure_point_2d_lossless_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossless_into::<f32>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossless_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossless_into::<f64>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossless_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossless_into::<f64>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossy_into_32_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossy_into_32_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossy_into_64_to_32() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

#[test]
fn measure_point_2d_lossy_into_64_to_64() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new(12., 23.);
    let mp2: MeasurePoint2d<Metre, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.x, 12.);
    assert_eq!(mp2.y, 23.);
}

/*
#[test]
fn measure_point_2d_mapped_by() {
    let m1 = MeasurePoint2d::<Metre, f32>::new(12.3, 45.6);
    let am = AffineMap2d::<Metre, f32>::new([[4., 5., 6.], [7., 8., 9.]]);
    let m2 = am.apply_to(m1);
    assert_eq!(m2.x, 12.3 * 4. + 45.6 * 5. + 6.);
    assert_eq!(m2.y, 12.3 * 7. + 45.6 * 8. + 9.);
}
*/

#[test]
fn measure_point_2d_addition_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let m = Measure2d::<Metre, f32>::new(34., -45.);
    let mp2 = mp1 + m;
    assert_eq!(mp2.x, 12. + 34.);
    assert_eq!(mp2.y, 23. + -45.);
}

#[test]
fn measure_point_2d_addition_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let m = Measure2d::<Metre, f32>::new(34., -45.);
    mp += m;
    assert_eq!(mp.x, 12. + 34.);
    assert_eq!(mp.y, 23. + -45.);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let m = Measure2d::<Metre, f32>::new(34., -45.);
    let mp2 = mp1 - m;
    assert_eq!(mp2.x, 12. - 34.);
    assert_eq!(mp2.y, 23. - -45.);
}

#[test]
fn measure_point_2d_subtraction_of_measure_2d_assignment() {
    let mut mp = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let m = Measure2d::<Metre, f32>::new(34., -45.);
    mp -= m;
    assert_eq!(mp.x, 12. - 34.);
    assert_eq!(mp.y, 23. - -45.);
}

#[test]
fn measures_point_2d_subtraction() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 23.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(34., -45.);
    let m: Measure2d<Metre, f32> = mp1 - mp2;
    assert_eq!(m.x, 12. - 34.);
    assert_eq!(m.y, 23. - -45.);
}

//CONTINUED
#[test]
fn measures_point_2d_weighted_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(7., -29.);
    let weight_of_m2 = 0.6;
    let mid = mp1.weighted_midpoint(mp2, weight_of_m2);
    assert_eq!(mid.x, mp1.x * 0.4 + mp2.x * 0.6);
    assert_eq!(mid.y, mp1.y * 0.4 + mp2.y * 0.6);
}

#[test]
fn measures_point_2d_midpoint() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(7., -29.);
    let mid = mp1.midpoint(mp2);
    assert_eq!(mid.x, mp1.x * 0.5 + mp2.x * 0.5);
    assert_eq!(mid.y, mp1.y * 0.5 + mp2.y * 0.5);
}

#[test]
fn measures_point_2d_barycentric_combination() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(7., -29.);
    let mp3 = MeasurePoint2d::<Metre, f32>::new(-4., 8.);
    let mp4 = MeasurePoint2d::<Metre, f32>::new(3.1, 6.9);
    let mid1 = barycentric_combination_2d(&[mp1], &[1.]);
    assert_eq!(mid1.x, mp1.x);
    assert_eq!(mid1.y, mp1.y);
    let mid2 = barycentric_combination_2d(&[mp1, mp2], &[0.4, 0.6]);
    assert_eq!(mid2.x, mp1.x * 0.4 + mp2.x * 0.6);
    assert_eq!(mid2.y, mp1.y * 0.4 + mp2.y * 0.6);
    let mid4 = barycentric_combination_2d(&[mp1, mp2, mp3, mp4], &[0.1, 0.4, 0.3, 0.2]);
    assert_eq!(
        mid4.x,
        mp1.x * 0.1 + mp2.x * 0.4 + mp3.x * 0.3 + mp4.x * 0.2,
    );
    assert_eq!(
        mid4.y,
        mp1.y * 0.1 + mp2.y * 0.4 + mp3.y * 0.3 + mp4.y * 0.2,
    );
}

#[test]
fn measure_point_2d_equals() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp3 = MeasurePoint2d::<Metre, f32>::new(13., -29.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));
}

#[test]
fn measure_point_2d_differs() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp3 = MeasurePoint2d::<Metre, f32>::new(13., -29.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn measure_point_2d_is_equal_to_its_clone() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = mp1.clone();
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_2d_is_equal_to_its_copy() {
    let mp1 = MeasurePoint2d::<Metre, f32>::new(12., 3.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_2d_shown_in_metres() {
    let mp = MeasurePoint2d::<Metre, f32>::new(12.25, 23.50);
    assert_eq!(mp.to_string(), "at (12.25, 23.5) m");
}
