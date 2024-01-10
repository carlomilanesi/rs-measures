# Architecture

## Limitations

This library is not meant to support:
* An exploratory style of development. The Rust language itself was not designed for that. Those who need a more interactive environment should use an interpreted language, like Scratch, Python or Wolfram.
* Advanced theoretical physics. Modern theoretical physics uses concepts like a space with more than 3 dimensions, or with a non-Euclidean geometry.
* Quantities whose units may have a varying value, like prices.
* Integer or fixed-point numbers.
* Arbitrary-precision numbers. Currently, the value type can be only `f32` and `f64`. Maybe that in the future a higher-precision value type will be supported, possibly having 80 or 128 bits.
* Powerful linear algebra algorithms. Currently some linear and affine transformations are supported. Maybe some others will be added, if they are simple enough.

## Why only three dimensions

This library is meant to support computations commonly performed in software used for engineering. Such systems use typically only 1, 2, or 3 dimensions di represent quantities.

## Why macros are used

The crate `rs_measures`, instead of being a library which defines items inside it, letting application code to use such items, actually is essentially a set of the three macros: `define_measure_1d`, `define_measure_2d`, and `define_measure_3d`.

Let's see why this solution has been chosen.

This library has the purpose of allowing the user to write code like the following one. First, we define some properties to measure, like `ElectricCurrent`, `ElectricCharge`, and `Time`, using these statements:
```rust
pub struct ElectricCurrent;
pub struct ElectricCharge;
pub struct Time;
```

Then, we define some units of measurement for the defined properties, like `Ampere` for `ElectricCurrent`, `Coulomb` for `ElectricCharge`, and `Second` for `Time`, using these statements:
```rust
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
```rust
    use rs_measures::measure1d::Measure;
    let charge = Measure::<Coulomb>::new(6.);
    let time = Measure::<Second>::new(2.);
    let mut current: Measure<Ampere>;
```

Though, the operations involving values of more that one property are not allowed yet. For example, the following statement, albeit reasonable, would generate a syntax error:
```rust
    current = charge / time;
```

The main error message is: ``cannot divide `Measure<Coulomb>` by `Measure<Second>` ``. A help message is: ``the trait `Div<Measure<Second>>` is not implemented for `Measure<Coulomb>` ``.

So, there is the need to implement the trait `Div<Measure<Second>>` for the type `Measure<Coulomb>`.

The Rust language specifies that *only traits defined in the current crate can be implemented for types defined outside of the crate*. So, either the trait or the type for which it is implemented must be defined in the same crate in which there is the implementation.

Let's consider these alternatives:
1. Either the trait or the type are defined in our library. In such a case, we can implement that trait inside our library.
2. Either the trait or the type are defined in our application. In such a case, we can implement that trait inside our application.

The trait is `core::ops::Div`, and so it is defined neither in our library nor in our application.

The type for which it is implemented is `Measure<Coulomb>`. We need it to be defined in the library or in the application. By design, we want to keep `Coulomb` in the application. So, `Measure<Coulomb>` cannot be in the library.
But to keep the `Measure<Coulomb>` in the application, we must define also `Measure` in the application.
In such a way, that trait implementation can be defined inside the application.

To define both `Measure` and the `Div` in the application, without writing all the code there, the solution is to define macros. Such macros are defined in libraries, but they are expanded in the application, and both `Measure` and the `Div` implementation appear to the compiler as they were defined in the application.

So the crate `rs_measure` does not define directly the type `Measure` and other types. Instead it exposes, in addition to some auxiliary traits and functions, only three macros. When such macros are called, the features of the crate are expanded inside the caller code.

In particular, one of such feature is the generic type `Measure`, that so becomes a user-defined type. Being `Measure` a user-defined type, a standard-library trait can be implemented for it.

The crate `rs_measure` is designed to be used by applications needing only 1-dimensional measure, or by applications needing only 1-dimensional and 1-dimensional measures, or by applications needing also 3-dimensional measures. It would be a useless overhead to provide more dimensions than what is required.

Therefore the `rs_measure` provide essentially three declarative macros:
* `define_measure_1d`: Defines the 1-dimensional types `Measure` and `MeasurePoint`, and the angle types `UnsignedDirection` and `SignedDirection`.
* `define_measure_2d`: Defines the 2-dimensional types `Measure2d` and `MeasurePoint2d`, and the 2-dimensional transformation types `LinearMap2d` and `AffineMap2d`. In addition, it includes `define_measure_1d`.
* `define_measure_3d`: Defines the 3-dimensional types `Measure3d` and `MeasurePoint3d`, and the 3-dimensional transformation types `LinearMap3d` and `AffineMap3d`. In addition, it includes `define_measure_2d` (and consequently also `define_measure_1d`).

To implement the mixed-unit operations, like the division of a measure in coulomb by a measure in seconds, a procedural macro has been designed, named `define_units_relation`. Procedural macros must be defined in a distinct crate, and so the crate `units-relation` has been created just to define this procedural macro.
