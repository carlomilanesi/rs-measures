rs_measures::define_1d_and_directions! {}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

pub struct Cycle;
impl MeasurementUnit for Cycle {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Cycle {
    const CYCLE_FRACTION: f64 = 1.;
}

#[test]
fn signed_direction_default() {
    let sd: SignedDirection<Degree, f32> = SignedDirection::default();
    assert_eq!(sd.value, 0.);
    let sd = SignedDirection::<Degree>::default();
    assert_eq!(sd.value, 0.);
}

#[test]
fn signed_direction_integer_cycles() {
    let sd = SignedDirection::<Degree, f32>::new(3600.);
    assert_eq!(sd.value, 0.);
}

#[test]
fn signed_direction_small_positive_value() {
    let sd = SignedDirection::<Degree, f32>::new(10.);
    assert_eq!(sd.value, 10.);
}

#[test]
fn signed_direction_small_negative_value() {
    let sd = SignedDirection::<Degree, f32>::new(-10.);
    assert_eq!(sd.value, -10.);
}

#[test]
fn signed_direction_large_positive_value() {
    let sd = SignedDirection::<Degree, f32>::new(3604.);
    assert_eq!(sd.value, 4.);
}

#[test]
fn signed_direction_large_negative_value() {
    let sd = SignedDirection::<Degree, f32>::new(-3604.);
    assert_eq!(sd.value, -4.);
}

#[test]
fn signed_direction_from_small_zero_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(0.);
    let sd: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(sd.value, 0.);
}

#[test]
fn signed_direction_from_small_positive_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(10.);
    let sd: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(sd.value, 10.);
}

#[test]
fn signed_direction_from_small_negative_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(-10.);
    let sd: SignedDirection<Degree, f32> = SignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(sd.value, -10.);
}

#[test]
fn signed_direction_zero_to_measure_point() {
    let sd = SignedDirection::<Degree, f32>::new(0.);
    let mp: MeasurePoint<Degree, f32> = sd.to_measure_point();
    assert_eq!(mp.value, 0.);
}

#[test]
fn signed_direction_small_to_measure_point() {
    let sd = SignedDirection::<Degree, f32>::new(10.);
    let mp: MeasurePoint<Degree, f32> = sd.to_measure_point();
    assert_eq!(mp.value, 10.);
}

#[test]
fn signed_direction_negative_to_measure_point() {
    let sd = SignedDirection::<Degree, f32>::new(-10.);
    let mp: MeasurePoint<Degree, f32> = sd.to_measure_point();
    assert_eq!(mp.value, -10.);
}

#[test]
fn signed_direction_zero_to_unsigned_direction() {
    let sd = SignedDirection::<Degree, f32>::new(0.);
    let ud: UnsignedDirection<Degree, f32> = sd.to_unsigned_direction();
    assert_eq!(ud.value, 0.);
}

#[test]
fn signed_direction_small_to_unsigned_direction() {
    let sd = SignedDirection::<Degree, f32>::new(10.);
    let ud: UnsignedDirection<Degree, f32> = sd.to_unsigned_direction();
    assert_eq!(ud.value, 10.);
}

#[test]
fn signed_direction_negative_to_unsigned_direction() {
    let sd = SignedDirection::<Degree, f32>::new(-10.);
    let ud: UnsignedDirection<Degree, f32> = sd.to_unsigned_direction();
    assert_eq!(ud.value, 350.);
}

#[test]
fn signed_direction_convert_positive() {
    let sd1 = SignedDirection::<Degree, f32>::new(90.);
    let sd2: SignedDirection<Cycle, f32> = sd1.convert::<Cycle>();
    assert_eq!(sd2.value, 0.25);
}

#[test]
fn signed_direction_convert_negative() {
    let sd1 = SignedDirection::<Degree, f32>::new(-90.);
    let sd2: SignedDirection<Cycle, f32> = sd1.convert::<Cycle>();
    assert_eq!(sd2.value, -0.25);
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
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(7.);
    let sd2: SignedDirection<Degree, f32> = sd1 + m1;
    assert_eq!(sd2.value, 19.);
    let m2 = Measure::<Degree, f32>::new(270.);
    let sd3: SignedDirection<Degree, f32> = sd1 + m2;
    assert_eq!(sd3.value, -78.);
}

#[test]
fn signed_direction_addition_of_vector_with_overflow() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(400.);
    let sd2: SignedDirection<Degree, f32> = sd1 + m1;
    assert_eq!(sd2.value, 52.);
    let m2 = Measure::<Degree, f32>::new(3610.);
    let sd3: SignedDirection<Degree, f32> = sd1 + m2;
    assert_eq!(sd3.value, 22.);
}

#[test]
fn signed_direction_addition_of_vector_with_underflow() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(20.);
    let sd2: SignedDirection<Degree, f32> = sd1 + m1;
    assert_eq!(sd2.value, 32.);
    let m2 = Measure::<Degree, f32>::new(3607.);
    let sd3: SignedDirection<Degree, f32> = sd1 + m2;
    assert_eq!(sd3.value, 19.);
}

#[test]
fn signed_direction_addition_of_vector_assignment() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd += Measure::<Degree, f32>::new(7.);
    assert_eq!(sd.value, 19.);
    sd += Measure::<Degree, f32>::new(170.);
    assert_eq!(sd.value, -171.);
}

#[test]
fn signed_direction_addition_of_vector_assignment_with_overflow() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd += Measure::<Degree, f32>::new(350.);
    assert_eq!(sd.value, 2.);
    sd += Measure::<Degree, f32>::new(3590.);
    assert_eq!(sd.value, -8.);
}

#[test]
fn signed_direction_addition_of_vector_assignment_with_underflow() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd += Measure::<Degree, f32>::new(-17.);
    assert_eq!(sd.value, -5.);
    sd += Measure::<Degree, f32>::new(-3602.);
    assert_eq!(sd.value, -7.);
}

#[test]
fn signed_direction_subtraction_of_vector() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(7.);
    let sd2: SignedDirection<Degree, f32> = sd1 - m1;
    assert_eq!(sd2.value, 5.);
    let m2 = Measure::<Degree, f32>::new(-270.);
    let sd3: SignedDirection<Degree, f32> = sd1 - m2;
    assert_eq!(sd3.value, -78.);
}

#[test]
fn signed_direction_subtraction_of_vector_with_overflow() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(-400.);
    let sd2: SignedDirection<Degree, f32> = sd1 - m1;
    assert_eq!(sd2.value, 52.);
    let m2 = Measure::<Degree, f32>::new(-3610.);
    let sd3: SignedDirection<Degree, f32> = sd1 - m2;
    assert_eq!(sd3.value, 22.);
}

#[test]
fn signed_direction_subtraction_of_vector_with_underflow() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(20.);
    let sd2: SignedDirection<Degree, f32> = sd1 - m1;
    assert_eq!(sd2.value, -8.);
    let m2 = Measure::<Degree, f32>::new(3607.);
    let sd3: SignedDirection<Degree, f32> = sd1 - m2;
    assert_eq!(sd3.value, 5.);
}

#[test]
fn signed_direction_subtraction_of_vector_assignment() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd -= Measure::<Degree, f32>::new(7.);
    assert_eq!(sd.value, 5.);
    sd -= Measure::<Degree, f32>::new(-200.);
    assert_eq!(sd.value, -155.);
}

#[test]
fn signed_direction_subtraction_of_vector_assignment_with_overflow() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd -= Measure::<Degree, f32>::new(-355.);
    assert_eq!(sd.value, 7.);
    sd -= Measure::<Degree, f32>::new(-3599.);
    assert_eq!(sd.value, 6.);
}

#[test]
fn signed_direction_subtraction_of_vector_assignment_with_underflow() {
    let mut sd = SignedDirection::<Degree, f32>::new(12.);
    sd -= Measure::<Degree, f32>::new(17.);
    assert_eq!(sd.value, -5.);
    sd -= Measure::<Degree, f32>::new(3602.);
    assert_eq!(sd.value, -7.);
}

#[test]
fn signed_directions_subtraction_with_diff_greater_than_half_cycle() {
    // if diff > half_cycle { diff - cycle }
    let sd1 = SignedDirection::<Degree, f32>::new(-160.);
    let sd2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, -170.);
}

#[test]
fn signed_directions_subtraction_with_diff_less_than_minus_half_cycle() {
    // if diff < -half_cycle { diff + cycle }
    let sd1 = SignedDirection::<Degree, f32>::new(10.);
    let sd2 = SignedDirection::<Degree, f32>::new(-160.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, 170.);
}

#[test]
fn signed_directions_subtraction_with_small_positive_diff() {
    // if 0 < diff && diff < half_cycle { diff }
    let sd1 = SignedDirection::<Degree, f32>::new(170.);
    let sd2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, 160.);
}

#[test]
fn signed_directions_subtraction_with_small_negative_diff() {
    // if -half_cycle < diff && diff < 0 { diff }
    let sd1 = SignedDirection::<Degree, f32>::new(10.);
    let sd2 = SignedDirection::<Degree, f32>::new(170.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, -160.);
}

#[test]
fn signed_directions_subtraction_with_minus_half_cycle_diff() {
    // if -half_cycle == diff { diff }
    let sd1 = SignedDirection::<Degree, f32>::new(10.);
    let sd2 = SignedDirection::<Degree, f32>::new(-170.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, 180.);
}

#[test]
fn signed_directions_subtraction_with_half_cycle_diff() {
    // if diff == half_cycle { diff }
    let sd1 = SignedDirection::<Degree, f32>::new(-170.);
    let sd2 = SignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = sd1 - sd2;
    assert_eq!(m.value, -180.);
}

#[test]
fn signed_direction_equals() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(sd2 == sd1);
    assert!(!(sd3 == sd1));
}

#[test]
fn signed_direction_differs() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(sd2 != sd1));
    assert!(sd3 != sd1);
}

#[test]
fn signed_direction_is_less_than() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(sd1 < sd2));
    assert!(sd1 < sd3);
    assert!(!(sd3 < sd1));
}

#[test]
fn signed_direction_is_less_than_or_equal_to() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(sd1 <= sd2);
    assert!(sd1 <= sd3);
    assert!(!(sd3 <= sd1));
}

#[test]
fn signed_direction_is_greater_than() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(!(sd1 > sd2));
    assert!(!(sd1 > sd3));
    assert!(sd3 > sd1);
}

#[test]
fn signed_direction_is_greater_than_or_equal_to() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = SignedDirection::<Degree, f32>::new(12.);
    let sd3 = SignedDirection::<Degree, f32>::new(13.);
    assert!(sd1 >= sd2);
    assert!(!(sd1 >= sd3));
    assert!(sd3 >= sd1);
}

#[test]
fn signed_direction_is_equal_to_its_clone() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = sd1.clone();
    assert!(sd2 == sd1);
}

#[test]
fn signed_direction_is_equal_to_its_copy() {
    let sd1 = SignedDirection::<Degree, f32>::new(12.);
    let sd2 = sd1;
    let _ = sd1; // Copy again
    assert!(sd2 == sd1);
}

#[test]
fn signed_direction_formatting_in_degrees() {
    let sd = SignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{}", sd), "at 12.25 deg (in -180°..180°)");
}

#[test]
fn signed_direction_formatting_in_degrees_one_fractional_digit() {
    let sd = SignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:.1}", sd), "at 12.2 deg (in -180°..180°)");
}

#[test]
fn signed_direction_formatting_for_debug_in_degrees() {
    let sd = SignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:?}", sd), "at 12.25 deg (in -180°..180°)");
}

#[test]
fn signed_direction_formatting_for_debug_in_degrees_one_fractional_digit() {
    let sd = SignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", sd), "at 12.2 deg (in -180°..180°)");
}
