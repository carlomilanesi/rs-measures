use std::f64::consts::TAU;
rs_measures::define_measure_2d! {}

mod test_utils;

struct Length;

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

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
fn linear_map_2d_new() {
    let lm = LinearMap2d::<f32>::new([[12., 23.], [34., -45.]]);
    assert_eq!(lm.c[0][0], 12.);
    assert_eq!(lm.c[0][1], 23.);
    assert_eq!(lm.c[1][0], 34.);
    assert_eq!(lm.c[1][1], -45.);
}

//// N.B.: Linear maps have no translations.

//// Rotations

#[test]
fn linear_map_2d_rotation_by_angle() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm1 = LinearMap2d::rotation(Measure::<Degree, f32>::new(90.));
    let m2 = lm1.apply_to(m1);
    assert_eq_32!(m2.x, -5.);
    assert_eq_32!(m2.y, 8.);

    let lm2 = LinearMap2d::rotation(Measure::<Degree, f32>::new(30.));
    let m3 = lm2.apply_to(m1);
    assert_eq_64!(m3.x, 4.428203105926514);
    assert_eq_64!(m3.y, 8.330126762390137);
}

#[test]
fn linear_map_2d_rotation_at_right() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation_at_right();
    let m2 = lm.apply_to(m1);
    assert_eq!(m2.x, 5.);
    assert_eq!(m2.y, -8.);
}

#[test]
fn linear_map_2d_rotation_at_left() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation_at_left();
    let m2 = lm.apply_to(m1);
    assert_eq!(m2.x, -5.);
    assert_eq!(m2.y, 8.);
}

// Projections

#[test]
fn linear_map_2d_projection_by_point_angle() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::projection_by_point_angle(MeasurePoint::<Degree, f64>::new(60.));
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, 4.165063509461098);
    assert_eq_64!(m2.y, 7.214101615137755);
}

#[test]
fn linear_map_2d_projection_by_signed_direction() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::projection_by_signed_direction(
        SignedDirection::<Degree, f64>::new(60.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, 4.165063509461098);
    assert_eq_64!(m2.y, 7.214101615137755);
}

#[test]
fn linear_map_2d_unsigned_direction() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::projection_by_unsigned_direction(
        UnsignedDirection::<Degree, f64>::new(60.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, 4.165063509461098);
    assert_eq_64!(m2.y, 7.214101615137755);
}

#[test]
fn linear_map_2d_projection_by_unit_vector() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let unit_vector = Measure2d::<Metre, f64>::new(1., 5.).normalized();
    assert_eq_64!(unit_vector.squared_norm(), 1.);
    let lm = LinearMap2d::<f64>::projection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, 1.2692307692307694);
    assert_eq_64!(m2.y, 6.346153846153847);
}

// Reflections

#[test]
fn linear_map_2d_reflection_by_point_angle() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::reflection_by_point_angle(MeasurePoint::<Degree, f64>::new(80.));
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, -5.807440249658923);
    assert_eq_64!(m2.y, 7.434624250534892);
}

#[test]
fn linear_map_2d_reflection_by_signed_direction() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::reflection_by_signed_direction(
        SignedDirection::<Degree, f64>::new(80.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, -5.807440249658923);
    assert_eq_64!(m2.y, 7.434624250534892);
}

#[test]
fn linear_map_2d_reflection_by_unsigned_direction() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::reflection_by_unsigned_direction(
        UnsignedDirection::<Degree, f64>::new(80.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, -5.807440249658923);
    assert_eq_64!(m2.y, 7.434624250534892);
}

#[test]
fn linear_map_2d_reflection_by_unit_vector() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let unit_vector = Measure2d::<Metre, f64>::new(1., 5.).normalized();
    assert_eq_64!(unit_vector.squared_norm(), 1.);
    let lm = LinearMap2d::<f64>::reflection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_64!(m2.x, -5.461538461538462);
    assert_eq_64!(m2.y, 7.692307692307694);
}

// Scaling by two factors

#[test]
fn linear_map_2d_scaling() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::scaling(3., 7.);
    let m2 = lm.apply_to(m1);
    assert_eq!(m2.x, 8. * 3.);
    assert_eq!(m2.y, 5. * 7.);
}

// Inversion

#[test]
fn linear_map_2d_inverted() {
    let m = Measure2d::<Metre, f64>::new(8., 5.);
    let lm = LinearMap2d::<f64>::new([[1.2, 0.8], [3.4, -1.3]]);
    let inverse_of_lm = lm.inverted();
    let transformed = lm.apply_to(m);
    assert!(m != transformed);
    let transformed_back = inverse_of_lm.apply_to(transformed);
    assert_eq_64!(transformed_back.x, 8.);
    assert_eq_64!(transformed_back.y, 5.);
}

#[test]
fn linear_map_2d_combined_with() {
    let m1 = Measure2d::<Metre, f64>::new(8., 5.);
    let lm1 = LinearMap2d::<f64>::new([[1.2, 0.8], [3.4, -1.3]]);
    let lm2 = LinearMap2d::<f64>::new([[-0.2, 3.1], [2.7, 4.4]]);

    // To the original vector, first a transformation is applied,
    // which represents the application of lm2 and then of lm1.
    let lm2_and_then_lm1 = lm1.combined_with(&lm2);
    let m2 = lm2_and_then_lm1.apply_to(m1);

    // To the resulting vector, another transformation is applied,
    // which represents the application of lm1 inverted and then of lm2 inverted.
    let lm1_inverted_and_then_lm2_inverted = lm2.inverted().combined_with(&lm1.inverted());
    let m3 = lm1_inverted_and_then_lm2_inverted.apply_to(m2);

    // The original vector should be obtained.
    assert_eq_64!(m3.x, 8.);
    assert_eq_64!(m3.y, 5.);

    // If lm1 and lm2 are swapped, and also their inverses are swapped,
    // the same result should be obtained.
    let lm1_and_then_lm2 = lm2.combined_with(&lm1);
    let m3 = lm1_and_then_lm2.apply_to(m1);
    let lm2_inverted_and_then_lm1_inverted = lm1.inverted().combined_with(&lm2.inverted());
    let m4 = lm2_inverted_and_then_lm1_inverted.apply_to(m3);
    assert_eq_64!(m4.x, 8.);
    assert_eq_64!(m4.y, 5.);
}

#[test]
fn linear_map_2d_formatting_with_no_padding() {
    let lm = LinearMap2d::<f64>::new([[1.2, 000.8], [3.400, 1.3]]);
    assert_eq!(lm.to_string(), "[1.2 0.8]\n[3.4 1.3]");
}

#[test]
fn linear_map_2d_formatting_with_initial_padding() {
    let lm = LinearMap2d::<f64>::new([[1.2, 20.8], [873.4, 1.3]]);
    assert_eq!(lm.to_string(), "[  1.2 20.8]\n[873.4  1.3]");
}

#[test]
fn linear_map_2d_formatting_with_final_padding() {
    let lm = LinearMap2d::<f64>::new([[1.254, 0.8], [3.4, 1.36]]);
    assert_eq!(lm.to_string(), "[1.254 0.8 ]\n[3.4   1.36]");
}

#[test]
fn linear_map_2d_formatting_with_both_padding() {
    let lm = LinearMap2d::<f64>::new([[1.254, 650.], [98763.4, 1.7658909]]);
    assert_eq!(
        lm.to_string(),
        "[    1.254 650        ]\n[98763.4     1.7658909]"
    );
}
