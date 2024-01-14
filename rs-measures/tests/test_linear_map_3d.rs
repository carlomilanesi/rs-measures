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
fn linear_map_3d_new() {
    let lm = LinearMap3d::<f32>::new([[12., 23., 34.], [45., -56., 67.], [78., 89., 90.]]);
    assert_eq!(lm.c[0][0], 12.);
    assert_eq!(lm.c[0][1], 23.);
    assert_eq!(lm.c[0][2], 34.);
    assert_eq!(lm.c[1][0], 45.);
    assert_eq!(lm.c[1][1], -56.);
    assert_eq!(lm.c[1][2], 67.);
    assert_eq!(lm.c[2][0], 78.);
    assert_eq!(lm.c[2][1], 89.);
    assert_eq!(lm.c[2][2], 90.);
}

// N.B.: Linear maps have no translations.

// Rotations

#[test]
fn linear_map_3d_rotation_around_x_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::rotation(
        Measure::<Degree, f64>::new(90.),
        Measure3d::<Unspecified, f64>::new(1., 0., 0.),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, 2.);
    assert_eq_64!(m2.z, 5.);
}

#[test]
fn linear_map_3d_rotation_around_y_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::rotation(
        Measure::<Degree, f64>::new(90.),
        Measure3d::<Unspecified, f64>::new(0., 1., 0.),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -2.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, -8.);
}

#[test]
fn linear_map_3d_rotation_around_z_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::rotation(
        Measure::<Degree, f64>::new(90.),
        Measure3d::<Unspecified, f64>::new(0., 0., 1.),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -5.);
    assert_eq_64!(m2.y, 8.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_rotation_by_angle() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::rotation(
        Measure::<Degree, f64>::new(30.),
        Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized(),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 4.726675262453692);
    assert_eq_64!(m2.y, 7.9914102488289105);
    assert_eq_64!(m2.z, -2.6068953178485286);
}

#[test]
fn linear_map_3d_rotation_as_reflection() {
    let m1 = Measure3d::<Metre, f64>::new(8.1, 5.2, -2.3);
    let direction = Measure3d::<Unspecified, f64>::new(-2.3, 1.7, 0.4).normalized();
    let rotation = LinearMap3d::rotation(Measure::<Degree, f64>::new(180.), direction);
    let rotated = rotation.apply_to(m1);
    let reflection = LinearMap3d::reflection_over_line(direction);
    let reflected = reflection.apply_to(m1);
    assert_eq_64!(rotated.x, reflected.x);
    assert_eq_64!(rotated.y, reflected.y);
    assert_eq_64!(rotated.z, reflected.z);
}

// Projections

#[test]
fn linear_map_3d_projection_onto_x_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_line(Measure3d::<Unspecified, f64>::new(1., 0., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, 0.);
    assert_eq_64!(m2.z, 0.);
}

#[test]
fn linear_map_3d_projection_onto_y_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_line(Measure3d::<Unspecified, f64>::new(0., 1., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 0.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, 0.);
}

#[test]
fn linear_map_3d_projection_onto_z_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_line(Measure3d::<Unspecified, f64>::new(0., 0., 1.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 0.);
    assert_eq_64!(m2.y, 0.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_projection_onto_line() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_line(
        Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized(),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 1.5862068965517244);
    assert_eq_64!(m2.y, 2.3793103448275863);
    assert_eq_64!(m2.z, 3.172413793103449);
}

#[test]
fn linear_map_3d_projection_onto_yz_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_plane(Measure3d::<Unspecified, f64>::new(1., 0., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 0.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_projection_onto_xz_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_plane(Measure3d::<Unspecified, f64>::new(0., 1., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, 0.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_projection_onto_xy_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_plane(Measure3d::<Unspecified, f64>::new(0., 0., 1.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, 0.);
}

#[test]
fn linear_map_3d_projection_onto_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::projection_onto_plane(
        Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized(),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 6.413793103448276);
    assert_eq_64!(m2.y, 2.620689655172414);
    assert_eq_64!(m2.z, -5.172413793103448);
}

#[test]
fn linear_map_3d_projection_onto_plane_plus_onto_line() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let unit_vector = Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized();
    let projector_onto_line = LinearMap3d::projection_onto_line(unit_vector);
    let projector_onto_plane = LinearMap3d::projection_onto_plane(unit_vector);
    let projected_onto_line = projector_onto_line.apply_to(m1);
    let projected_onto_plane = projector_onto_plane.apply_to(m1);
    let estimate = projected_onto_line + projected_onto_plane;
    assert_eq_64!(m1.x, estimate.x);
    assert_eq_64!(m1.y, estimate.y);
    assert_eq_64!(m1.z, estimate.z);
}

// Reflections

#[test]
fn linear_map_3d_reflection_over_x_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_line(Measure3d::<Unspecified, f64>::new(1., 0., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, -5.);
    assert_eq_64!(m2.z, 2);
}

#[test]
fn linear_map_3d_reflection_over_y_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_line(Measure3d::<Unspecified, f64>::new(0., 1., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -8.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, 2.);
}

#[test]
fn linear_map_3d_reflection_over_z_axis() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_line(Measure3d::<Unspecified, f64>::new(0., 0., 1.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -8.);
    assert_eq_64!(m2.y, -5.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_reflection_over_line() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_line(
        Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized(),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -4.827586206896552);
    assert_eq_64!(m2.y, -0.24137931034482762);
    assert_eq_64!(m2.z, 8.344827586206897);
}

#[test]
fn linear_map_3d_reflection_over_yz_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_plane(Measure3d::<Unspecified, f64>::new(1., 0., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, -8.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_reflection_over_xz_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_plane(Measure3d::<Unspecified, f64>::new(0., 1., 0.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, -5.);
    assert_eq_64!(m2.z, -2.);
}

#[test]
fn linear_map_3d_reflection_over_xy_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_plane(Measure3d::<Unspecified, f64>::new(0., 0., 1.));
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 8.);
    assert_eq_64!(m2.y, 5.);
    assert_eq_64!(m2.z, 2.);
}

#[test]
fn linear_map_3d_reflection_over_plane() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::reflection_over_plane(
        Measure3d::<Unspecified, f64>::new(2., 3., 4.).normalized(),
    );
    let m2 = lm1.apply_to(m1);
    assert_eq_64!(m2.x, 4.827586206896552);
    assert_eq_64!(m2.y, 0.24137931034482762);
    assert_eq_64!(m2.z, -8.344827586206897);
}

// Scaling by two factors

#[test]
fn linear_map_3d_scaling() {
    let m1 = Measure3d::<Metre, f32>::new(8., 5., -2.);
    let lm = LinearMap3d::<f32>::scaling(3., 7., 6.);
    let m2 = lm.apply_to(m1);
    assert_eq!(m2.x, 8. * 3.);
    assert_eq!(m2.y, 5. * 7.);
    assert_eq!(m2.z, -2. * 6.);
}

// Inversion

#[test]
fn linear_map_3d_inverted() {
    let m = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm = LinearMap3d::<f64>::new([[1.2, 0.8, 4.7], [3.4, -1.3, 0.6], [9.1, -5., 0.2]]);
    let inverse_of_lm = lm.inverted();
    let transformed = lm.apply_to(m);
    assert!(m != transformed);
    let transformed_back = inverse_of_lm.apply_to(transformed);
    assert_eq_64!(transformed_back.x, 8.);
    assert_eq_64!(transformed_back.y, 5.);
    assert_eq_64!(transformed_back.z, -2.);
}

#[test]
fn linear_map_3d_combined_with() {
    let m1 = Measure3d::<Metre, f64>::new(8., 5., -2.);
    let lm1 = LinearMap3d::<f64>::new([[1.2, 0.8, 4.7], [3.4, -1.3, 0.6], [1.1, -5., 0.2]]);
    let lm2 = LinearMap3d::<f64>::new([[8.3, 1.2, 7.4], [-6.1, 0.4, -7.7], [4., 5.2, -3.1]]);

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
    assert_eq_64!(m3.z, -2.);

    // If lm1 and lm2 are swapped, and also their inverses are swapped,
    // the same result should be obtained.
    let lm1_and_then_lm2 = lm2.combined_with(&lm1);
    let m3 = lm1_and_then_lm2.apply_to(m1);
    let lm2_inverted_and_then_lm1_inverted = lm1.inverted().combined_with(&lm2.inverted());
    let m4 = lm2_inverted_and_then_lm1_inverted.apply_to(m3);
    assert_eq_64!(m4.x, 8.);
    assert_eq_64!(m4.y, 5.);
    assert_eq_64!(m4.z, -2.);
}

#[test]
fn linear_map_3d_formatting_with_no_padding() {
    let lm = LinearMap3d::<f64>::new([[1.2, 000.8, 1.2], [3.400, 1.3, 1.4], [8.7, 3.1, 5.8]]);
    assert_eq!(
        lm.to_string(),
        "[1.2 0.8 1.2]\n[3.4 1.3 1.4]\n[8.7 3.1 5.8]"
    );
}

#[test]
fn linear_map_3d_formatting_with_initial_padding() {
    let lm = LinearMap3d::<f64>::new([[1.2, 20.8, 2567.], [873.4, 1.3, 0.], [3.1, 3473.2, 10.]]);
    assert_eq!(
        lm.to_string(),
        "[  1.2   20.8 2567]\n[873.4    1.3    0]\n[  3.1 3473.2   10]"
    );
}

#[test]
fn linear_map_3d_formatting_with_final_padding() {
    let lm = LinearMap3d::<f64>::new([
        [1.254, 0.8, -0.3401],
        [3.4, 1.36, 45.958234],
        [2.64, 5.452710, 45.],
    ]);
    assert_eq!(
        lm.to_string(),
        "[1.254 0.8     -0.3401  ]\n[3.4   1.36    45.958234]\n[2.64  5.45271 45       ]"
    );
}

#[test]
fn linear_map_3d_formatting_with_both_padding() {
    let lm = LinearMap3d::<f64>::new([
        [1.254, 650., -872.],
        [98763.4, 1.7658909, 5.43],
        [756., 3.8472, 41.81],
    ]);
    assert_eq!(
        lm.to_string(),
        "[    1.254 650         -872   ]\n[98763.4     1.7658909    5.43]\n[  756       3.8472      41.81]"
    );
}
