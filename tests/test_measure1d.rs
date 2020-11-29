use rs_measures::define_measure1d;
define_measure1d! {}
struct Length;
struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

mod angles_decl;

#[test]
fn angle_units_consts() {
    use rs_measures::{angle::Radian, traits::AngleMeasurementUnit};
    let pi = 3.141592653589793238462643383279;

    // Radians
    assert_eq!(2. * pi, Radian::TURN_FRACTION);
    assert_eq!(1., Radian::RATIO);
    assert_eq!(0., Radian::OFFSET);

    // Degrees
    assert_eq!(360., angles_decl::Degree::TURN_FRACTION);
    assert_eq!(2. * pi / 360., angles_decl::Degree::RATIO);
    assert_eq!(0., angles_decl::Degree::OFFSET);

    // Turns
    assert_eq!(1., angles_decl::Turn::TURN_FRACTION);
    assert_eq!(2. * pi, angles_decl::Turn::RATIO);
    assert_eq!(0., angles_decl::Turn::OFFSET);
}

// Measure

#[test]
fn measure_value() {
    let m = Measure::<f32, Metre>::new(123.);
    assert_eq!(123., m.value);
}

#[test]
fn measure_unary_minus() {
    let m = -Measure::<f32, Metre>::new(12.);
    assert_eq!(-12., m.value);
}

#[test]
fn measures_addition() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(7.);
    assert_eq!(19., (m1 + m2).value);
}

#[test]
fn measures_addition_assignment() {
    let mut m = Measure::<f32, Metre>::new(12.);
    m += Measure::<f32, Metre>::new(7.);
    assert_eq!(19., m.value);
}

#[test]
fn measures_subtraction() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(7.);
    assert_eq!(5., (m1 - m2).value);
}

#[test]
fn measures_subtraction_assignment() {
    let mut m = Measure::<f32, Metre>::new(12.);
    m -= Measure::<f32, Metre>::new(7.);
    assert_eq!(5., m.value);
}

#[test]
fn measure_scalar_multiplication() {
    let m = Measure::<f32, Metre>::new(12.) * 3.;
    assert_eq!(36., m.value);
}

#[test]
fn measure_scalar_multiplication_assignment() {
    let mut m = Measure::<f32, Metre>::new(12.);
    m *= 3.;
    assert_eq!(36., m.value);
}

#[test]
fn measure_scalar_division() {
    let m = Measure::<f32, Metre>::new(12.) / 3.;
    assert_eq!(4., m.value);
}

#[test]
fn measure_scalar_division_assignment() {
    let mut m = Measure::<f32, Metre>::new(12.);
    m /= 3.;
    assert_eq!(4., m.value);
}

#[test]
fn measure_equals() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(m1 == m2);
    assert!(!(m1 == m3));
}

#[test]
fn measure_differ() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(!(m1 != m2));
    assert!(m1 != m3);
}

#[test]
fn measure_is_less_than() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(!(m1 < m2));
    assert!(m1 < m3);
    assert!(!(m3 < m1));
}

#[test]
fn measure_is_less_than_or_equal_to() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(m1 <= m2);
    assert!(m1 <= m3);
    assert!(!(m3 <= m1));
}

#[test]
fn measure_is_greater_than() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(!(m1 > m2));
    assert!(!(m1 > m3));
    assert!(m3 > m1);
}

#[test]
fn measure_is_greater_than_or_equal_to() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = Measure::<f32, Metre>::new(12.);
    let m3 = Measure::<f32, Metre>::new(13.);
    assert!(m1 >= m2);
    assert!(!(m1 >= m3));
    assert!(m3 >= m1);
}

#[test]
fn measure_is_equal_to_its_clone() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = m1.clone();
    assert!(m1 == m2);
}

#[test]
fn measure_is_equal_to_its_copy() {
    let m1 = Measure::<f32, Metre>::new(12.);
    let m2 = m1;
    let _ = m1; // Copy again
    assert!(m1 == m2);
}

#[test]
fn measure_shown_in_metres() {
    let m1 = Measure::<f32, Metre>::new(12.);
    assert_eq!("12 m", m1.to_string());
}

// MeasurePoint

#[test]
fn measure_point_value() {
    let mp = MeasurePoint::<f32, Metre>::new(123.);
    assert_eq!(123., mp.value);
}

#[test]
fn measure_point_addition_of_vector() {
    let mp = MeasurePoint::<f32, Metre>::new(12.);
    let m = Measure::<f32, Metre>::new(7.);
    assert_eq!(19., (mp + m).value);
}

#[test]
fn measure_point_addition_of_vector_assignment() {
    let mut mp = MeasurePoint::<f32, Metre>::new(12.);
    mp += Measure::<f32, Metre>::new(7.);
    assert_eq!(19., mp.value);
}

#[test]
fn measure_point_subtraction_of_vector() {
    let mp = MeasurePoint::<f32, Metre>::new(12.);
    let m = Measure::<f32, Metre>::new(7.);
    assert_eq!(5., (mp - m).value);
}

#[test]
fn measure_point_subtraction_of_vector_assignment() {
    let mut mp = MeasurePoint::<f32, Metre>::new(12.);
    mp -= Measure::<f32, Metre>::new(7.);
    assert_eq!(5., mp.value);
}

#[test]
fn measures_point_subtraction() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(7.);
    assert_eq!(5., (mp1 - mp2).value);
}

#[test]
fn measure_point_equals() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(mp1 == mp2);
    assert!(!(mp1 == mp3));
}

#[test]
fn measure_point_differs() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(!(mp1 != mp2));
    assert!(mp1 != mp3);
}

#[test]
fn measure_point_is_less_than() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(!(mp1 < mp2));
    assert!(mp1 < mp3);
    assert!(!(mp3 < mp1));
}

#[test]
fn measure_point_is_less_than_or_equal_to() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(mp1 <= mp2);
    assert!(mp1 <= mp3);
    assert!(!(mp3 <= mp1));
}

#[test]
fn measure_point_is_greater_than() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(!(mp1 > mp2));
    assert!(!(mp1 > mp3));
    assert!(mp3 > mp1);
}

#[test]
fn measure_point_is_greater_than_or_equal_to() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = MeasurePoint::<f32, Metre>::new(12.);
    let mp3 = MeasurePoint::<f32, Metre>::new(13.);
    assert!(mp1 >= mp2);
    assert!(!(mp1 >= mp3));
    assert!(mp3 >= mp1);
}

#[test]
fn measure_point_is_equal_to_its_clone() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = mp1.clone();
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_is_equal_to_its_copy() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    let mp2 = mp1;
    let _ = mp1; // Copy again
    assert!(mp1 == mp2);
}

#[test]
fn measure_point_shown_in_metres() {
    let mp1 = MeasurePoint::<f32, Metre>::new(12.);
    assert_eq!("at 12 m", mp1.to_string());
}
