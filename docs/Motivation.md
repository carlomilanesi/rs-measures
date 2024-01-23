# Motivation

This document describes the rationale for the crates `rs-measures` and `units-relation`.

Using primitive Rust data types to store the value of physical or geometrical quantities, several programming errors are possible. But some of them can be avoided, at no run-time cost, by encapsulating these values in custom types.

## Mixing measures having different unit of measurement

Consider the following Rust code:

```rust
let mass1 = 1.2; // In kilograms.
let mut mass2: f64; // In pounds.
mass2 = mass1; // Wrong.
fn set_mass_in_pounds(val: f64) { /* ... */ }
set_mass_in_pounds(mass1); // Wrong.
```

In the third statement, a value meant to be in kilograms is assigned to a variable meant to be in pounds. In the last statement, a function meant to receive a value in pounds receives a value meant to be in kilograms.

This is avoided by using this code:

```rust
let mass1 = Measure::<KiloGram>::new(1.2);
let mut mass2: Measure<Pound>;
mass2 = mass1; // Compilation error.
fn set_mass(val: Measure<Pound>) { /* ... */ }
set_mass(mass1); // Compilation error.
```

The third and the last statements cause compilation errors for type mismatch, as the values are encapsulated in objects whose types is characterized by a unit of measurement, and any value must have the same type of the variable to which it is assigned.

## Unit conversions

Consider the following Rust code:

```rust
let mass1 = 1.2; // kilograms
let mut mass2: f64; // pounds
mass2 = mass1 * 0.45359237; // Wrong conversion operation; it should be a division.
mass2 = mass1 / 453.59237; // Wrong conversion ratio; it should be 0.45359237.
```

The third statement tries to convert a value from kilograms to pounds, but it uses a multiplication instead of a division. Also the fourth statement tries to do the same unit conversion, but it uses a wrong divisor.

This is avoided by using the following code:

```rust
let mass1 = Measure::<KiloGram>::new(1.2);
let mut mass2: Measure<Pound>;
mass2 = mass1.convert();
// or
let mass3 = mass1.convert::<Pound>();
```

The third statement simply invokes the `convert` method. The correct formula is inferred by using the type of the destination variable. Instead, in the last statement, the destination variable does not have a type annotation, and so the unit conversion must specify the destination unit of measurement.

## Non-standard units of measurement

Some existing libraries, for example the UOM crate, encapsulate each value in an object which represents that value in a standard unit of measurement, typically taken from the SI international standard.
This raises the problem of how to handle non-standard units, like milligrams or pounds for mass.
These libraries always use internally a standard unit, and so they can convert, at run-time, any input value from a user-chosen unit to a standard unit, and any output value from a standard unit to a user-chosen unit.

Such technique has the following drawbacks:

1. It forces developers to use units that are different from the ones actually seen by users. For example, a developer wants to use only the pound as a unit of mass. But if he tries to explore with a debugger the contents of a mass object, he finds a value in kilograms.
2. It may introduce rounding errors.
3. It introduces a small conversion overhead for every I/O operation.

Instead, when using rs-measures, the developer can choose the pound as a unit of the mass, so that the input is in pounds, the computations are in pounds, and the output is in pounds. This eases debugging, avoids unexpected rounding errors, and avoids run-time conversion overhead.

Regarding the rounding errors, consider, for example, the following Rust code using the UOM crate:

```rust
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::f32::*;
use uom::si::mass::{gram, kilogram, pound};

fn main() {
    let mass1 = Mass::new::<gram>(8030.);
    let mass2 = Mass::new::<pound>(27.);
    let as_grams = Mass::format_args(gram, Abbreviation);
    let as_kilograms = Mass::format_args(kilogram, Abbreviation);
    let as_pounds = Mass::format_args(pound, Abbreviation);
    println!(
        "{}, {}, {}.",
        as_grams.with(mass1),
        as_kilograms.with(mass1),
        as_pounds.with(mass2)
    );
}
```

It defines the variables `mass1` and `mass2` by specifying a value in grams and a value in pounds, but it stores internally those values in kilograms. Then it prints the first variable using two different formatters, one for grams, and one for kilograms, and the second variable using the formatter for pounds. The output is: `8030.0005 g, 8.030001 kg, 27.000002 lb.`.

As you can see, even the value in grams and the value in pounds are different from the original values. This happens even with such small integer values, which usually do not introduce rounding errors in floating-point numbers.

The corresponding program using rs-measures is this:

```rust
using measures::{Measure, units::{Gram, KiloGram, Pound}};
fn main() {
    let mass1 = Measure::<Gram, f32>::new(8030.);
    let mass2 = Measure::<Pound, f32>::new(27.);
    println!("{}, {}, {}.", mass1, mass1.convert::<KiloGram>(), mass2);
}
```

It will print `8030 g, 8.030001 kg, 27 lb.`.

Only the value in kilograms is not exact, because its value is stored internally in grams, and the conversion to kilograms introduced a rounding error.

## All-encompassing libraries

To represent all possible physical or geometrical properties, such libraries try to define inside them all existing properties and all existing units.
This has the following drawbacks:
1. It makes the library heavy to compile, as the resulting types are quite numerous.
2. If the library is not extensible, it may be incomplete, as there are some little used units that some applications may require and which have not been included in the library. If it is extensible, it is usually somewhat difficult to extend it.
3. It forces developers to use a specific natural language to name the properties and their units of measurements, instead of the natural language of the developers. For example, some people prefer to write `metre` and others prefer to write `meter`, for the same unit.

Instead, the library `rs-measures` has just one built-in property, `Angle`, and just one built-in unit of measurement for it, `Radian`.
Every other property and every other unit of measurement can be easily defined by the application developers.
In this way:
1. The library uses little compile-time and run-time resources, because it will include only the few properties and units needed by the application.
2. The library is by-design easily extensible, by adding custom properties and units.
3. The developers can define their preferred names for the properties and the units of measurement, and also the preferred unit-of-measurement suffixes for outputting measures.

## Specifying derived units

If you have a value representing a mass and another value representing a volume, the division of the first value by the second value must represent a mass density.
This can be enforced by specifying relationships among units.
In existing libraries, these relationships are built-in, and implemented in very complex types, in a way that cannot be modified by the application developers.
This has the following drawbacks:
1. The complexity of the resulting types makes the application heavy to compile.
2. The complexity of the resulting types makes the compiler error messages quite cryptic.
3. No relationships can be added by application developers.

Instead, the library `rs-measures` has no built-in relationships to define derived properties.
Every derivation relationship can be easily defined by the application developers.
In this way:
1. The simplicity of the resulting types makes the application quick to compile.
2. The simplicity of the resulting types makes the compiler error messages quite clear.
3. Application developers can define relationships for exotic properties or units of measurement.

Regarding the compilation error messages, let's see some examples.
If, using the UOM crate, we write the statement `let _: Length = Mass::new::<gram>(0.);`, which tries to assign a mass to a length, we get the following hard-to-understand compilation error message, shown in a 110-column terminal:

```text
error[E0308]: mismatched types
  --> src/main.rs:14:21
   |
14 |     let _: Length = Mass::new::<gram>(0.);
   |            ------   ^^^^^^^^^^^^^^^^^^^^^ expected `Z0`, found `PInt<UInt<..., ...>>`
   |            |
   |            expected due to this
   |
   = note: expected struct `Quantity<dyn Dimension<I = ..., J = ..., Kind = ..., L = ..., M = ..., N = ..., T 
= ..., Th = ...>, ..., ...>` (`Z0`)
              found struct `Quantity<dyn Dimension<I = ..., J = ..., Kind = ..., L = ..., M = ..., N = ..., T 
= ..., Th = ...>, ..., ...>` (`PInt<UInt<..., ...>>`)
```

Instead, using the crate `rs-measures`, if we write the statement `let _: Measure<Metre> = Measure::<Gram>::new(0.);`, which tries to assign a mass in grams to a length in metres, we get the following compilation error message:

```text
error[E0308]: mismatched types
  --> src/main.rs:50:29
   |
50 |     let _: Measure<Metre> = Measure::<Gram>::new(0.);
   |            --------------   ^^^^^^^^^^^^^^^^^^^^^^^^ expected `Measure<Metre>`, found `Measure<Gram>`
   |            |
   |            expected due to this
   |
   = note: expected struct `units::Measure<units::Metre>`
              found struct `units::Measure<units::Gram>`
```

From it, it is clear that it was used unit `Gram` where unit `Metre` was expected (or vice versa).

Instead, if we write the statement `let _ = Measure::<Metre>::new(0.).convert::<Second>();`, that tries to convert a measure in metres into a measure in seconds, the generated error message is this one:

```text
error[E0271]: type mismatch resolving `<Second as MeasurementUnit>::Property == 
Length`
    --> src/main.rs:10:49
     |
10   |     let _ = Measure::<Metre>::new(0.).convert::<Second>();
     |                                                 ^^^^^^ type mismatch resolving `<Second as MeasurementU
nit>::Property == Length`
     |
note: expected this to be `Length`
    --> src/units.rs:2139:21
     |
2139 |     type Property = Time;
     |                     ^^^^
note: required by a bound in `units::Measure::<Unit, Number>::convert`
    --> src/units.rs:1:1
     |
1    | rs_measures::define_1d! {}
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Measure::<Unit, Number>::convert`
```

From it, it is enough clear that, for the destination unit of the conversion, a unit of `Length` was expected, but a unit of `Time` was specified.

## Meaningless operations for measure points

There are other features that makes this library useful to prevent programming errors.

Consider the following Rust code:

```rust
let position1 = 1.2;
let position2 = 2.3;
let displacement1 = 0.3;
let position3 = position1 + displacement1; // Meaningful.
let what1 = position1 + position2; // Meaningless.
let displacement2 = position2 - position1; // Meaningful.
let what2 = position1 * 2.; // Meaningless.
```

The value of the variable `position3` is a position, obtained by incrementing another position by a displacement. It is a meaningful operation.

Instead, the value of the variable `what1` is obtained by incrementing a position by another position. It is a meaningless operation, but it is allowed by the compiler.

The value of the variable `displacement2` is a displacement, obtained by computing the difference between two positions. It is the movement to go from the `position1` to `position2`. It is a meaningful operation.

Instead, the variable `what2` is obtained by multiplying a position by 2. It is a meaningless operation, but it is allowed by the compiler.

This can be avoided by using this code:

```rust
let position1 = MeasurePoint::<Metre, f64>::new(1.2);
let position2 = MeasurePoint::<Metre, f64>::new(2.3);
let displacement1 = Measure::<Metre, f64>::new(0.3);
let position3 = position1 + displacement1; // Allowed.
let what1 = position1 + position2; // Compilation error.
let displacement2 = position2 - position1; // Allowed.
let what2 = position1 * 2.; // Compilation error.
```

In this code, displacements are encapsulated in objects of type `Measure`, and positions are encapsulated in objects of type `MeasurePoint`. Using such types, the attempts to add two positions or to multiply a position by a number cause compilation errors.

## Different origins for measure points

Consider the following Rust code:

```rust
let fahrenheit_temperature_variation = 3.;
let celsius_temperature_variation = fahrenheit_temperature_variation
    / 9. * 5.; // Right.
let fahrenheit_temperature_point = 70.;
let celsius_temperature_point = fahrenheit_temperature_point
    / 9. * 5.; // Wrong; different scale origin.
```

To compute the value to assign to the variable `celsius_temperature_variation`, a temperature variation is correctly converted from the Fahrenheit scale to the Celsius scale.
Instead, to compute the value to assign to the variable `celsius_temperature_point`, the same formula is wrongly applied to convert a temperature point from the Celsius scale to the Fahrenheit scale.
It is wrong, because it does not take into account the fact that these two scales have different origins.

This is avoided by using this code:

```rust
let fahrenheit_temperature_variation = Measure::<Fahrenheit>::new(3.);
let celsius_temperature_variation =
    fahrenheit_temperature_variation.convert::<Celsius>();
let fahrenheit_temperature_point = MeasurePoint::<Fahrenheit>::new(3.);
let celsius_temperature_point =
    fahrenheit_temperature_point.convert::<Celsius>();
```

Both unit conversions use the clause `.convert::<Celsius>()`; though, the second one uses the type `MeasurePoint`, and so it takes into account the different origins.

## Wrong derived units of measurement

Consider the following Rust code:

```rust
let distance = 8.; // Miles.
let time_elapsed = 2.; // Hours.
let average_speed_in_metres_per_second = distance / time_elapsed;
// Wrong; this value is in miles per hour.
```

The value of the variable `average_speed_in_metres_per_second` is computed in miles per hours, but it is wrongly assigned to a variable meant to keep a value in metres per second.

This is avoided by using this code:

```rust
let distance = Measure::<Mile>::new(8.);
let time_elapsed = Measure::<Hour>::new(2.);
let average_speed1: Measure<MilePerHour>
    = distance / time_elapsed; // Allowed.
// or
let average_speed2 = distance / time_elapsed; // Allowed.
let average_speed3: Measure<MetrePerSecond>
    = distance / time_elapsed; // Compilation error.
```

The compilation of the assignment to `average_speed1` checks that the value assigned has the same derived unit of measurement of the variable that receives that value.

The compilation of the assignment to `average_speed2` infers such unit of measurement.

The compilation of the assignment to `average_speed3` generates a type-mismatch compilation error.

## Plane (2D) And space (3D) measures

So far, we have seen features comparable to what is offered by most other crates which handles units of measurement.
Though, the library `rs-measures` has some multi-dimensional features which make it useful to work which bi-dimensional or three-dimensional quantities in an ergonomic and safe manner.

Some physical or geometrical properties are properly described as multidimensional. For example, a displacement in a plane has two dimensions (X and Y), and a displacement in space has three dimensions (X, Y, and Z). They are called “vector measures”. Plane vector measures (a.k.a. 2D measures) have two components, and space vector measures (a.k.a. 3D measures) have three components.

For vector measures, usually there is no reason to use a numeric type and a unit of measurement for one component, and a different numeric type or a different unit of measurement for another component. So, the library `rs-measures` allows to encapsulate all the components of a vector measure into a single object, like `Measure3<Metre, f64>`, in which all three components have `Metre` as unit of measurement and `f64` as inner type.

For example, to represent a planar force and a planar displacement, instead of writing this code:

```rust
let force = (Measure::<Newton>::new(3.), Measure::<Newton>::new(4.));
let displacement = (Measure::<Metre>::new(7.), Measure::<Metre>::new(1.));
```

You can and should write this code:

```rust
let force = Measure2d::<Newton>::new(3., 4.);
let displacement = Measure2d::<Metre>::new(7., 1.);
```

Similarly, it is possible to create 3D measures in space:

```rust
let force = Measure3d::<Newton>::new(3., 4., 5.);
let displacement = Measure3d::<Metre>::new(7., 1., -6.);
```

Points in a plane or in space can be represented by single objects:

```rust
let position_in_plane = MeasurePoint2d::<Metre>::new(3., 4.);
let position_in_space = MeasurePoint3d::<Metre>::new(3., 4., 5.);
```

Dot products and cross products between measures have the right units of measurement:

```rust
// In a plane
let force = Measure2d::<Newton>::new(3., 4.);
let displacement = Measure2d::<Metre>::new(7., 1.);
let work: Measure<Joule> = force * displacement;
let torque: Measure<NewtonMetre> = force.cross_product(displacement);
```

The value of `work` is obtained by multiplying `force`, a 2D force expressed in newtons, by `displacement`, a 2D displacement expressed in metres. Such multiplication, which uses the "`*`" operator, is the *dot product* between the two 2D vectors. So, the result is an energy (or work), expressed in joules.

The value of `torque` is obtained by computing the cross product between `force` and `displacement`, resulting in a torque, expressed in newton-metres.

Notice that the unit `Joule` and `NewtonMetre` are different types, even if they have the same dimensional composition (mass space space / time time).

Similarly it happens in 3D space:

```rust
// In space
let force = Measure3d::<Newton>::new(3., 4., 5.);
let displacement = Measure3d::<Metre>::new(7., 1., -6.);
let work: Measure<Joule> = force * displacement;
let torque: Measure3d<NewtonMetre> = force.cross_product(displacement);
```

Notice a difference: the 2D torque is a scalar (a `Measure`), while the 3D torque is a 3D vector (a `Measure3d`).

## Angles and directions

Angles deserve a special treatment. Consider the following Rust code:

```rust
let mut angular_position = 300.; // In degrees.
let rotation = 400.; // In degrees.
angular_position += rotation;
assert_eq!(angular_position, 700.);
```

The third statement increments an angular position by a rotation. The resulting position is 700 degrees, that is more than one cycle. It may be just what we needed, but it may be that we needed instead a position in the circumference, i.e. a directional angle. And in such a case, 700 degrees is not a valid result.

There are two widely used conventions to specify a direction: using an angle from 0 to 360 degrees, or an angle from -180 to +180 degrees. The former case, which uses only non-negative values, is computed by the the following statement:

```rust
let direction360 = angular_position % 360.;
assert_eq!(direction360, 340.);
```

The other case, using positive or negative values, is computed by the following statement:

```rust
let direction180 = (angular_position + 180.) % 360. - 180.;
assert_eq!(direction180, -20.);
```

Though, these formulas are error-prone, and depend on the angular unit of measurement. These unsafe expressions are avoided by using this code:

```rust
// An unconstrained angular position.
let mut angular_position = MeasurePoint::<Degree>::new(300.);

// A rotation.
let rotation = Measure::<Degree>::new(400.);

// The position is incremented by the rotation.
angular_position += rotation;
assert_eq!(angular_position.value, 700.);

// The method `to_unsigned_direction` converts the unconstrained angular position
// to a direction type, whose values are constrained to be in the range 0 to 360 degrees.
// Such a type is named `UnsignedDirection`, because it may have only non-negative values.
let unsigned_direction: UnsignedDirection::<Degree> = angular_position.to_unsigned_direction();
assert_eq!(unsigned_direction.value, 340.);

// The method `to_signed_direction` converts the unconstrained angular position
// to a direction type, whose values are constrained to be in the range -180 to 180 degrees.
// Such a type is named `SignedDirection`, because it may have negative or positive values.
let signed_direction: SignedDirection::<Degree> = angular_position.to_signed_direction();
assert_eq!(signed_direction.value, -20.);
```

## Vector and affine transformations

When using vector measures in a plane or in the 3D space, it is quite typical to need to perform vector operations. There are good linear algebra crates, like the crate `nalgebra`, which could be used for this. Though, when using them, it is not possible to keep the measurement unit information of components. Consider the following Rust program, which uses the crate `nalgebra`:

```rust
extern crate nalgebra;
use nalgebra::{Rotation2, Vector2};
fn main() {
    let displacement = Vector2::new(4., 5.);
    let rotation = Rotation2::new(0.1);
    let rotated_displacement = rotation * displacement;
}
```

First, it stores in `displacement` a 2D measure, with no units of measurement. Then, it stores in `rotation` a 2D linear transformation. Then, such rotation is applied to the displacement, using a matrix multiplication, obtaining a transformed 2D measure. Such resulting measure is stored in the variable `rotated_displacement`. Of course, such a variable is meant to have the same unit of measurement of the first variable, but this is not specified by the used types.

so, let’s try to attach units of measurement to these measures:

```rust
let displacement = Vector2::new(
    Measure::<Metre>::new(4.),
    Measure::<Metre>::new(5.),
);
let rotation = Rotation2::new(Measure::<Radian>::new(0.1));
```

The first statement is allowed, but the second one is not allowed by `nalgebra`.
This happens because, as argument of a call to `Rotation2::new`, `nalgebra` allows only some value which behaves like a number.
For example, we must be able to multiply it by itself, to compute its square root, and so on.
This is not available for all measures, in general.

Then, we can try to use a naked number for the rotation:

```rust
let displacement = Vector2::new(
    Measure::<Metre>::new(4.),
    Measure::<Metre>::new(5.),
);
let rotation = Rotation2::new(0.1);
let rotated_displacement = rotation * displacement;
```

This causes an error on the last statement, as `nalgebra` is not able to apply a linear transformation to a vector of `Measure` values, for the same reason that measures do not implement all the traits implemented by numbers.
And even if it were able, probably the resulting value, which is assigned to `rotated_displacement`, wouldn’t have the proper unit of measurement.

Instead, using the library `rs-measures`, it is possible to compile the following code:

```rust
// A 2D measure in metres.
let displacement = Measure2d::<Metre>::new(4., 5.);

// A 2D linear transformation, corresponding
// to a counterclockwise rotation on 0.1 radians.
let rotation = LinearMap2d::rotation(
    Measure::<Radian>::new(0.1));

// The rotation is applied to the planar measure,
// obtaining a rotated measure,
// having the same type of the original measure.
let rotated_displacement = rotation.apply_to(displacement);
```

The above code uses the type `LinearMap2d` to transform objects of type `Measure2d`. To transform objects of type `MeasurePoint2d`, the type `AffineMap2d` should be used instead.

And when working in three dimensions, the types `LinearMap3d` and `AffineMap3d` should be used.
