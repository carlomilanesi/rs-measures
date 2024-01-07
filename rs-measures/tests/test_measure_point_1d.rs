rs_measures::define_measure_1d! {}

mod test_utils;

pub struct Temperature;

pub struct Celsius;
impl MeasurementUnit for Celsius {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 273.15;
    const SUFFIX: &'static str = " \u{B0}C";
}

pub struct Fahrenheit;
impl MeasurementUnit for Fahrenheit {
    type Property = Temperature;
    const RATIO: f64 = 5. / 9.;
    const OFFSET: f64 = 273.15 - 32. * 5. / 9.;
    const SUFFIX: &'static str = " \u{B0}F";
}

#[test]
fn measure_point_new() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.);
    assert_eq!(mp.value, 12.);
}

#[test]
fn measure_point_convert() {
    // 0 °C is 32 °F
    let mp1 = MeasurePoint::<Celsius, f32>::new(0.);
    let mp2: MeasurePoint<Fahrenheit, f32> = mp1.convert::<Fahrenheit>();
    assert_eq!(mp2.value, 32.);

    // 68 °F is 20 °C
    let mp3 = MeasurePoint::<Fahrenheit, f32>::new(68.);
    let mp4: MeasurePoint<Celsius, f32> = mp3.convert::<Celsius>();
    assert_eq!(mp4.value, 20.);
}

#[test]
fn measure_point_lossless_into_32_to_32() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2: MeasurePoint<Celsius, f32> = mp1.lossless_into::<f32>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossless_into_32_to_64() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2: MeasurePoint<Celsius, f64> = mp1.lossless_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossless_into_64_to_64() {
    let mp1 = MeasurePoint::<Celsius, f64>::new(12.);
    let mp2: MeasurePoint<Celsius, f64> = mp1.lossless_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossy_into_32_to_32() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2: MeasurePoint<Celsius, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossy_into_32_to_64() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2: MeasurePoint<Celsius, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossy_into_64_to_32() {
    let mp1 = MeasurePoint::<Celsius, f64>::new(12.);
    let mp2: MeasurePoint<Celsius, f32> = mp1.lossy_into::<f32>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_lossy_into_64_to_64() {
    let mp1 = MeasurePoint::<Celsius, f64>::new(12.);
    let mp2: MeasurePoint<Celsius, f64> = mp1.lossy_into::<f64>();
    assert_eq!(mp2.value, 12.);
}

#[test]
fn measure_point_addition_of_measure() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m = Measure::<Celsius, f32>::new(7.);
    let mp2: MeasurePoint<Celsius, f32> = mp1 + m;
    assert_eq!(mp2.value, 19.);
}

#[test]
fn measure_point_addition_of_measure_assignment() {
    let mut mp = MeasurePoint::<Celsius, f32>::new(12.);
    mp += Measure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 19.);
}

#[test]
fn measure_point_subtraction_of_measure() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let m = Measure::<Celsius, f32>::new(7.);
    let mp2: MeasurePoint<Celsius, f32> = mp1 - m;
    assert_eq!(mp2.value, 5.);
}

#[test]
fn measure_point_subtraction_of_measure_assignment() {
    let mut mp = MeasurePoint::<Celsius, f32>::new(12.);
    mp -= Measure::<Celsius, f32>::new(7.);
    assert_eq!(mp.value, 5.);
}

#[test]
fn measures_point_subtraction() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(7.);
    let m: Measure<Celsius, f32> = mp1 - mp2;
    assert_eq!(m.value, 5.);
}

#[test]
fn measures_point_weighted_midpoint() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3: MeasurePoint<Celsius, f32> = weighted_midpoint(mp1, mp2, 0.4);
    assert_eq_32!(mp3.value, 26.);
}

#[test]
fn measures_point_midpoint() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3: MeasurePoint<Celsius, f32> = midpoint(mp1, mp2);
    assert_eq_32!(mp3.value, 25.);
}

#[test]
fn measures_point_barycentric_combination() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(20.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(30.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(80.);
    let mp4: MeasurePoint<Celsius, f32> =
        barycentric_combination(&[mp1, mp2, mp3], &[0.1, 0.3, 0.7]);
    assert_eq_32!(mp4.value, 67.);
}

#[test]
fn measure_point_equals() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(mp2 == mp1);
    assert!(!(mp3 == mp1));
}

#[test]
fn measure_point_differs() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(!(mp2 != mp1));
    assert!(mp3 != mp1);
}

#[test]
fn measure_point_is_less_than() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(!(mp1 < mp2));
    assert!(mp1 < mp3);
    assert!(!(mp3 < mp1));
}

#[test]
fn measure_point_is_less_than_or_equal_to() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(mp1 <= mp2);
    assert!(mp1 <= mp3);
    assert!(!(mp3 <= mp1));
}

#[test]
fn measure_point_is_greater_than() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(!(mp1 > mp2));
    assert!(!(mp1 > mp3));
    assert!(mp3 > mp1);
}

#[test]
fn measure_point_is_greater_than_or_equal_to() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp3 = MeasurePoint::<Celsius, f32>::new(13.);
    assert!(mp1 >= mp2);
    assert!(!(mp1 >= mp3));
    assert!(mp3 >= mp1);
}

#[test]
fn measure_point_is_equal_to_its_clone() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = mp1.clone();
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_is_equal_to_its_copy() {
    let mp1 = MeasurePoint::<Celsius, f32>::new(12.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp2 == mp1);
}

#[test]
fn measure_point_shown_in_celsius() {
    let mp = MeasurePoint::<Celsius, f32>::new(12.25);
    assert_eq!(mp.to_string(), "at 12.25 °C");
}
