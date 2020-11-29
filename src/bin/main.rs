use rs_measures::{define_derived_measure_1_1, define_measure1d};
define_measure1d! {}

struct Length;
#[derive(Copy, Clone)]
struct Metre;
impl MeasurementUnit for Metre {
    type Quantity = Length;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " m";
}

struct Time;
#[derive(Copy, Clone)]
struct Second;
impl MeasurementUnit for Second {
    type Quantity = Time;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " s";
}

struct Velocity;
#[derive(Copy, Clone)]
struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Quantity = Velocity;
    const RATIO: f64 = 1.0;
    const OFFSET: f64 = 0.0;
    const SUFFIX: &'static str = " m/s";
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
TODO
- allow 2D and 3D measures
- allow composite quantites (like speed, angular speed,
  energy and angular momentum should be considered different
- allow vector and affine 2D and 3D transformations
- allow to have vectors and points for some quanties and only vectors for others
*/
