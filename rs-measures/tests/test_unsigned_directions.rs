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
fn unsigned_direction_default() {
    let ud: UnsignedDirection<Degree, f32> = UnsignedDirection::default();
    assert_eq!(ud.value, 0.);
    let ud = UnsignedDirection::<Degree>::default();
    assert_eq!(ud.value, 0.);
}

#[test]
fn unsigned_direction_integer_cycles() {
    let ud = UnsignedDirection::<Degree, f32>::new(3600.);
    assert_eq!(ud.value, 0.);
}

#[test]
fn unsigned_direction_small_positive_value() {
    let ud = UnsignedDirection::<Degree, f32>::new(10.);
    assert_eq!(ud.value, 10.);
}

#[test]
fn unsigned_direction_small_negative_value() {
    let ud = UnsignedDirection::<Degree, f32>::new(-10.);
    assert_eq!(ud.value, 350.);
}

#[test]
fn unsigned_direction_large_positive_value() {
    let ud = UnsignedDirection::<Degree, f32>::new(3604.);
    assert_eq!(ud.value, 4.);
}

#[test]
fn unsigned_direction_large_negative_value() {
    let ud = UnsignedDirection::<Degree, f32>::new(-3604.);
    assert_eq!(ud.value, 356.);
}

#[test]
fn unsigned_direction_from_small_zero_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(0.);
    let ud: UnsignedDirection<Degree, f32> =
        UnsignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, 0.);
}

#[test]
fn unsigned_direction_from_small_positive_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(10.);
    let ud: UnsignedDirection<Degree, f32> =
        UnsignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, 10.);
}

#[test]
fn unsigned_direction_from_small_negative_measure_point() {
    let mp = MeasurePoint::<Degree, f32>::new(-10.);
    let ud: UnsignedDirection<Degree, f32> =
        UnsignedDirection::<Degree, f32>::from_measure_point(mp);
    assert_eq!(ud.value, 350.);
}

#[test]
fn unsigned_direction_zero_to_measure_point() {
    let ud = UnsignedDirection::<Degree, f32>::new(0.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, 0.);
}

#[test]
fn unsigned_direction_small_to_measure_point() {
    let ud = UnsignedDirection::<Degree, f32>::new(10.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, 10.);
}

#[test]
fn unsigned_direction_large_to_measure_point() {
    let ud = UnsignedDirection::<Degree, f32>::new(350.);
    let mp: MeasurePoint<Degree, f32> = ud.to_measure_point();
    assert_eq!(mp.value, 350.);
}

#[test]
fn unsigned_direction_zero_to_signed_direction() {
    let ud = UnsignedDirection::<Degree, f32>::new(0.);
    let sd: SignedDirection<Degree, f32> = ud.to_signed_direction();
    assert_eq!(sd.value, 0.);
}

#[test]
fn unsigned_direction_small_to_signed_direction() {
    let ud = UnsignedDirection::<Degree, f32>::new(10.);
    let sd: SignedDirection<Degree, f32> = ud.to_signed_direction();
    assert_eq!(sd.value, 10.);
}

#[test]
fn unsigned_direction_large_to_signed_direction() {
    let ud = UnsignedDirection::<Degree, f32>::new(350.);
    let sd: SignedDirection<Degree, f32> = ud.to_signed_direction();
    assert_eq!(sd.value, -10.);
}

#[test]
fn unsigned_direction_convert_positive() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(90.);
    let ud2: UnsignedDirection<Cycle, f32> = ud1.convert::<Cycle>();
    assert_eq!(ud2.value, 0.25);
}

#[test]
fn unsigned_direction_convert_negative() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(-90.);
    let ud2: UnsignedDirection<Cycle, f32> = ud1.convert::<Cycle>();
    assert_eq!(ud2.value, 0.75);
}

#[test]
fn unsigned_direction_lossless_into_32_to_32() {
    let m1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m2: UnsignedDirection<Degree, f32> = m1.lossless_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossless_into_32_to_64() {
    let m1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m2: UnsignedDirection<Degree, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossless_into_64_to_64() {
    let m1 = UnsignedDirection::<Degree, f64>::new(12.);
    let m2: UnsignedDirection<Degree, f64> = m1.lossless_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossy_into_32_to_32() {
    let m1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m2: UnsignedDirection<Degree, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossy_into_32_to_64() {
    let m1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m2: UnsignedDirection<Degree, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossy_into_64_to_32() {
    let m1 = UnsignedDirection::<Degree, f64>::new(12.);
    let m2: UnsignedDirection<Degree, f32> = m1.lossy_into::<f32>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_lossy_into_64_to_64() {
    let m1 = UnsignedDirection::<Degree, f64>::new(12.);
    let m2: UnsignedDirection<Degree, f64> = m1.lossy_into::<f64>();
    assert_eq!(m2.value, 12.);
}

#[test]
fn unsigned_direction_addition_of_vector() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(7.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 + m1;
    assert_eq!(ud2.value, 19.);
    let m2 = Measure::<Degree, f32>::new(270.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 + m2;
    assert_eq!(ud3.value, 282.);
}

#[test]
fn unsigned_direction_addition_of_vector_with_overflow() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(400.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 + m1;
    assert_eq!(ud2.value, 52.);
    let m2 = Measure::<Degree, f32>::new(3610.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 + m2;
    assert_eq!(ud3.value, 22.);
}

#[test]
fn unsigned_direction_addition_of_vector_with_underflow() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(20.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 + m1;
    assert_eq!(ud2.value, 32.);
    let m2 = Measure::<Degree, f32>::new(3607.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 + m2;
    assert_eq!(ud3.value, 19.);
}

#[test]
fn unsigned_direction_addition_of_vector_assignment() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud += Measure::<Degree, f32>::new(7.);
    assert_eq!(ud.value, 19.);
    ud += Measure::<Degree, f32>::new(170.);
    assert_eq!(ud.value, 189.);
}

#[test]
fn unsigned_direction_addition_of_vector_assignment_with_overflow() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud += Measure::<Degree, f32>::new(350.);
    assert_eq!(ud.value, 2.);
    ud += Measure::<Degree, f32>::new(3590.);
    assert_eq!(ud.value, 352.);
}

#[test]
fn unsigned_direction_addition_of_vector_assignment_with_underflow() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud += Measure::<Degree, f32>::new(-17.);
    assert_eq!(ud.value, 355.);
    ud += Measure::<Degree, f32>::new(-3602.);
    assert_eq!(ud.value, 353.);
}

#[test]
fn unsigned_direction_subtraction_of_vector() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(7.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 - m1;
    assert_eq!(ud2.value, 5.);
    let m2 = Measure::<Degree, f32>::new(-270.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 - m2;
    assert_eq!(ud3.value, 282.);
}

#[test]
fn unsigned_direction_subtraction_of_vector_with_overflow() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(-400.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 - m1;
    assert_eq!(ud2.value, 52.);
    let m2 = Measure::<Degree, f32>::new(-3610.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 - m2;
    assert_eq!(ud3.value, 22.);
}

#[test]
fn unsigned_direction_subtraction_of_vector_with_underflow() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let m1 = Measure::<Degree, f32>::new(20.);
    let ud2: UnsignedDirection<Degree, f32> = ud1 - m1;
    assert_eq!(ud2.value, 352.);
    let m2 = Measure::<Degree, f32>::new(3607.);
    let ud3: UnsignedDirection<Degree, f32> = ud1 - m2;
    assert_eq!(ud3.value, 5.);
}

#[test]
fn unsigned_direction_subtraction_of_vector_assignment() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud -= Measure::<Degree, f32>::new(7.);
    assert_eq!(ud.value, 5.);
    ud -= Measure::<Degree, f32>::new(-200.);
    assert_eq!(ud.value, 205.);
}

#[test]
fn unsigned_direction_subtraction_of_vector_assignment_with_overflow() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud -= Measure::<Degree, f32>::new(-355.);
    assert_eq!(ud.value, 7.);
    ud -= Measure::<Degree, f32>::new(-3599.);
    assert_eq!(ud.value, 6.);
}

#[test]
fn unsigned_direction_subtraction_of_vector_assignment_with_underflow() {
    let mut ud = UnsignedDirection::<Degree, f32>::new(12.);
    ud -= Measure::<Degree, f32>::new(17.);
    assert_eq!(ud.value, 355.);
    ud -= Measure::<Degree, f32>::new(3602.);
    assert_eq!(ud.value, 353.);
}

#[test]
fn unsigned_directions_subtraction_with_diff_greater_than_half_cycle() {
    // if diff > half_cycle { diff - cycle }
    let ud1 = UnsignedDirection::<Degree, f32>::new(200.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -170.);
}

#[test]
fn unsigned_directions_subtraction_with_diff_less_than_minus_half_cycle() {
    // if diff < -half_cycle { diff + cycle }
    let ud1 = UnsignedDirection::<Degree, f32>::new(10.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(200.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 170.);
}

#[test]
fn unsigned_directions_subtraction_with_small_positive_diff() {
    // if 0 < diff && diff < half_cycle { diff }
    let ud1 = UnsignedDirection::<Degree, f32>::new(170.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 160.);
}

#[test]
fn unsigned_directions_subtraction_with_small_negative_diff() {
    // if -half_cycle < diff && diff < 0 { diff }
    let ud1 = UnsignedDirection::<Degree, f32>::new(10.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(170.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -160.);
}

#[test]
fn unsigned_directions_subtraction_with_minus_half_cycle_diff() {
    // if -half_cycle == diff { diff }
    let ud1 = UnsignedDirection::<Degree, f32>::new(10.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(190.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, -180.);
}

#[test]
fn unsigned_directions_subtraction_with_half_cycle_diff() {
    // if diff == half_cycle { diff }
    let ud1 = UnsignedDirection::<Degree, f32>::new(190.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(10.);
    let m: Measure<Degree, f32> = ud1 - ud2;
    assert_eq!(m.value, 180.);
}

#[test]
fn unsigned_direction_equals() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(ud2 == ud1);
    assert!(!(ud3 == ud1));
}

#[test]
fn unsigned_direction_differs() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud2 != ud1));
    assert!(ud3 != ud1);
}

#[test]
fn unsigned_direction_is_less_than() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud1 < ud2));
    assert!(ud1 < ud3);
    assert!(!(ud3 < ud1));
}

#[test]
fn unsigned_direction_is_less_than_or_equal_to() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(ud1 <= ud2);
    assert!(ud1 <= ud3);
    assert!(!(ud3 <= ud1));
}

#[test]
fn unsigned_direction_is_greater_than() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(!(ud1 > ud2));
    assert!(!(ud1 > ud3));
    assert!(ud3 > ud1);
}

#[test]
fn unsigned_direction_is_greater_than_or_equal_to() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud3 = UnsignedDirection::<Degree, f32>::new(13.);
    assert!(ud1 >= ud2);
    assert!(!(ud1 >= ud3));
    assert!(ud3 >= ud1);
}

#[test]
fn unsigned_direction_is_equal_to_its_clone() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = ud1.clone();
    assert!(ud2 == ud1);
}

#[test]
fn unsigned_direction_is_equal_to_its_copy() {
    let ud1 = UnsignedDirection::<Degree, f32>::new(12.);
    let ud2 = ud1;
    let _ = ud1; // Copy again
    assert!(ud2 == ud1);
}

#[test]
fn unsigned_direction_formatting_in_degrees() {
    let ud = UnsignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{}", ud), "at 12.25 deg (in 0°..360°)");
}

#[test]
fn unsigned_direction_formatting_in_degrees_one_fractional_digit() {
    let ud = UnsignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:.1}", ud), "at 12.2 deg (in 0°..360°)");
}

#[test]
fn unsigned_direction_formatting_for_debug_in_degrees() {
    let ud = UnsignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:?}", ud), "at 12.25 deg (in 0°..360°)");
}

#[test]
fn unsigned_direction_formatting_for_debug_in_degrees_one_fractional_digit() {
    let ud = UnsignedDirection::<Degree, f32>::new(12.25);
    assert_eq!(format!("{:.1?}", ud), "at 12.2 deg (in 0°..360°)");
}
