use rs_measures::{define_derived_measure_1_1, define_measure1d, define_measure2d};
define_measure1d! {}
define_measure2d! {}

struct Length;

#[derive(Copy, Clone)]
struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

struct Time;

#[derive(Copy, Clone)]
struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " s";
}

struct Velocity;

#[derive(Copy, Clone)]
struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " m/s";
}
impl VectorMeasurementUnit for MetrePerSecond {}

struct Energy;

#[derive(Copy, Clone)]
struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " J";
}

define_derived_measure_1_1! {Second, MetrePerSecond, Metre}

fn main() {
    let length1 = Measure::<f64, Metre>::new(20.);
    let length2 = Measure::<f64, Metre>::new(30.);
    let length3: Measure<f64, Metre> = length1 + length2;
    let time = Measure::<f64, Second>::new(4.);
    let speed: Measure<f64, MetrePerSecond> = length3 / time;
    println!(
        "{} plus {} is {}. They are run in {}, at an average speed of {}.",
        length1, length2, length3, time, speed
    );

    let _move1d = Measure::<f64, Metre>::new(2.);
    let _move2d = Measure2d::<f64, Metre>::new(2., 3.);
    let _energy1d = Measure::<f64, Joule>::new(4.);
    //let _energy2d = Measure2d::<f64, Joule>::new(4., 5.);
}

/*
features:
- define measures as numbers with unit in their type
- allow vector operations on measures
- define measure points as numbers with unit in their type
- allow affine operations on measure points
- allow unit convertions for measures,
  forbidding conversions between different quantities.
- allow unit convertions for measure points
  forbidding conversions between different quantities.
- allow lossless number casting for measures
- allow lossless number casting for measure points
- allow lossy number casting for measures
- allow lossy number casting for measure points
- allow 2D and 3D measures
- allow composite quantites (like speed, angular speed,
  energy and angular momentum should be considered different
- forbid vector measures for some kind of units
TODO
- allow vector and affine 2D and 3D transformations
*/
