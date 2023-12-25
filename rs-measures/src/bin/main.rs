use define_units_relation::define_units_relation;
use rs_measures::define_measure_3d;
define_measure_3d! {}

pub struct Dimensionless;

#[derive(Debug, Clone, Copy)]
pub struct Unspecified;
impl MeasurementUnit for Unspecified {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
impl VectorMeasurementUnit for Unspecified {}

#[derive(Debug, Clone, Copy)]
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

pub struct Length;

#[derive(Debug, Clone, Copy)]
pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Velocity;

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}
impl VectorMeasurementUnit for MetrePerSecond {}

pub struct Area;

#[derive(Debug, Clone, Copy)]
pub struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Property = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}";
}

pub struct Force;

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}
impl VectorMeasurementUnit for Newton {}

pub struct Torque;

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}
impl VectorMeasurementUnit for NewtonMetre {}

define_units_relation! { Metre / Second == MetrePerSecond }
define_units_relation! { Metre:2 / Second == MetrePerSecond:2 }
define_units_relation! { Metre * Metre == SquareMetre }

define_units_relation! { Metre:2 X Newton:2 == NewtonMetre }
define_units_relation! { Metre:2 X Metre:2 == Metre }

fn main() {
    let interval = Measure::<Second, f64>::new(4.);
    let distance1 = Measure::<Metre, f64>::new(30.);
    let speed1 = distance1 / interval;
    println!(
        "Distance: {}, Interval: {}, Speed: {}.",
        distance1, interval, speed1
    );
    let area1 = distance1 * distance1;
    println!("Area: {}.", area1);
    let distance2 = Measure2d::<Metre, f64>::new(17., 7.8);
    let speed2 = distance2 / interval;
    println!(
        "Distance: {}, Interval: {}, Speed: {}.",
        distance2, interval, speed2
    );

    let force2 = Measure2d::<Newton, f64>::new(3.5, -6.);
    println!("Torque: {}.", distance2.cross_product(force2));
    println!("Inverted torque: {}.", force2.cross_product(distance2));
    println!(
        "Self cross-product: {}.",
        distance2.cross_product(distance2)
    );
    println!(
        "{}",
        LinearMap2d::<f64>::new([[1.2678, -0.8], [873.4, 1.34]])
    );
    println!(
        "{}",
        AffineMap2d::<MetrePerSecond, f64>::new([[7., -0.8, 897.23455663], [873.4, 1.3487, -5.]])
    );
    println!(
        "{}",
        LinearMap3d::<f64>::new([[1.2678, -0.8, 54.3], [873.4, 1.34, 2.], [-65., 34.7, 1.]])
    );
    println!(
        "{}",
        AffineMap3d::<MetrePerSecond, f64>::new([
            [7., -0.8, 87.2, 1_897.23455663],
            [873.4, 1.3487, 9.12, -5.],
            [-4.2, -45.00, 7.7000, 32.8]
        ])
    );

    println!(
        "{}",
        AffineMap3d::<Metre, f64>::rotation::<Degree, Unspecified>(
            Measure::<Degree, f64>::new(1.),
            Measure3d::<Unspecified, f64>::new(3., 4., 0.3).normalized(),
            MeasurePoint3d::<Metre, f64>::new(6., 4., 2.),
        )
        .apply_to(MeasurePoint3d::<Metre, f64>::new(8., 5., 2.))
    );

    println!(
        "{}",
        AffineMap3d::<Metre, f64>::scaling(
            MeasurePoint3d::<Metre, f64>::new(6., 4., -7.),
            2.,
            3.,
            4.,
        )
        .apply_to(MeasurePoint3d::<Metre, f64>::new(8., 5., 2.))
    );
}

/*
use rs_measures::define_measure_1d;
//use rs_measures::define_measure_2d;
define_measure_1d!{}
//define_measure_2d!{}

mod units;
use units::*;

fn main() {
    let unsigned_up = UnsignedDirection::<f64, Degree>::new(90.);
    let signed_down = SignedDirection::<f64, Degree>::new(270.);
    let signed_up = unsigned_up.to_signed_direction();
    let unsigned_down = signed_down.to_unsigned_direction();
    println!("{}, {}\n{}, {}",
        unsigned_up, unsigned_down, signed_up, signed_down);
    let right_angle = Measure::<Turn, f64>::new(0.25);
    let to_left = MeasurePoint::<Turn, f64>::new(0.) + right_angle * 6.;
    println!("{}, {}",
        UnsignedDirection::from_measure_point(to_left),
        SignedDirection::from_measure_point(to_left)
    );
}
*/
