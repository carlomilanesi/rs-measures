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
fn linear_map_2d_values() {
    let lm = LinearMap2d::<f32> {
        c: [[1.2, 2.3], [3.4, 7.8]],
    };
    assert_eq!(lm.c[0][0], 1.2);
    assert_eq!(lm.c[0][1], 2.3);
    assert_eq!(lm.c[1][0], 3.4);
    assert_eq!(lm.c[1][1], 7.8);
}

#[test]
fn linear_map_2d_new() {
    let lm = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    assert_eq!(lm.c[0][0], 1.2);
    assert_eq!(lm.c[0][1], 2.3);
    assert_eq!(lm.c[1][0], 3.4);
    assert_eq!(lm.c[1][1], 7.8);
}

#[test]
fn linear_map_2d_rotation() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::rotation(Measure::<Degree, f32>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, -5.);
    assert_eq_32!(m2.y, 8.);
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

#[test]
fn linear_map_2d_projection_by_angle_point() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_angle_point(MeasurePoint::<Degree, f32>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, 0.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_projection_by_signed_direction() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_signed_direction(
        SignedDirection::<Degree, f32>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, 0.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_unsigned_direction() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::projection_by_unsigned_direction(
        UnsignedDirection::<Degree, f32>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, 0.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_projection_by_unit_vector() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let unit_vector = Measure2d::<Metre, f32>::new(0., 1.);
    assert_eq!(unit_vector.squared_norm(), 1.);
    let lm = LinearMap2d::<f32>::projection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, 0.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_reflection_by_angle_point() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_angle_point(MeasurePoint::<Degree, f32>::new(90.));
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, -8.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_reflection_by_signed_direction() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_signed_direction(
        SignedDirection::<Degree, f32>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, -8.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_reflection_by_unsigned_direction() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::reflection_by_unsigned_direction(
        UnsignedDirection::<Degree, f32>::new(90.),
    );
    let m2 = lm.apply_to(m1);
    assert_eq_32!(-8., m2.x);
    assert_eq_32!(5., m2.y);
}

#[test]
fn linear_map_2d_reflection_by_unit_vector() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let unit_vector = Measure2d::<Metre, f32>::new(0., 1.);
    assert_eq!(unit_vector.squared_norm(), 1.);
    let lm = LinearMap2d::<f32>::reflection_by_unit_vector(unit_vector);
    let m2 = lm.apply_to(m1);
    assert_eq_32!(m2.x, -8.);
    assert_eq_32!(m2.y, 5.);
}

#[test]
fn linear_map_2d_scaling() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm = LinearMap2d::<f32>::scaling(3., 7.);
    let m2 = lm.apply_to(m1);
    assert_eq!(m2.x, 8. * 3.);
    assert_eq!(m2.y, 5. * 7.);
}

#[test]
fn linear_map_2d_inverted() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm1 = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    let lm2 = lm1.inverted();
    let m2 = lm1.apply_to(m1);
    let m3 = lm2.apply_to(m2);
    assert_eq_32!(m3.x, m1.x);
    assert_eq_32!(m3.y, m1.y);
}

#[test]
fn linear_map_2d_apply_after() {
    let m1 = Measure2d::<Metre, f32>::new(8., 5.);
    let lm1 = LinearMap2d::<f32>::new([[1.2, 2.3], [3.4, 7.8]]);
    let lm2 = lm1.inverted();
    let lm3 = lm1.apply_after(&lm2);
    let m2 = lm3.apply_to(m1);
    assert_eq_32!(m2.x, m1.x);
    assert_eq_32!(m2.y, m1.y);
}
