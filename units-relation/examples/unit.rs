//! Rs-measures version of the example `unit.rs` of the crate `uom` version 0.35.0.
//! Example showing how to add new units to existing quantities.
mod units;
use rs_measures::traits::MeasurementUnit;
use units::{Length, Measure, Metre};

pub struct Smoot;
impl MeasurementUnit for Smoot {
    type Property = Length;
    const RATIO: f64 = 1.702;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " smoot";
}

fn main() {
    let l1 = Measure::<Metre, f32>::new(15.);
    let l2 = Measure::<Smoot, f32>::new(1.);

    println!("{} = {}", l1, l1.convert::<Smoot>());
    println!("{} = {}", l2, l2.convert::<Metre>());
}
/*
Rs-measures will print:
15 m = 8.813161 smoot
1 smoot = 1.702 m

UOM will print:
15 m = 8.813161 smoot
1 smoot = 1.702 m
*/
