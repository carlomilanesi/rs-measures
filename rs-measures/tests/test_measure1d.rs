use rs_measures::define_measure_1d;
define_measure_1d! {}
struct Length;

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Property = Length;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}

mod angles_decl;

#[test]
fn angle_units_consts() {
    use rs_measures::{angle::Radian, traits::AngleMeasurementUnit};
    let pi = std::f64::consts::PI;

    // Radians
    assert_eq!(Radian::TURN_FRACTION, 2. * pi);
    assert_eq!(Radian::RATIO, 1.);
    assert_eq!(Radian::OFFSET, 0.);

    // Degrees
    assert_eq!(angles_decl::Degree::TURN_FRACTION, 360.);
    assert_eq!(angles_decl::Degree::RATIO, 2. * pi / 360.);
    assert_eq!(angles_decl::Degree::OFFSET, 0.);

    // Turns
    assert_eq!(angles_decl::Turn::TURN_FRACTION, 1.);
    assert_eq!(angles_decl::Turn::RATIO, 2. * pi);
    assert_eq!(angles_decl::Turn::OFFSET, 0.);
}

// Measure

#[test]
fn measure_value() {
    let m: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.);
    assert_eq!(m.value, 12.);
}

#[test]
fn measure_convert() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<KiloMetre, f32> = m1.convert::<KiloMetre>();
    assert_eq!(m2.value, 0.012);
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
fn measure_squared_norm() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let n1: f32 = m1.squared_norm();
    assert_eq!(n1, 12. * 12.);
    let m2 = Measure::<Metre, f64>::new(-12.);
    let n2: f64 = m2.squared_norm();
    assert_eq!(n2, 12. * 12.);
}

#[test]
fn measure_normalized() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2: Measure<Metre, f32> = m1.normalized();
    assert_eq!(m2.value, 1.);
    let m3 = Measure::<Metre, f64>::new(-12.);
    let m4: Measure<Metre, f64> = m3.normalized();
    assert_eq!(m4.value, -1.);
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
fn measure_scalar_multiplication() {
    let m1: Measure<Metre, f32> = Measure::<Metre, f32>::new(12.) * 3.;
    assert_eq!(m1.value, 36.);
    let m2: Measure<Metre, f32> = 3. * Measure::<Metre, f32>::new(12.);
    assert_eq!(m2.value, 36.);
    let m3: Measure<Metre, f64> = Measure::<Metre, f64>::new(12.) * 3.;
    assert_eq!(m3.value, 36.);
    let m4: Measure<Metre, f64> = 3. * Measure::<Metre, f64>::new(12.);
    assert_eq!(m4.value, 36.);
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
fn measure_is_less_than() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(!(m1 < m2));
    assert!(m1 < m3);
    assert!(!(m3 < m1));
}

#[test]
fn measure_is_less_than_or_equal_to() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(m1 <= m2);
    assert!(m1 <= m3);
    assert!(!(m3 <= m1));
}

#[test]
fn measure_is_greater_than() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(!(m1 > m2));
    assert!(!(m1 > m3));
    assert!(m3 > m1);
}

#[test]
fn measure_is_greater_than_or_equal_to() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = Measure::<Metre, f32>::new(12.);
    let m3 = Measure::<Metre, f32>::new(13.);
    assert!(m1 >= m2);
    assert!(!(m1 >= m3));
    assert!(m3 >= m1);
}

#[test]
fn measure_is_equal_to_its_clone() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = m1.clone();
    assert!(m2 == m1);
}

#[test]
fn measure_is_equal_to_its_copy() {
    let m1 = Measure::<Metre, f32>::new(12.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m2 == m1);
}

#[test]
fn measure_shown_in_metres() {
    let m1 = Measure::<Metre, f32>::new(12.);
    assert_eq!(m1.to_string(), "12 m");
}

// MeasurePoint

#[test]
fn measure_point_value() {
    let mp = MeasurePoint::<Metre, f32>::new(12.);
    assert_eq!(mp.value, 12.);
}

#[test]
fn measure_point_convert() {
    let m1 = MeasurePoint::<Metre, f32>::new(12.);
    let m2: MeasurePoint<KiloMetre, f32> = m1.convert::<KiloMetre>();
    assert_eq!(m2.value, 0.012);
}

#[test]
fn measure_point_lossless_into_32_to_32() {
    let m1 = MeasurePoint::<Metre, f32>::new(12.);
    let m2: MeasurePoint<Metre, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossless_into_32_to_64() {
    let m1 = MeasurePoint::<Metre, f32>::new(12.);
    let m2: MeasurePoint<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossless_into_64_to_64() {
    let m1 = MeasurePoint::<Metre, f64>::new(12.);
    let m2: MeasurePoint<Metre, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossy_into_32_to_32() {
    let m1 = MeasurePoint::<Metre, f32>::new(12.);
    let m2: MeasurePoint<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossy_into_32_to_64() {
    let m1 = MeasurePoint::<Metre, f32>::new(12.);
    let m2: MeasurePoint<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossy_into_64_to_32() {
    let m1 = MeasurePoint::<Metre, f64>::new(12.);
    let m2: MeasurePoint<Metre, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_lossy_into_64_to_64() {
    let m1 = MeasurePoint::<Metre, f64>::new(12.);
    let m2: MeasurePoint<Metre, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn measure_point_addition_of_vector() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let m = Measure::<Metre, f32>::new(7.);
    let mp2: MeasurePoint<Metre, f32> = mp1 + m;
    assert_eq!(mp2.value, 19.);
}

#[test]
fn measure_point_addition_of_vector_assignment() {
    let mut mp = MeasurePoint::<Metre, f32>::new(12.);
    mp += Measure::<Metre, f32>::new(7.);
    assert_eq!(mp.value, 19.);
}

#[test]
fn measure_point_subtraction_of_vector() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let m = Measure::<Metre, f32>::new(7.);
    let mp2: MeasurePoint<Metre, f32> = mp1 - m;
    assert_eq!(mp2.value, 5.);
}

#[test]
fn measure_point_subtraction_of_vector_assignment() {
    let mut mp = MeasurePoint::<Metre, f32>::new(12.);
    mp -= Measure::<Metre, f32>::new(7.);
    assert_eq!(mp.value, 5.);
}

#[test]
fn measures_point_subtraction() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(7.);
    let m: Measure<Metre, f32> = mp1 - mp2;
    assert_eq!(m.value, 5.);
}

#[test]
fn measure_point_equals() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));
}

#[test]
fn measure_point_differs() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn measure_point_is_less_than() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(!(mp1 < mp2));
    assert!(mp1 < mp3);
    assert!(!(mp3 < mp1));
}

#[test]
fn measure_point_is_less_than_or_equal_to() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(mp1 <= mp2);
    assert!(mp1 <= mp3);
    assert!(!(mp3 <= mp1));
}

#[test]
fn measure_point_is_greater_than() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(!(mp1 > mp2));
    assert!(!(mp1 > mp3));
    assert!(mp3 > mp1);
}

#[test]
fn measure_point_is_greater_than_or_equal_to() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = MeasurePoint::<Metre, f32>::new(12.);
    let mp3 = MeasurePoint::<Metre, f32>::new(13.);
    assert!(mp1 >= mp2);
    assert!(!(mp1 >= mp3));
    assert!(mp3 >= mp1);
}

#[test]
fn measure_point_is_equal_to_its_clone() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = mp1.clone();
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_is_equal_to_its_copy() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_shown_in_metres() {
    let mp1 = MeasurePoint::<Metre, f32>::new(12.);
    assert_eq!(mp1.to_string(), "at 12 m");
}

/*
/// ```
/// use rs_measures::define_measure_1d;
/// define_measure_1d! {}
/// struct Mass;
/// struct KiloGram;
/// impl MeasurementUnit for KiloGram {
///     type Property = Mass;
///     const RATIO: f64 = 1.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " kg";
/// }
/// struct Time;
/// struct Second;
/// impl MeasurementUnit for Second {
///     type Property = Time;
///     const RATIO: f64 = 1.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " s";
/// }
/// struct Hour;
/// impl MeasurementUnit for Hour {
///     type Property = Time;
///     const RATIO: f64 = 3600.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " h";
/// }
/// let m = Measure::<Second, f32>::new(1.);
/// let _ = m.convert::<Hour>();
/// ```
///
/// ```
/// use rs_measures::define_measure_1d;
/// define_measure_1d! {}
/// struct Mass;
/// struct KiloGram;
/// impl MeasurementUnit for KiloGram {
///     type Property = Mass;
///     const RATIO: f64 = 1.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " kg";
/// }
/// struct Time;
/// struct Second;
/// impl MeasurementUnit for Second {
///     type Property = Time;
///     const RATIO: f64 = 1.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " s";
/// }
/// struct Hour;
/// impl MeasurementUnit for Hour {
///     type Property = Time;
///     const RATIO: f64 = 3600.;
///     const OFFSET: f64 = 0.;
///     const SUFFIX: &'static str = " h";
/// }
/// let m = Measure::<Second, f32>::new(1.);
/// let _ = m.convert::<KiloGram>();
/// ```
#[test]
fn ff001() {}

#[test]
fn ui() {
    /*
    let mut threads = vec![];
    for i in 0..3 {
        let t = trybuild::TestCases::new();
        threads.push(std::thread::spawn(move || {
            t.compile_fail(format!("tests/ui/measure0{}*.rs", i));
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
     */
    //let t = trybuild::TestCases::new();
    //t.compile_fail("tests/ui/measure00*.rs");
}
 */
