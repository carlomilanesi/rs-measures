use rs_measures::define_measure_1d;
define_measure_1d! {}

mod angles_decl;

#[test]
fn angle_units_consts() {
    use rs_measures::{angle::Radian, traits::AngleMeasurementUnit};
    let pi = std::f64::consts::PI;

    // Radians
    assert_eq!(Radian::CYCLE_FRACTION, 2. * pi);
    assert_eq!(Radian::RATIO, 1.);
    assert_eq!(Radian::OFFSET, 0.);

    // Degrees
    assert_eq!(angles_decl::Degree::CYCLE_FRACTION, 360.);
    assert_eq!(angles_decl::Degree::RATIO, 2. * pi / 360.);
    assert_eq!(angles_decl::Degree::OFFSET, 0.);

    // Cycles
    assert_eq!(angles_decl::Cycle::CYCLE_FRACTION, 1.);
    assert_eq!(angles_decl::Cycle::RATIO, 2. * pi);
    assert_eq!(angles_decl::Cycle::OFFSET, 0.);
}
