use std::f64::consts::TAU;
rs_measures::define_measure_2d! {}

mod test_utils;

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
fn affine_map_2d_new() {
    let am = AffineMap2d::<Metre, f32>::new([[12., 23., 34.], [-45., 56., 67.]]);
    assert_eq!(am.c[0][0], 12.);
    assert_eq!(am.c[0][1], 23.);
    assert_eq!(am.c[0][2], 34.);
    assert_eq!(am.c[1][0], -45.);
    assert_eq!(am.c[1][1], 56.);
    assert_eq!(am.c[1][2], 67.);
}

#[test]
fn affine_map_2d_convert() {
    let am1 = AffineMap2d::<Metre, f32>::new([[12., 23., 34.], [-45., 56., 67.]]);
    let am2: AffineMap2d<MilliMetre, f32> = am1.convert::<MilliMetre>();
    assert_eq!(am2.c[0][0], 12.);
    assert_eq!(am2.c[0][1], 23.);
    assert_eq_32!(am2.c[0][2], 34000.);
    assert_eq!(am2.c[1][0], -45.);
    assert_eq!(am2.c[1][1], 56.);
    assert_eq_32!(am2.c[1][2], 67000.);
}

// Translations

#[test]
fn affine_map_2d_translation() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);
    let am: AffineMap2d<Metre, f64> =
        AffineMap2d::<Metre, f64>::translation(Measure2d::new(12., 23.));
    let mp2: MeasurePoint2d<Metre, f64> = am.apply_to(mp1);
    assert_eq!(mp2.x, 8. + 12.);
    assert_eq!(mp2.y, 5. + 23.);
}

// Rotations
#[test]
fn affine_map_2d_rotation_by_angle() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::rotation(fixed_point, Measure::<Degree, f64>::new(90.));
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 3.);
    assert_eq_64!(mp2.y, 4.);

    let am2 = AffineMap2d::<Metre, f64>::rotation(fixed_point, Measure::<Degree, f64>::new(30.));
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 6.2320508075688785);
    assert_eq_64!(mp3.y, 5.598076211353316);
}

#[test]
fn affine_map_2d_rotation_at_right() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::rotation_at_right(fixed_point);
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 9.);
    assert_eq_64!(mp2.y, 0.);
}

#[test]
fn affine_map_2d_rotation_at_left() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::rotation_at_left(fixed_point);
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 3.);
    assert_eq_64!(mp2.y, 4.);
}

// Projections

#[test]
fn affine_map_2d_projection_by_point_angle() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::projection_by_point_angle(
        fixed_point,
        MeasurePoint::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 6.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::projection_by_point_angle(
        fixed_point,
        MeasurePoint::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.79903810567666);
    assert_eq_64!(mp3.y, 5.116025403784439);
}

#[test]
fn affine_map_2d_projection_by_signed_direction() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::projection_by_signed_direction(
        fixed_point,
        SignedDirection::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 6.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::projection_by_signed_direction(
        fixed_point,
        SignedDirection::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.79903810567666);
    assert_eq_64!(mp3.y, 5.116025403784439);
}

#[test]
fn affine_map_2d_projection_by_unsigned_direction() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::projection_by_unsigned_direction(
        fixed_point,
        UnsignedDirection::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 6.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::projection_by_unsigned_direction(
        fixed_point,
        UnsignedDirection::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.79903810567666);
    assert_eq_64!(mp3.y, 5.116025403784439);
}

#[test]
fn affine_map_2d_projection_by_unit_vector() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::projection_by_unit_vector(
        fixed_point,
        Measure2d::<Metre, f64>::new(0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 6.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::projection_by_unit_vector(
        fixed_point,
        Measure2d::<Metre, f64>::new(1., 5.).normalized(),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 6.653846153846154);
    assert_eq_64!(mp3.y, 5.26923076923077);
}

// Reflections

#[test]
fn affine_map_2d_reflection_by_point_angle() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::reflection_by_point_angle(
        fixed_point,
        MeasurePoint::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 4.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::reflection_by_point_angle(
        fixed_point,
        MeasurePoint::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.598076211353319);
    assert_eq_64!(mp3.y, 5.232050807568878);
}

#[test]
fn affine_map_2d_reflection_by_signed_direction() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::reflection_by_signed_direction(
        fixed_point,
        SignedDirection::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 4.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::reflection_by_signed_direction(
        fixed_point,
        SignedDirection::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.598076211353319);
    assert_eq_64!(mp3.y, 5.232050807568878);
}

#[test]
fn affine_map_2d_reflection_by_unsigned_direction() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::reflection_by_unsigned_direction(
        fixed_point,
        UnsignedDirection::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 4.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::reflection_by_unsigned_direction(
        fixed_point,
        UnsignedDirection::<Degree, f64>::new(60.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 7.598076211353319);
    assert_eq_64!(mp3.y, 5.232050807568878);
}

#[test]
fn affine_map_2d_reflection_by_unit_vector() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);

    let am1 = AffineMap2d::<Metre, f64>::reflection_by_unit_vector(
        fixed_point,
        Measure2d::<Metre, f64>::new(0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 4.);
    assert_eq_64!(mp2.y, 5.);

    let am2 = AffineMap2d::<Metre, f64>::reflection_by_unit_vector(
        fixed_point,
        Measure2d::<Metre, f64>::new(1., 5.).normalized(),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 5.3076923076923075);
    assert_eq_64!(mp3.y, 5.53846153846154);
}

// Scaling by two factors

//vvvv
#[test]
fn affine_map_2d_scaling() {
    let fixed_point = MeasurePoint2d::<Metre, f64>::new(6., 2.);
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);
    let am = AffineMap2d::<Metre, f64>::scaling(fixed_point, 3., 7.);
    let mp2 = am.apply_to(mp1);
    assert_eq!(mp2.x, 6. + (8. - 6.) * 3.);
    assert_eq!(mp2.y, 2. + (5. - 2.) * 7.);
}

// Inversion

#[test]
fn affine_map_2d_inverted() {
    let mp = MeasurePoint2d::<Metre, f64>::new(8., 5.);
    let am = AffineMap2d::<Metre, f64>::new([[1.2, 0.8, 6.], [3.4, -1.3, 2.]]);
    let inverse_of_am = am.inverted();
    let transformed = am.apply_to(mp);
    assert!(mp != transformed);
    let transformed_back = inverse_of_am.apply_to(transformed);
    assert_eq_64!(transformed_back.x, 8.);
    assert_eq_64!(transformed_back.y, 5.);
}

#[test]
fn affine_map_2d_combined_with() {
    let mp1 = MeasurePoint2d::<Metre, f64>::new(8., 5.);
    let am1 = AffineMap2d::<Metre, f64>::new([[1.2, 0.8, 6.], [3.4, -1.3, 2.]]);
    let am2 = AffineMap2d::<Metre, f64>::new([[-0.2, 3.1, -1.], [2.7, 4.4, 3.]]);

    // To the original point, first a transformation is applied,
    // which represents the application of am2 and then of am1.
    let am2_and_then_am1 = am1.combined_with(&am2);
    let mp2 = am2_and_then_am1.apply_to(mp1);

    // to the resulting point, another transformation is applied,
    // which represents the application of am1 inverted and then of am2 inverted.
    let am1_inverted_and_then_am2_inverted = am2.inverted().combined_with(&am1.inverted());
    let mp3 = am1_inverted_and_then_am2_inverted.apply_to(mp2);

    // The original point should be obtained.
    assert_eq_64!(mp3.x, 8.);
    assert_eq_64!(mp3.y, 5.);

    // If am1 and am2 are swapped, and also their inverses are swapped,
    // the same result should be obtained.
    let am1_and_then_am2 = am2.combined_with(&am1);
    let mp3 = am1_and_then_am2.apply_to(mp1);
    let am2_inverted_and_then_am1_inverted = am1.inverted().combined_with(&am2.inverted());
    let mp4 = am2_inverted_and_then_am1_inverted.apply_to(mp3);
    assert_eq_64!(mp4.x, 8.);
    assert_eq_64!(mp4.y, 5.);
}

#[test]
fn affine_map_2d_formatting_with_no_padding() {
    let am = AffineMap2d::<Metre, f64>::new([[1.2, 000.8, 1.2], [3.400, 1.3, 1.4]]);
    assert_eq!(am.to_string(), "[1.2 0.8 1.2] m\n[3.4 1.3 1.4]");
}

#[test]
fn affine_map_2d_formatting_with_initial_padding() {
    let am = AffineMap2d::<Metre, f64>::new([[1.2, 20.8, 2567.], [873.4, 1.3, 0.]]);
    assert_eq!(am.to_string(), "[  1.2 20.8 2567] m\n[873.4  1.3    0]");
}

#[test]
fn affine_map_2d_formatting_with_final_padding() {
    let am = AffineMap2d::<Metre, f64>::new([[1.254, 0.8, -0.3401], [3.4, 1.36, 45.]]);
    assert_eq!(
        am.to_string(),
        "[1.254 0.8  -0.3401] m\n[3.4   1.36 45     ]"
    );
}

#[test]
fn affine_map_2d_formatting_with_both_padding() {
    let am = AffineMap2d::<Metre, f64>::new([[1.254, 650., -872.], [98763.4, 1.7658909, 5.43]]);
    assert_eq!(
        am.to_string(),
        "[    1.254 650         -872   ] m\n[98763.4     1.7658909    5.43]"
    );
}
