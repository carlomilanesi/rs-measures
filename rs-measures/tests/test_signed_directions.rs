use rs_measures::define_measure_1d;
define_measure_1d! {}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}

pub struct Turn;
impl MeasurementUnit for Turn {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Turn {
    const TURN_FRACTION: f64 = 1.;
}

#[test]
fn signed_direction_integer_turns() {
    let ud = SignedDirection::<Degree, f32>::new(3600.);
    assert_eq!(ud.value, 0.);
}

#[test]
fn signed_direction_small_positive_value() {
    let ud = SignedDirection::<Degree, f32>::new(10.);
    assert_eq!(ud.value, 10.);
}

#[test]
fn signed_direction_small_negative_value() {
    let ud = SignedDirection::<Degree, f32>::new(-10.);
    assert_eq!(ud.value, -10.);
}

#[test]
fn signed_direction_large_positive_value() {
    let ud = SignedDirection::<Degree, f32>::new(3604.);
    assert_eq!(ud.value, 4.);
}

#[test]
fn signed_direction_large_negative_value() {
    let ud = SignedDirection::<Degree, f32>::new(-3604.);
    assert_eq!(ud.value, -4.);
}

#[test]
fn signed_direction_from_small_zero_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(0.);
    let ud: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, 0.);
}

#[test]
fn signed_direction_from_small_positive_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(10.);
    let ud: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, 10.);
}

#[test]
fn signed_direction_from_small_negative_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(-10.);
    let ud: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, -10.);
}

#[test]
fn signed_direction_zero_to_measure_point() {
    let ud = SignedDirection::<Degree, f32>::new(0.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, 0.);
}

#[test]
fn signed_direction_small_to_measure_point() {
    let ud = SignedDirection::<Degree, f32>::new(10.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, 10.);
}

#[test]
fn signed_direction_negative_to_measure_point() {
    let ud = SignedDirection::<Degree, f32>::new(-10.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, -10.);
}

#[test]
fn signed_direction_zero_to_unsigned_direction() {
    let ud = SignedDirection::<Degree, f32>::new(0.);
    let sd: UnsignedDirection<Degree, f32> = ud.to_unsigned_direction();
    assert_eq!(sd.value, 0.);
}

#[test]
fn signed_direction_small_to_unsigned_direction() {
    let ud = SignedDirection::<Degree, f32>::new(10.);
    let sd: UnsignedDirection<Degree, f32> = ud.to_unsigned_direction();
    assert_eq!(sd.value, 10.);
}

#[test]
fn signed_direction_negative_to_unsigned_direction() {
    let ud = SignedDirection::<Degree, f32>::new(-10.);
    let sd: UnsignedDirection<Degree, f32> = ud.to_unsigned_direction();
    assert_eq!(sd.value, 350.);
}

#[test]
fn signed_direction_convert_positive() {
    let ud1 = SignedDirection::<Degree, f32>::new(90.);
    let ud2: SignedDirection<Turn, f32> = ud1.convert::<Turn>();
    assert_eq!(ud2.value, 0.25);
}

#[test]
fn signed_direction_convert_negative() {
    let ud1 = SignedDirection::<Degree, f32>::new(-90.);
    let ud2: SignedDirection<Turn, f32> = ud1.convert::<Turn>();
    assert_eq!(ud2.value, -0.25);
}

#[test]
fn signed_direction_lossless_into_32_to_32() {
    let m1 = SignedDirection::<Degree, f32>::new(12.);
    let m2: SignedDirection<Degree, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossless_into_32_to_64() {
    let m1 = SignedDirection::<Degree, f32>::new(12.);
    let m2: SignedDirection<Degree, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossless_into_64_to_64() {
    let m1 = SignedDirection::<Degree, f64>::new(12.);
    let m2: SignedDirection<Degree, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossy_into_32_to_32() {
    let m1 = SignedDirection::<Degree, f32>::new(12.);
    let m2: SignedDirection<Degree, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossy_into_32_to_64() {
    let m1 = SignedDirection::<Degree, f32>::new(12.);
    let m2: SignedDirection<Degree, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossy_into_64_to_32() {
    let m1 = SignedDirection::<Degree, f64>::new(12.);
    let m2: SignedDirection<Degree, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_lossy_into_64_to_64() {
    let m1 = SignedDirection::<Degree, f64>::new(12.);
    let m2: SignedDirection<Degree, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn signed_direction_addition_of_vector() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let m = Measure::<Degree, f32>::new(7.);
    let ud2: SignedDirection<Degree, f32> = ud1 + m;
    assert_eq!(ud2.value, 19.);
}

#[test]
fn signed_direction_addition_of_vector_assignment() {
    let mut ud = SignedDirection::<Degree, f32>::new(12.);
    ud += Measure::<Degree, f32>::new(7.);
    assert_eq!(ud.value, 19.);
}

#[test]
fn signed_direction_subtraction_of_vector() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let m = Measure::<Degree, f32>::new(7.);
    let ud2: SignedDirection<Degree, f32> = ud1 - m;
    assert_eq!(ud2.value, 5.);
}

#[test]
fn signed_direction_subtraction_of_vector_assignment() {
    let mut ud = SignedDirection::<Degree, f32>::new(12.);
    ud -= Measure::<Degree, f32>::new(7.);
    assert_eq!(ud.value, 5.);
}

#[test]
fn signed_directions_subtraction_with_diff_greater_than_half_turn() {
    // if diff > half_turn { diff - turn }
    let ud1 = SignedDirection::<Degree, f32>::new(-160.);
    let ud2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -170.);
}

#[test]
fn signed_directions_subtraction_with_diff_less_than_minus_half_turn() {
    // if diff < -half_turn { diff + turn }
    let ud1 = SignedDirection::<Degree, f32>::new(10.);
    let ud2 = SignedDirection::<Degree, f32>::new(-160.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 170.);
}

#[test]
fn signed_directions_subtraction_with_small_positive_diff() {
    // if 0 < diff && diff < half_turn { diff }
    let ud1 = SignedDirection::<Degree, f32>::new(170.);
    let ud2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 160.);
}

#[test]
fn signed_directions_subtraction_with_small_negative_diff() {
    // if -half_turn < diff && diff < 0 { diff }
    let ud1 = SignedDirection::<Degree, f32>::new(10.);
    let ud2 = SignedDirection::<Degree, f32>::new(170.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -160.);
}

#[test]
fn signed_directions_subtraction_with_minus_half_turn_diff() {
    // if -half_turn == diff { diff }
    let ud1 = SignedDirection::<Degree, f32>::new(10.);
    let ud2 = SignedDirection::<Degree, f32>::new(-170.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 180.);
}

#[test]
fn signed_directions_subtraction_with_half_turn_diff() {
    // if diff == half_turn { diff }
    let ud1 = SignedDirection::<Degree, f32>::new(-170.);
    let ud2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -180.);
}

#[test]
fn signed_direction_equals() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(ud2 == ud1);
    assert!(!(ud3 == ud1));
}

#[test]
fn signed_direction_differs() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud2 != ud1));
    assert!(ud3 != ud1);
}

#[test]
fn signed_direction_is_less_than() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud1 < ud2));
    assert!(ud1 < ud3);
    assert!(!(ud3 < ud1));
}

#[test]
fn signed_direction_is_less_than_or_equal_to() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(ud1 <= ud2);
    assert!(ud1 <= ud3);
    assert!(!(ud3 <= ud1));
}

#[test]
fn signed_direction_is_greater_than() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud1 > ud2));
    assert!(!(ud1 > ud3));
    assert!(ud3 > ud1);
}

#[test]
fn signed_direction_is_greater_than_or_equal_to() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = SignedDirection::<Degree, f32>::new(12.);
    let ud3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(ud1 >= ud2);
    assert!(!(ud1 >= ud3));
    assert!(ud3 >= ud1);
}

#[test]
fn signed_direction_is_equal_to_its_clone() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = ud1.clone();
    assert!(ud2 == ud1);
}

#[test]
fn signed_direction_is_equal_to_its_copy() {
    let ud1 = SignedDirection::<Degree, f32>::new(12.);
    let ud2 = ud1;
    let _ = ud1; // Copy again
    assert!(ud2 == ud1);
}

#[test]
fn signed_direction_shown_in_degrees() {
    let ud = SignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(ud.to_string(), "at 12.25 deg (in -180°..180°)");
}
