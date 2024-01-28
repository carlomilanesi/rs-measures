//! Rs-measures version of the example `mks.rs` of the crate `uom` version 0.35.0.
//! Example showing how to create a custom system of quantities.
/*
Rs-measures will print:
100 m = 328.08398 ft

UOM will print:
100 m = 328.08398 ft
*/

rs_measures::define_1d! {}

fn main() {
    let l1 = Measure::<Metre, f32>::new(100.);

    println!("{} = {}", l1, l1.convert::<Foot>());
}

pub struct Length;

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

pub struct Foot;
impl MeasurementUnit for Foot {
    type Property = Length;
    const RATIO: f64 = 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft";
}
