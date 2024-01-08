# Architecture

## Why macros are used

The crate `rs_measures`, instead of being a library which defines items inside it, letting application code to use such items, actually is essentially a set of the three macros: `define_measure_1d`, `define_measure_2d`, and `define_measure_3d`.

Let's see why this solution has been chosen.

This library has the purpose of allowing the user to write code like the following one. First, we define some properties to measure, like `ElectricCurrent`, `ElectricCharge`, and `Time`, using these statements:
```
pub struct ElectricCurrent;
pub struct ElectricCharge;
pub struct Time;
```
Then, we define some units of measurement for the defined properties, like `Ampere` for `ElectricCurrent`, `Coulomb` for `ElectricCharge`, and `Second` for `Time`, using these statements:
```
use rs_measures::traits::MeasurementUnit;

pub struct Ampere;
impl MeasurementUnit for Ampere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A";
}

pub struct Coulomb;
impl MeasurementUnit for Coulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C";
}

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}
```
With the above definitions, we can define measures having the defined units, with these statements:
```
    use rs_measures::measure1d::Measure;
    let charge = Measure::<Coulomb>::new(6.);
    let time = Measure::<Second>::new(2.);
    let mut current: Measure<Ampere>;
```
Though, the operations involving values of more that one property are not allowed yet. For example, the following statement, albeit reasonable, would generate a syntax error:
```
    current = charge / time;
```
The main error message is: ``cannot divide `Measure<Coulomb>` by `Measure<Second>` ``. A help message is: ``the trait `Div<Measure<Second>>` is not implemented for `Measure<Coulomb>` ``.

So, there is the need to implement the trait `Div<Measure<Second>>` for the type `Measure<Coulomb>`.
Such a trait cannot be implemented inside the crate `rs_measure`, because that trait depends on the type `Second`, and that type depend on the type `Coulomb`. The types `Second` and `Coulomb` are defined by the application code, not by the library code. 
So, that trait implementation must be performed inside the application.

Though, the generic trait `Div` has been declared in the standard library, and the generic type `Measure` has been declared in the crate `rs_measure`.
This forbids such a trait implementation.

A workaround is to declare the type `Measure` in the current application crate, instead of in the crate `rs_measure`.

To that goal, the crate `rs_measure` does not expose directly its features. Actually it exposes, in addition to some auxiliary traits and functions, only three macros. When such macros are called, the features of the crate are expanded inside the caller code.

In particular, one of such feature is the generic type `Measure`, that so becomes a user-defined type. Being `Measure` a user-defined type, a standard-library trait can be implemented for it.
