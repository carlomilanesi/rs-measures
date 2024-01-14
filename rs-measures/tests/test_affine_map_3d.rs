use std::f64::consts::TAU;
rs_measures::define_measure_3d! {}

mod test_utils;

pub struct Dimensionless;
impl VectorProperty for Dimensionless {}

pub struct Unspecified;
impl MeasurementUnit for Unspecified {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}

struct Length;
impl VectorProperty for Length {}

struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 0.001;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

#[test]
fn affine_map_3d_new() {
    let am = AffineMap3d::<Metre, f32>::new([
        [12., 23., -34., 45.],
        [-56., 67., 78., 89.],
        [91., -102., 113., 124.],
    ]);
    assert_eq!(am.c[0][0], 12.);
    assert_eq!(am.c[0][1], 23.);
    assert_eq!(am.c[0][2], -34.);
    assert_eq!(am.c[0][3], 45.);
    assert_eq!(am.c[1][0], -56.);
    assert_eq!(am.c[1][1], 67.);
    assert_eq!(am.c[1][2], 78.);
    assert_eq!(am.c[1][3], 89.);
    assert_eq!(am.c[2][0], 91.);
    assert_eq!(am.c[2][1], -102.);
    assert_eq!(am.c[2][2], 113.);
    assert_eq!(am.c[2][3], 124.);
}

#[test]
fn affine_map_3d_convert() {
    let am1 = AffineMap3d::<Metre, f32>::new([
        [12., 23., -34., 45.],
        [-56., 67., 78., 89.],
        [91., -102., 113., 124.],
    ]);
    let am2: AffineMap3d<MilliMetre, f32> = am1.convert::<MilliMetre>();
    assert_eq!(am2.c[0][0], 12.);
    assert_eq!(am2.c[0][1], 23.);
    assert_eq!(am2.c[0][2], -34.);
    assert_eq_32!(am2.c[0][3], 45000.);
    assert_eq!(am2.c[1][0], -56.);
    assert_eq!(am2.c[1][1], 67.);
    assert_eq!(am2.c[1][2], 78.);
    assert_eq_32!(am2.c[1][3], 89000.);
    assert_eq!(am2.c[2][0], 91.);
    assert_eq!(am2.c[2][1], -102.);
    assert_eq!(am2.c[2][2], 113.);
    assert_eq_32!(am2.c[2][3], 124000.);
}

// Translations

#[test]
fn affine_map_3d_translation() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);
    let am: AffineMap3d<Metre, f64> =
        AffineMap3d::<Metre, f64>::translation(Measure3d::new(12., -23., 45.));
    let mp2: MeasurePoint3d<Metre, f64> = am.apply_to(mp1);
    assert_eq!(mp2.x, 8. + 12.);
    assert_eq!(mp2.y, 5. - 23.);
    assert_eq!(mp2.z, -2. + 45.);
}

// Rotations

#[test]
fn affine_map_3d_rotation() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);

    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let am1 = AffineMap3d::<Metre, f64>::rotation(
        fixed_point,
        Measure3d::<Unspecified, f64>::new(0., 0., 1.),
        Measure::<Degree, f64>::new(90.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 3.);
    assert_eq_64!(mp2.y, 4.);
    assert_eq_64!(mp2.z, -2.);

    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let am2 = AffineMap3d::<Metre, f64>::rotation(
        fixed_point,
        unit_vector,
        Measure::<Degree, f64>::new(30.),
    );
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 8.946504549699267);
    assert_eq_64!(mp3.y, 4.6092272773226455);
    assert_eq_64!(mp3.z, -2.1801727328416165);
}

// Projections

#[test]
fn affine_map_3d_projection_onto_line() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);

    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let am1 = AffineMap3d::<Metre, f64>::projection_onto_line(
        fixed_point,
        Measure3d::<Metre, f64>::new(0., 0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 6.);
    assert_eq_64!(mp2.y, 2.);
    assert_eq_64!(mp2.z, -2.);

    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let am2 = AffineMap3d::<Metre, f64>::projection_onto_line(fixed_point, unit_vector);
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 8.827586206896552);
    assert_eq_64!(mp3.y, 6.241379310344827);
    assert_eq_64!(mp3.z, -3.3448275862068955);
}

#[test]
fn affine_map_3d_projection_onto_plane() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);

    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let am1 = AffineMap3d::<Metre, f64>::projection_onto_plane(
        fixed_point,
        Measure3d::<Metre, f64>::new(0., 0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 8.);
    assert_eq_64!(mp2.y, 5.);
    assert_eq_64!(mp2.z, -9.);

    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let am2 = AffineMap3d::<Metre, f64>::projection_onto_plane(fixed_point, unit_vector);
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 5.172413793103448);
    assert_eq_64!(mp3.y, 0.7586206896551726);
    assert_eq_64!(mp3.z, -7.655172413793104);
}

// Reflections

#[test]
fn affine_map_3d_reflection_over_line() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);

    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let am1 = AffineMap3d::<Metre, f64>::reflection_over_line(
        fixed_point,
        Measure3d::<Metre, f64>::new(0., 0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 4.);
    assert_eq_64!(mp2.y, -1.);
    assert_eq_64!(mp2.z, -2.);

    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let am2 = AffineMap3d::<Metre, f64>::reflection_over_line(fixed_point, unit_vector);
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 9.655172413793103);
    assert_eq_64!(mp3.y, 7.482758620689655);
    assert_eq_64!(mp3.z, -4.689655172413792);
}

#[test]
fn affine_map_3d_reflection_over_plane() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);

    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let am1 = AffineMap3d::<Metre, f64>::reflection_over_plane(
        fixed_point,
        Measure3d::<Metre, f64>::new(0., 0., 1.),
    );
    let mp2 = am1.apply_to(mp1);
    assert_eq_64!(mp2.x, 8.);
    assert_eq_64!(mp2.y, 5.);
    assert_eq_64!(mp2.z, -16.);

    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let am2 = AffineMap3d::<Metre, f64>::reflection_over_plane(fixed_point, unit_vector);
    let mp3 = am2.apply_to(mp1);
    assert_eq_64!(mp3.x, 2.344827586206896);
    assert_eq_64!(mp3.y, -3.4827586206896557);
    assert_eq_64!(mp3.z, -13.310344827586208);
}

// Scaling by two factors

#[test]
fn affine_map_3d_scaling() {
    let fixed_point = MeasurePoint3d::<Metre, f64>::new(6., 2., -9.);
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);
    let am = AffineMap3d::<Metre, f64>::scaling(fixed_point, 3., -7., 4.);
    let mp2 = am.apply_to(mp1);
    assert_eq!(mp2.x, 6. + (8. - 6.) * 3.);
    assert_eq!(mp2.y, 2. + (5. - 2.) * -7.);
    assert_eq!(mp2.z, -9. + (-2. - -9.) * 4.);
}

// Inversion

#[test]
fn affine_map_3d_inverted() {
    let mp = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);
    let am = AffineMap3d::<Metre, f64>::new([
        [1.2, 0.8, 6., -1.7],
        [3.4, -1.3, 2., 0.4],
        [9.1, -5., 0.2, 1.9],
    ]);
    let inverse_of_am = am.inverted();
    let transformed = am.apply_to(mp);
    assert!(mp != transformed);
    let transformed_back = inverse_of_am.apply_to(transformed);
    assert_eq_64!(transformed_back.x, 8.);
    assert_eq_64!(transformed_back.y, 5.);
    assert_eq_64!(transformed_back.z, -2.);
}

#[test]
fn affine_map_3d_combined_with() {
    let mp1 = MeasurePoint3d::<Metre, f64>::new(8., 5., -2.);
    let am1 = AffineMap3d::<Metre, f64>::new([
        [1.2, 0.8, 4.7, 6.],
        [3.4, -1.3, 0.6, 2.],
        [1.1, -5., 0.2, -7.1],
    ]);
    let am2 = AffineMap3d::<Metre, f64>::new([
        [-0.2, 3.1, -1., 7.4],
        [2.7, 4.4, 3., 7.7],
        [4., 5.2, -3.1, 4.4],
    ]);

    // To the original point, first a transformation is applied,
    // which represents the application of am2 and then of am1.
    let am2_and_then_am1 = am1.combined_with(&am2);
    let mp2 = am2_and_then_am1.apply_to(mp1);

    // to the resulting point, another transformation is applied,
    // which represents the application of am1 inverted and then of am2 inverted.
    let am1_inverted_and_then_am2_inverted = am2.inverted().combined_with(&am1.inverted());
    let mp3 = am1_inverted_and_then_am2_inverted.apply_to(mp2);

    // The original point should be obtained.
    //println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n", mp1, am1, am2, am2_and_then_am1, mp2);
    assert_eq_64!(mp3.x, 8.);
    assert_eq_64!(mp3.y, 5.);
    assert_eq_64!(mp3.z, -2.);

    // If am1 and am2 are swapped, and also their inverses are swapped,
    // the same result should be obtained.
    let am1_and_then_am2 = am2.combined_with(&am1);
    let mp3 = am1_and_then_am2.apply_to(mp1);
    let am2_inverted_and_then_am1_inverted = am1.inverted().combined_with(&am2.inverted());
    let mp4 = am2_inverted_and_then_am1_inverted.apply_to(mp3);
    assert_eq_64!(mp4.x, 8.);
    assert_eq_64!(mp4.y, 5.);
    assert_eq_64!(mp4.z, -2.);
}

#[test]
fn affine_map_3d_formatting_with_no_padding() {
    let am = AffineMap3d::<Metre, f64>::new([
        [1.2, 000.8, 1.2, 6.],
        [3.400, 1.3, 1.4, 2.],
        [8.7, 3.1, 5.8, 0.],
    ]);
    assert_eq!(
        am.to_string(),
        "[1.2 0.8 1.2 6] m\n[3.4 1.3 1.4 2]\n[8.7 3.1 5.8 0]"
    );
}

#[test]
fn affine_map_3d_formatting_with_initial_padding() {
    let am = AffineMap3d::<Metre, f64>::new([
        [1.2, 20.8, 2567., 56.3],
        [873.4, 1.3, 0., 2151.1],
        [3.1, 3473.2, 10., 34.2],
    ]);
    assert_eq!(
        am.to_string(),
        "[  1.2   20.8 2567   56.3] m\n[873.4    1.3    0 2151.1]\n[  3.1 3473.2   10   34.2]"
    );
}

#[test]
fn affine_map_3d_formatting_with_final_padding() {
    let am = AffineMap3d::<Metre, f64>::new([
        [1.254, 0.8, -0.3401, -34.2],
        [3.4, 1.36, 45.958234, 345.23],
        [2.64, 5.452710, 45., 731.],
    ]);
    assert_eq!(
        am.to_string(),
        "[1.254 0.8     -0.3401   -34.2 ] m\n[3.4   1.36    45.958234 345.23]\n[2.64  5.45271 45        731   ]"
    );
}

#[test]
fn affine_map_3d_formatting_with_both_padding() {
    let am = AffineMap3d::<Metre, f64>::new([
        [1.254, 650., -872., 6.],
        [98763.4, 1.7658909, 5.43, 9.],
        [756., 3.8472, 41.81, -561.81],
    ]);
    assert_eq!(
        am.to_string(),
        "[    1.254 650         -872       6   ] m\n[98763.4     1.7658909    5.43    9   ]\n[  756       3.8472      41.81 -561.81]"
    );
}
