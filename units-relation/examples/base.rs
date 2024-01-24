//! Rs-measures version of the example `base.rs` of the crate `uom` version 0.35.0.
//! Example showing how to create a set of `Quantity` type aliases for a different set of base
//! units.
mod units;
use units::{CentiMetre, Measure, Metre, Second};

fn main() {
    let l1 = Measure::<Metre, f32>::new(1.);
    let l2 = Measure::<CentiMetre, f32>::new(1.);
    let t1 = Measure::<Second, f32>::new(15.);

    println!("length: {:?}", l1);
    println!("length: {:?}", l2);
    println!("{:?} + {:?} = {:?}", l1, l2, l1 + l2.convert());
    println!("{:?} + {:?} = {:?}", l2, l1, l2 + l1.convert());
    println!("{:?} / {:?} = {:?}", l2, t1, l2 / t1);
}
/*
Rs-measures will print:
length: 1 m
length: 1 cm
1 m + 1 cm = 1.01 m
1 cm + 1 m = 101 cm
1 cm / 15 s = 0.06666667 cm/s

UOM will print:
length: 1.0 m^1
length: 1.0 cm^1
1.0 m^1 + 1.0 cm^1 = 1.01 m^1
1.0 cm^1 + 1.0 m^1 = 101.0 cm^1
1.0 cm^1 / 15.0 s^1 = 0.06666667 cm^1 s^-1

After replacing every `{:?}` with `{}`, Rs-measures will print the same.
UOM will emit compilation error.
*/
