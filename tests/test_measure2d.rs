use rs_measures::{define_measure1d, define_measure2d};
use std::f32::consts::TAU as TAU_f32;
use std::f64::consts::TAU;
define_measure1d! {}
define_measure2d! {}
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

#[macro_export]
macro_rules! assert_eq_f64 {
    ($expected:expr, $actual:expr) => {
        let e = $expected;
        let a = $actual;
        if if e >= a { e / a } else { a / e } - 1. > f64::EPSILON * 8.
            && if e >= a { e - a } else { a - e } > f64::EPSILON * 8.
        {
            panic!(
                "Expected: {}, Actual: {}, Difference: {}.",
                $expected,
                $actual,
                e - a,
            );
        }
    };
}

#[macro_export]
macro_rules! assert_eq_f32 {
    ($expected:expr, $actual:expr) => {
        let e = $expected;
        let a = $actual;
        if if e >= a { e / a } else { a / e } - 1. > f32::EPSILON * 8.
            && if e >= a { e - a } else { a - e } > f32::EPSILON * 8.
        {
            panic!(
                "Expected: {}, Actual: {}, Difference: {}.",
                $expected,
                $actual,
                e - a,
            );
        }
    };
}

// Measure2d

#[test]
fn measure_2d_value() {
    let m = Measure2d::<f32, Metre> {
        x: 12.3,
        y: 45.6,
        phantom: PhantomData,
    };
    assert_eq!(12.3, m.x);
    assert_eq!(45.6, m.y);
}

#[test]
fn measure_2d_new() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(12.3, m.x);
    assert_eq!(45.6, m.y);
}

#[test]
fn measure_2d_x() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(12.3, m.x().value);
}

#[test]
fn measure_2d_y() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(45.6, m.y().value);
}

#[test]
fn measure_2d_convert() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let km = m.convert::<MilliMetre>();
    assert_eq!(12300., km.x);
    assert_eq!(45600., km.y);
}

#[test]
fn measure_2d_lossless_into() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    // f64 <- f32
    let m2 = m1.lossless_into::<f64>();
    // f32 <- f32
    let m3 = m1.lossless_into::<f32>();
    // f64 <- f64
    let m4 = m2.lossless_into::<f64>();
    assert_eq!(12.3_f32, m1.x);
    assert_eq!(45.6_f32, m1.y);
    assert_eq!(m1.x as f64, m2.x);
    assert_eq!(m1.y as f64, m2.y);
    assert_eq!(m1.x, m3.x);
    assert_eq!(m1.y, m3.y);
    assert_eq!(m2.x, m4.x);
    assert_eq!(m2.y, m4.y);
}

#[test]
fn measure_2d_lossy_into() {
    let m1 = Measure2d::<f64, Metre>::new(12.3, 45.6);
    // f32 <- f64
    let m2 = m1.lossy_into::<f32>();
    // f64 <- f32
    let m3 = m2.lossy_into::<f64>();
    // f64 <- f64
    let m4 = m1.lossy_into::<f64>();
    // f32 <- f32
    let m5 = m2.lossy_into::<f32>();
    assert_eq!(12.3_f64, m1.x);
    assert_eq!(45.6_f64, m1.y);
    assert_eq!(12.3_f32, m2.x);
    assert_eq!(45.6_f32, m2.y);
    assert_eq!(m2.x as f64, m3.x);
    assert_eq!(m2.y as f64, m3.y);
    assert_eq!(m1.x, m4.x);
    assert_eq!(m1.y, m4.y);
    assert_eq!(m2.x, m5.x);
    assert_eq!(m2.y, m5.y);
}

#[test]
fn measure_2d_squared_norm() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(12.3 * 12.3 + 45.6 * 45.6, m.squared_norm());
}

#[test]
fn measure_2d_normalized() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = m1.normalized();
    assert_eq_f32!(1., m2.squared_norm());
    assert_eq_f32!(m1.y / m1.x, m2.y / m2.x);
}

#[test]
fn measure_2d_signed_direction() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let signed_dir = m.signed_direction::<Radian>();
    assert_eq_f32!((45.6_f32 / 12.3_f32).atan(), signed_dir.value);
}

#[test]
fn measure_2d_unsigned_direction() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let unsigned_dir = m.unsigned_direction::<Radian>();
    assert_eq_f32!((45.6_f32 / 12.3_f32).atan(), unsigned_dir.value);
}

#[test]
fn measure_2d_unary_minus() {
    let m = -Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(-12.3, m.x);
    assert_eq!(-45.6, m.y);
}

#[test]
fn measure_2d_multiplication() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = m1 * 5.3;
    assert_eq!(12.3, m1.x);
    assert_eq!(45.6, m1.y);
    assert_eq!(12.3 * 5.3, m2.x);
    assert_eq!(45.6 * 5.3, m2.y);
}

#[test]
fn measure_2d_multiplication_and_assignment() {
    let mut m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    m1 *= 5.3;
    assert_eq!(12.3 * 5.3, m1.x);
    assert_eq!(45.6 * 5.3, m1.y);
}

#[test]
fn measure_2d_division() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = m1 / 5.3;
    assert_eq!(12.3, m1.x);
    assert_eq!(45.6, m1.y);
    assert_eq!(12.3 / 5.3, m2.x);
    assert_eq!(45.6 / 5.3, m2.y);
}

#[test]
fn measure_2d_division_and_assignment() {
    let mut m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    m1 /= 5.3;
    assert_eq_f32!(12.3 / 5.3, m1.x);
    assert_eq_f32!(45.6 / 5.3, m1.y);
}

#[test]
fn measure_2d_addition() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = Measure2d::<f32, Metre>::new(32.1, 27.3);
    let m3 = m1 + m2;
    assert_eq!(12.3, m1.x);
    assert_eq!(45.6, m1.y);
    assert_eq!(32.1, m2.x);
    assert_eq!(27.3, m2.y);
    assert_eq_f32!(12.3 + 32.1, m3.x);
    assert_eq_f32!(45.6 + 27.3, m3.y);
}

#[test]
fn measure_2d_addition_and_assignment() {
    let mut m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = Measure2d::<f32, Metre>::new(32.1, 27.3);
    m1 += m2;
    assert_eq!(32.1, m2.x);
    assert_eq!(27.3, m2.y);
    assert_eq_f32!(12.3 + 32.1, m1.x);
    assert_eq_f32!(45.6 + 27.3, m1.y);
}

#[test]
fn measure_2d_subtraction() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = Measure2d::<f32, Metre>::new(32.1, 27.3);
    let m3 = m1 - m2;
    assert_eq!(12.3, m1.x);
    assert_eq!(45.6, m1.y);
    assert_eq!(32.1, m2.x);
    assert_eq!(27.3, m2.y);
    assert_eq_f32!(12.3 - 32.1, m3.x);
    assert_eq_f32!(45.6 - 27.3, m3.y);
}

#[test]
fn measure_2d_subtraction_and_assignment() {
    let mut m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = Measure2d::<f32, Metre>::new(32.1, 27.3);
    m1 -= m2;
    assert_eq!(32.1, m2.x);
    assert_eq!(27.3, m2.y);
    assert_eq_f32!(12.3 - 32.1, m1.x);
    assert_eq_f32!(45.6 - 27.3, m1.y);
}

#[test]
fn measure_2d_equals() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(12., 3.);
    let m3 = Measure2d::<f32, Metre>::new(12., 3.1);
    let m4 = Measure2d::<f32, Metre>::new(12.1, 3.);
    assert_eq!(true, m1 == m2);
    assert_eq!(false, m1 == m3);
    assert_eq!(false, m1 == m4);
}

#[test]
fn measure_2d_differs() {
    let m1 = Measure2d::<f32, Metre>::new(12., 3.);
    let m2 = Measure2d::<f32, Metre>::new(12., 3.);
    let m3 = Measure2d::<f32, Metre>::new(12., 3.1);
    let m4 = Measure2d::<f32, Metre>::new(12.1, 3.);
    assert_eq!(false, m1 != m2);
    assert_eq!(true, m1 != m3);
    assert_eq!(true, m1 != m4);
}

#[test]
fn measure_2d_cloning() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = m1.clone();
    assert_eq!(m1, m2);
}

#[test]
fn measure_2d_copying() {
    let m1 = Measure2d::<f32, Metre>::new(12.3, 45.6);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn measure_2d_formatting() {
    let m = Measure2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!("(12.3, 45.6) m", m.to_string());
}

// Measure point

#[test]
fn measure_point_2d_value() {
    let m = MeasurePoint2d::<f32, Metre> {
        x: 12.3,
        y: 45.6,
        phantom: PhantomData,
    };
    assert_eq!(12.3, m.x);
    assert_eq!(45.6, m.y);
}

#[test]
fn measure_point_2d_new() {
    let mp = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(12.3, mp.x);
    assert_eq!(45.6, mp.y);
}

#[test]
fn measure_point_2d_x() {
    let mp = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(12.3, mp.x().value);
}

#[test]
fn measure_point_2d_y() {
    let mp = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    assert_eq!(45.6, mp.y().value);
}

#[test]
fn measure_point_2d_convert() {
    let m = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    let km = m.convert::<MilliMetre>();
    assert_eq!(12300., km.x);
    assert_eq!(45600., km.y);
}

#[test]
fn measure_point_2d_lossless_into() {
    let m1 = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    // f64 <- f32
    let m2 = m1.lossless_into::<f64>();
    // f32 <- f32
    let m3 = m1.lossless_into::<f32>();
    // f64 <- f64
    let m4 = m2.lossless_into::<f64>();
    assert_eq!(12.3_f32, m1.x);
    assert_eq!(45.6_f32, m1.y);
    assert_eq!(m1.x as f64, m2.x);
    assert_eq!(m1.y as f64, m2.y);
    assert_eq!(m1.x, m3.x);
    assert_eq!(m1.y, m3.y);
    assert_eq!(m2.x, m4.x);
    assert_eq!(m2.y, m4.y);
}

#[test]
fn measure_point_2d_lossy_into() {
    let m1 = MeasurePoint2d::<f64, Metre>::new(12.3, 45.6);
    // f32 <- f64
    let m2 = m1.lossy_into::<f32>();
    // f64 <- f32
    let m3 = m2.lossy_into::<f64>();
    // f64 <- f64
    let m4 = m1.lossy_into::<f64>();
    // f32 <- f32
    let m5 = m2.lossy_into::<f32>();
    assert_eq!(12.3_f64, m1.x);
    assert_eq!(45.6_f64, m1.y);
    assert_eq!(12.3_f32, m2.x);
    assert_eq!(45.6_f32, m2.y);
    assert_eq!(m2.x as f64, m3.x);
    assert_eq!(m2.y as f64, m3.y);
    assert_eq!(m1.x, m4.x);
    assert_eq!(m1.y, m4.y);
    assert_eq!(m2.x, m5.x);
    assert_eq!(m2.y, m5.y);
}

#[test]
fn measure_point_2d_mapped_by() {
    let m1 = MeasurePoint2d::<f32, Metre>::new(12.3, 45.6);
    let am = AffineMap2d::<f32, Metre>::new([[4., 5., 6.], [7., 8., 9.]]);
    let m2 = am.apply_to(m1);
    assert_eq!(12.3 * 4. + 45.6 * 5. + 6., m2.x);
    assert_eq!(12.3 * 7. + 45.6 * 8. + 9., m2.y);
}

#[test]
fn measure_point_2d_addition_of_measure() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let m = Measure2d::<f32, Metre>::new(7., -29.);
    let mp2 = mp1 + m;
    assert_eq!(19., mp2.x);
    assert_eq!(-26., mp2.y);
}

#[test]
fn measure_point_2d_addition_of_measure_assignment() {
    let mut mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    mp += Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(19., mp.x);
    assert_eq!(-26., mp.y);
}

#[test]
fn measure_point_2d_subtraction_of_measure() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let m = Measure2d::<f32, Metre>::new(7., -29.);
    let mp2 = mp1 - m;
    assert_eq!(5., mp2.x);
    assert_eq!(32., mp2.y);
}

#[test]
fn measure_point_2d_subtraction_of_measure_assignment() {
    let mut mp = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    mp -= Measure2d::<f32, Metre>::new(7., -29.);
    assert_eq!(5., mp.x);
    assert_eq!(32., mp.y);
}

#[test]
fn measures_point_2d_subtraction_of_measure_point() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(7., -29.);
    let m = mp1 - mp2;
    assert_eq!(5., m.x);
    assert_eq!(32., m.y);
}

#[test]
fn measures_point_2d_weighted_midpoint() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(7., -29.);
    let weight_of_m2 = 0.6;
    let mid = mp1.weighted_midpoint(mp2, weight_of_m2);
    assert_eq!(mp1.x * 0.4 + mp2.x * 0.6, mid.x);
    assert_eq!(mp1.y * 0.4 + mp2.y * 0.6, mid.y);
}

#[test]
fn measures_point_2d_midpoint() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(7., -29.);
    let mid = mp1.midpoint(mp2);
    assert_eq!(mp1.x * 0.5 + mp2.x * 0.5, mid.x);
    assert_eq!(mp1.y * 0.5 + mp2.y * 0.5, mid.y);
}

#[test]
fn measures_point_2d_barycentric_combination() {
    let mp1 = MeasurePoint2d::<f32, Metre>::new(12., 3.);
    let mp2 = MeasurePoint2d::<f32, Metre>::new(7., -29.);
    let mp3 = MeasurePoint2d::<f32, Metre>::new(-4., 8.);
    let mp4 = MeasurePoint2d::<f32, Metre>::new(3.1, 6.9);
    let mid1 = barycentric_combination_2d(&[mp1], &[1.]);
    assert_eq!(mp1.x, mid1.x);
    assert_eq!(mp1.y, mid1.y);
    let mid2 = barycentric_combination_2d(&[mp1, mp2], &[0.4, 0.6]);
    assert_eq!(mp1.x * 0.4 + mp2.x * 0.6, mid2.x);
    assert_eq!(mp1.y * 0.4 + mp2.y * 0.6, mid2.y);
    let mid4 = barycentric_combination_2d(&[mp1, mp2, mp3, mp4], &[0.1, 0.4, 0.3, 0.2]);
    assert_eq!(
        mp1.x * 0.1 + mp2.x * 0.4 + mp3.x * 0.3 + mp4.x * 0.2,
        mid4.x
    );
    assert_eq!(
        mp1.y * 0.1 + mp2.y * 0.4 + mp3.y * 0.3 + mp4.y * 0.2,
        mid4.y
    );
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

// Unsigned direction

#[test]
fn unsigned_direction_value() {
    let ud = UnsignedDirection::<f32, Metre> {
        value: 12.3,
        phantom: PhantomData,
    };
    assert_eq!(12.3, ud.value);
}

#[test]
fn unsigned_direction_new() {
    let ud = UnsignedDirection::<f32, Radian>::new(1.23);
    assert_eq!(1.23, ud.value);
    let ud = UnsignedDirection::<f32, Radian>::new(0.);
    assert_eq!(0., ud.value);
    let ud = UnsignedDirection::<f32, Radian>::new(-0.1);
    assert_eq_f32!(TAU_f32 - 0.1, ud.value);
    let ud = UnsignedDirection::<f32, Radian>::new(TAU_f32 + 0.1);
    assert_eq_f32!(0.1, ud.value);
    let ud = UnsignedDirection::<f32, Radian>::new(TAU_f32);
    assert_eq!(0., ud.value);

    let ud = UnsignedDirection::<f32, Degree>::new(12.3);
    assert_eq!(12.3, ud.value);
    let ud = UnsignedDirection::<f32, Degree>::new(0.);
    assert_eq!(0., ud.value);
    let ud = UnsignedDirection::<f32, Degree>::new(-10.);
    assert_eq!(360. - 10., ud.value);
    let ud = UnsignedDirection::<f32, Degree>::new(360. + 10.);
    assert_eq!(10., ud.value);
    let ud = UnsignedDirection::<f32, Degree>::new(360.);
    assert_eq!(0., ud.value);
}

#[test]
fn unsigned_direction_to_measure_point() {
    let ud = UnsignedDirection::<f32, Radian>::new(1.23);
    let mp = ud.to_measure_point();
    assert_eq!(ud.value, mp.value);
}

#[test]
fn unsigned_direction_convert() {
    let ud_rad1 = UnsignedDirection::<f32, Radian>::new(1.23);
    let ud_deg = ud_rad1.convert::<Degree>();
    assert_eq_f32!(ud_rad1.value * 360. / TAU_f32, ud_deg.value);
    let ud_rad2 = ud_deg.convert::<Radian>();
    assert_eq_f32!(ud_deg.value * TAU_f32 / 360., ud_rad2.value);
}

#[test]
fn unsigned_direction_lossless_into() {
    let ud1 = UnsignedDirection::<f32, Radian>::new(1.23);
    // f64 <- f32
    let ud2 = ud1.lossless_into::<f64>();
    // f32 <- f32
    let ud3 = ud1.lossless_into::<f32>();
    // f64 <- f64
    let ud4 = ud2.lossless_into::<f64>();
    assert_eq_f32!(1.23_f32, ud1.value);
    assert_eq!(ud1.value as f64, ud2.value);
    assert_eq!(ud1.value, ud3.value);
    assert_eq!(ud2.value, ud4.value);
}

#[test]
fn unsigned_direction_lossy_into() {
    let ud1 = UnsignedDirection::<f64, Radian>::new(1.23);
    // f32 <- f64
    let ud2 = ud1.lossy_into::<f32>();
    // f64 <- f32
    let ud3 = ud2.lossy_into::<f64>();
    // f64 <- f64
    let ud4 = ud1.lossy_into::<f64>();
    // f32 <- f32
    let ud5 = ud2.lossy_into::<f32>();
    assert_eq!(1.23_f64, ud1.value);
    assert_eq!(1.23_f32, ud2.value);
    assert_eq!(ud2.value as f64, ud3.value);
    assert_eq!(ud1.value, ud4.value);
    assert_eq!(ud2.value, ud5.value);
}

#[test]
fn unsigned_direction_addition_of_measure() {
    let mp1 = UnsignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    let mp2 = mp1 + m;
    assert_eq!(12., mp1.value);
    assert_eq!(7., m.value);
    assert_eq!(19., mp2.value);
}

#[test]
fn unsigned_direction_addition_of_measure_assignment() {
    let mut mp = UnsignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    mp += m;
    assert_eq!(7., m.value);
    assert_eq!(19., mp.value);
}

#[test]
fn unsigned_direction_subtraction_of_measure() {
    let mp1 = UnsignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    let mp2 = mp1 - m;
    assert_eq!(12., mp1.value);
    assert_eq!(7., m.value);
    assert_eq!(5., mp2.value);
}

#[test]
fn unsigned_direction_subtraction_of_measure_assignment() {
    let mut mp = UnsignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    mp -= m;
    assert_eq!(7., m.value);
    assert_eq!(5., mp.value);
}

#[test]
fn unsigned_direction_subtraction_of_unsigned_direction() {
    let ud1 = UnsignedDirection::<f32, Degree>::new(12.);
    let ud2 = UnsignedDirection::<f32, Degree>::new(7.);
    let ud3 = UnsignedDirection::<f32, Degree>::new(200.);
    let m1 = ud1 - ud2;
    assert_eq!(5., m1.value);
    let m2 = ud2 - ud1;
    assert_eq!(-5., m2.value);
    let m3 = ud2 - ud3;
    assert_eq!(167., m3.value);
}

#[test]
fn unsigned_direction_equals() {
    let m1 = UnsignedDirection::<f32, Degree>::new(12.);
    let m2 = UnsignedDirection::<f32, Degree>::new(12.);
    let m3 = UnsignedDirection::<f32, Degree>::new(12.1);
    assert_eq!(true, m1 == m2);
    assert_eq!(false, m1 == m3);
}

#[test]
fn unsigned_direction_differs() {
    let m1 = UnsignedDirection::<f32, Degree>::new(12.);
    let m2 = UnsignedDirection::<f32, Degree>::new(12.);
    let m3 = UnsignedDirection::<f32, Degree>::new(12.1);
    assert_eq!(false, m1 != m2);
    assert_eq!(true, m1 != m3);
}

#[test]
fn unsigned_direction_ord() {
    let m1 = UnsignedDirection::<f32, Degree>::new(-12.);
    let m2 = UnsignedDirection::<f32, Degree>::new(12.);
    let m3 = UnsignedDirection::<f32, Degree>::new(200.);
    assert_eq!(false, m1 < m2);
    assert_eq!(false, m1 <= m2);
    assert_eq!(true, m1 > m2);
    assert_eq!(true, m1 >= m2);
    assert_eq!(false, m1 < m3);
    assert_eq!(false, m1 <= m3);
    assert_eq!(true, m1 > m3);
    assert_eq!(true, m1 >= m3);
    assert_eq!(false, m1 < m1);
    assert_eq!(true, m1 <= m1);
    assert_eq!(false, m1 > m1);
    assert_eq!(true, m1 >= m1);
}

#[test]
fn unsigned_direction_cloning() {
    let m1 = UnsignedDirection::<f32, Degree>::new(12.3);
    let m2 = m1.clone();
    assert_eq!(m1, m2);
}

#[test]
fn unsigned_direction_copying() {
    let m1 = UnsignedDirection::<f32, Degree>::new(12.3);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn unsigned_direction_formatting() {
    let m = UnsignedDirection::<f32, Degree>::new(12.);
    assert_eq!("at 12 deg (in 0°-360°)", m.to_string());
}

// Signed directions

#[test]
fn signed_direction_value() {
    let sd = SignedDirection::<f32, Metre> {
        value: 12.3,
        phantom: PhantomData,
    };
    assert_eq!(12.3, sd.value);
}

#[test]
fn signed_direction_new() {
    let sd = SignedDirection::<f32, Radian>::new(1.23);
    assert_eq_f32!(1.23, sd.value);
    let sd = SignedDirection::<f32, Radian>::new(0.);
    assert_eq!(0., sd.value);
    let sd = SignedDirection::<f32, Radian>::new(-0.1);
    assert_eq_f32!(TAU_f32 - 0.1, sd.value);
    let sd = SignedDirection::<f32, Radian>::new(TAU_f32 + 0.1);
    assert_eq_f32!(0.1, sd.value);
    let sd = SignedDirection::<f32, Radian>::new(TAU_f32);
    assert_eq_f32!(0., sd.value);

    let sd = SignedDirection::<f32, Degree>::new(12.3);
    assert_eq_f32!(12.3, sd.value);
    let sd = SignedDirection::<f32, Degree>::new(0.);
    assert_eq!(0., sd.value);
    let sd = SignedDirection::<f32, Degree>::new(-10.);
    assert_eq_f32!(360. - 10., sd.value);
    let sd = SignedDirection::<f32, Degree>::new(360. + 10.);
    assert_eq!(10., sd.value);
    let sd = SignedDirection::<f32, Degree>::new(360.);
    assert_eq!(0., sd.value);
}

#[test]
fn signed_direction_to_measure_point() {
    let sd = SignedDirection::<f32, Radian>::new(1.23);
    let mp = sd.to_measure_point();
    assert_eq!(sd.value, mp.value);
}

#[test]
fn signed_direction_convert() {
    let sd_rad1 = SignedDirection::<f32, Radian>::new(1.23);
    let sd_deg = sd_rad1.convert::<Degree>();
    assert_eq_f32!(sd_rad1.value * 360. / TAU_f32, sd_deg.value);
    let sd_rad2 = sd_deg.convert::<Radian>();
    assert_eq_f32!(sd_deg.value * TAU_f32 / 360., sd_rad2.value);
}

#[test]
fn signed_direction_lossless_into() {
    let sd1 = SignedDirection::<f32, Radian>::new(1.23);
    // f64 <- f32
    let sd2 = sd1.lossless_into::<f64>();
    // f32 <- f32
    let sd3 = sd1.lossless_into::<f32>();
    // f64 <- f64
    let sd4 = sd2.lossless_into::<f64>();
    assert_eq_f32!(1.23_f32, sd1.value);
    assert_eq!(sd1.value as f64, sd2.value);
    assert_eq!(sd1.value, sd3.value);
    assert_eq!(sd2.value, sd4.value);
}

#[test]
fn signed_direction_lossy_into() {
    let sd1 = SignedDirection::<f64, Radian>::new(1.23);
    // f32 <- f64
    let sd2 = sd1.lossy_into::<f32>();
    // f64 <- f32
    let sd3 = sd2.lossy_into::<f64>();
    // f64 <- f64
    let sd4 = sd1.lossy_into::<f64>();
    // f32 <- f32
    let sd5 = sd2.lossy_into::<f32>();
    assert_eq_f64!(1.23_f64, sd1.value);
    assert_eq!(1.23_f32, sd2.value);
    assert_eq!(sd2.value as f64, sd3.value);
    assert_eq!(sd1.value, sd4.value);
    assert_eq!(sd2.value, sd5.value);
}

#[test]
fn signed_direction_addition_of_measure() {
    let mp1 = SignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    let mp2 = mp1 + m;
    assert_eq!(12., mp1.value);
    assert_eq!(7., m.value);
    assert_eq!(19., mp2.value);
}

#[test]
fn signed_direction_addition_of_measure_assignment() {
    let mut mp = SignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    mp += m;
    assert_eq!(7., m.value);
    assert_eq!(19., mp.value);
}

#[test]
fn signed_direction_subtraction_of_measure() {
    let mp1 = SignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    let mp2 = mp1 - m;
    assert_eq!(12., mp1.value);
    assert_eq!(7., m.value);
    assert_eq!(5., mp2.value);
}

#[test]
fn signed_direction_subtraction_of_measure_assignment() {
    let mut mp = SignedDirection::<f32, Degree>::new(12.);
    let m = Measure::<f32, Degree>::new(7.);
    mp -= m;
    assert_eq!(7., m.value);
    assert_eq!(5., mp.value);
}

#[test]
fn signed_direction_subtraction_of_signed_direction() {
    let sd1 = SignedDirection::<f32, Degree>::new(12.);
    let sd2 = SignedDirection::<f32, Degree>::new(7.);
    let sd3 = SignedDirection::<f32, Degree>::new(200.);
    let m1 = sd1 - sd2;
    assert_eq!(5., m1.value);
    let m2 = sd2 - sd1;
    assert_eq!(-5., m2.value);
    let m3 = sd2 - sd3;
    assert_eq!(167., m3.value);
}

#[test]
fn signed_direction_equals() {
    let m1 = SignedDirection::<f32, Degree>::new(12.);
    let m2 = SignedDirection::<f32, Degree>::new(12.);
    let m3 = SignedDirection::<f32, Degree>::new(12.1);
    assert_eq!(true, m1 == m2);
    assert_eq!(false, m1 == m3);
}

#[test]
fn signed_direction_differs() {
    let m1 = SignedDirection::<f32, Degree>::new(12.);
    let m2 = SignedDirection::<f32, Degree>::new(12.);
    let m3 = SignedDirection::<f32, Degree>::new(12.1);
    assert_eq!(false, m1 != m2);
    assert_eq!(true, m1 != m3);
}

#[test]
fn signed_direction_ord() {
    let m1 = SignedDirection::<f32, Degree>::new(-12.);
    let m2 = SignedDirection::<f32, Degree>::new(12.);
    let m3 = SignedDirection::<f32, Degree>::new(200.);
    assert_eq!(true, m1 < m2);
    assert_eq!(true, m1 <= m2);
    assert_eq!(false, m1 > m2);
    assert_eq!(false, m1 >= m2);
    assert_eq!(false, m1 < m3);
    assert_eq!(false, m1 <= m3);
    assert_eq!(true, m1 > m3);
    assert_eq!(true, m1 >= m3);
    assert_eq!(false, m1 < m1);
    assert_eq!(true, m1 <= m1);
    assert_eq!(false, m1 > m1);
    assert_eq!(true, m1 >= m1);
}

#[test]
fn signed_direction_cloning() {
    let m1 = SignedDirection::<f32, Degree>::new(12.3);
    let m2 = m1.clone();
    assert_eq!(m1, m2);
}

#[test]
fn signed_direction_copying() {
    let m1 = SignedDirection::<f32, Degree>::new(12.3);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn signed_direction_formatting() {
    let m = SignedDirection::<f32, Degree>::new(12.);
    assert_eq!("at 12 deg (in -180°-180°)", m.to_string());
}

// Linear map

#[test]
fn linear_map_2d_values() {
    let lm = LinearMap2d::<f32> {
        c: [[1.2, 2.3], [3.4, 7.8]],
    };
    assert_eq!(1.2, lm.c[0][0]);
    assert_eq!(2.3, lm.c[0][1]);
    assert_eq!(3.4, lm.c[1][0]);
    assert_eq!(7.8, lm.c[1][1]);
}

#[test]
fn linear_map_2d_new() {
    let lm = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    assert_eq!(1.2, lm.c[0][0]);
    assert_eq!(2.3, lm.c[0][1]);
    assert_eq!(3.4, lm.c[1][0]);
    assert_eq!(7.8, lm.c[1][1]);
}

#[test]
fn linear_map_2d_rotation() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation(Measure::<f32, Degree>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(-5., m2.x);
    assert_eq_f32!(8., m2.y);
}

#[test]
fn linear_map_2d_rotation_at_right() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation_at_right();
    let m2 = lm.apply_to(m1);
    assert_eq!(5., m2.x);
    assert_eq!(-8., m2.y);
}

#[test]
fn linear_map_2d_rotation_at_left() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation_at_left();
    let m2 = lm.apply_to(m1);
    assert_eq!(-5., m2.x);
    assert_eq!(8., m2.y);
}

#[test]
fn linear_map_2d_projection_by_angle_point() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_angle_point(MeasurePoint::<f32, Degree>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(0., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_projection_by_signed_direction() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_signed_direction(
        SignedDirection::<f32, Degree>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(0., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_unsigned_direction() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_unsigned_direction(
        UnsignedDirection::<f32, Degree>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(0., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_projection_by_unit_vector() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let unit_vector = Measure2d::<f32, Metre>::new(0., 1.);
    assert_eq!(1., unit_vector.squared_norm());
    let lm = LinearMap2d::<f32>::projection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(0., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_reflection_by_angle_point() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_angle_point(MeasurePoint::<f32, Degree>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(-8., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_reflection_by_signed_direction() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_signed_direction(
        SignedDirection::<f32, Degree>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(-8., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_reflection_by_unsigned_direction() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_unsigned_direction(
        UnsignedDirection::<f32, Degree>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(-8., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_reflection_by_unit_vector() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let unit_vector = Measure2d::<f32, Metre>::new(0., 1.);
    assert_eq!(1., unit_vector.squared_norm());
    let lm = LinearMap2d::<f32>::reflection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_f32!(-8., m2.x);
    assert_eq_f32!(5., m2.y);
}

#[test]
fn linear_map_2d_scaling() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm = LinearMap2d::<f32>::scaling(3., 7.);
    let m2 = lm.apply_to(m1);
    assert_eq!(8. * 3., m2.x);
    assert_eq!(5. * 7., m2.y);
}

#[test]
fn linear_map_2d_inverted() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm1 = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    let lm2 = lm1.inverted();
    let m2 = lm1.apply_to(m1);
    let m3 = lm2.apply_to(m2);
    assert_eq_f32!(m1.x, m3.x);
    assert_eq_f32!(m1.y, m3.y);
}

#[test]
fn linear_map_2d_apply_after() {
    let m1 = Measure2d::<f32, Metre>::new(8., 5.);
    let lm1 = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    let lm2 = lm1.inverted();
    let lm3 = lm1.apply_after(&lm2);
    let m2 = lm3.apply_to(m1);
    assert_eq_f32!(m1.x, m2.x);
    assert_eq_f32!(m1.y, m2.y);
}

/*
// Affine map

#[test]
fn affine_map_2d_values() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_new() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_convert() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_translation() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_rotation() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_rotation_at_right() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_rotation_at_left() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_projection_by_angle_point() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_projection_by_signed_direction() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_projection_by_unsigned_direction() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_projection_by_unit_vector() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_reflection_by_angle_point() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_reflection_by_signed_direction() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_reflection_by_unsigned_direction() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_reflection_by_unit_vector() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_scaling() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_inverted() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}

#[test]
fn affine_map_2d_apply_after() {
    let am = AffineMap2d::<f32, Metre>::new(12.);
    assert_eq!(exp, am);
}
*/
