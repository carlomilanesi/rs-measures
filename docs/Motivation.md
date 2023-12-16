# Rs-Measures: Rationales for a Library to Handle Measures

Using primitive Rust data types to store the value of physical or geometrical quantities, several programming errors are possible. But some of them can be avoided, at no run-time cost, by using the Rs-Measures crate.

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
let mass1 = Measure::<KiloGram, f64>::new(1.2);
let mut mass2: Measure<Pound, f64>;
mass2 = mass1; // Compilation error.
fn set_mass(val: Measure<Pound, f64>) { /* ... */ }
set_mass(mass1); // Compilation error.
```

The third and the last statements cause compilation errors for type mismatch, as the values are encapsulated in objects whose types is characterized by a unit of measurement, and any value must have the same type of the variable to which it is assigned.

## Bad unit conversions

Consider the following Rust code:

```rust
let mass1 = 1.2; // kilograms
let mut mass2: f64; // pounds
mass2 = mass1 * 0.45359237; // Wrong conversion operation; it should be a division.
mass2 = mass1 / 453.59237; // Wrong conversion ratio; it should be 0.45359237.
```

The third statement tries to convert a value from kilograms to pounds, but it uses a multiplication instead of a division. Also the fourth statement tries to do the same unit conversion, but uses a wrong divisor.

This is avoided by using the following code:

```rust
let mass1 = Measure::<KiloGram, f64>::new(1.2);
let mut mass2: Measure<Pound, f64>;
mass2 = mass1.convert();
// or
let mass3 = mass1.convert::<Pound>();
```

The third statement simply invokes the `convert` method. The correct formula is inferred by using the type of the destination variable. The last statement must specify the destination unit of measurement, as for the destination variable no type is specified.

## User-specified properties and units of measurement

Some existing libraries, for example the UOM crate, solve the previous issues by defining a set of standard units, typically taken from the SI international standard, and, at run-time, they convert all input values from the user-chosen units to such standard units, and they convert all output values from such standard units to user-chosen units.

Such technique has the following drawbacks:

1. It forces programmers to use units that are different from the ones actually seen by users. For example, a developer wants to use everywhere only pounds. But if he tries to explore with a debugger the contents of a weight object, he finds a value in kilograms.
2. It may introduce rounding errors.
3. It introduces a small conversion overhead for every I/O operation. Instead, when using Rs-Measures, if the input is always in pounds, the number is always stored in pounds, and the output is always in pounds, and so no run-time conversion operations are needed.

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

It defines the `mass1` and `mass2` variables by specifying a value in grams and a value in pounds, but it stores internally those values in kilograms. Then it prints the first variable using two different formatters, one for grams, and one for kilograms, and the second variable using the formatter for pounds. The output is: `8030.0005 g, 8.030001 kg, 27.000002 lb.`.

As you can see, even the value in grams and the value in pounds are different from the original values. This happens even they being small integer values, which usually do not introduce rounding errors in floating-point numbers.

The corresponding program using Rs-Measures is this:

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

In addition, trying to define in a library all possible properties and units has the following drawbacks:

1. It makes the library unnecessarily heavy to compile, as the resulting types are very complex.
2. Because of the complexity of the resulting types, the compiler error messages are quite verbose.
3. If the library is not extensible, it may be incomplete, as there are some little used units that some application may require and that have not been included in the library. If it is extensible, it is usually somewhat difficult to do.
4. It forces programmers to use a specific natural language to name the properties and their units of measurements, instead of the natural language of the programmer. For example, some people prefer to write "metre" and others prefer to write "meter".

Therefore, it is better to allow the application programmers to define only the units of measurement they need, and which they are going to use in their software application, instead of trying to cover all existing units of measurement.

Regarding the compilation error messages, let's see some examples. If, using the UOM crate, we write the statement `let _: Length = Mass::new::<gram>(0.);`, that tries to assign a mass to a length, we get the following compilation error message:

```text
  --> src/main.rs:106:21
   |
50 |     let _: Length = Mass::new::<gram>(0.);
   |            ------   ^^^^^^^^^^^^^^^^^^^^^ expected struct `uom::typenum::Z0`, found struct `uom::typenum::PInt`
   |            |
   |            expected due to this
   |
   = note: expected struct `uom::si::Quantity<(dyn uom::si::Dimension<T = uom::typenum::Z0, Th = uom::typenum::Z0, N = uom::typenum::Z0, J = uom::typenum::Z0, M = uom::typenum::Z0, I = uom::typenum::Z0, L = uom::typenum::PInt<uom::typenum::UInt<uom::typenum::UTerm, uom::typenum::B1>>, Kind = (dyn uom::Kind + 'static)> + 'static), (dyn uom::si::Units<f32, length = uom::si::length::meter, mass = uom::si::mass::kilogram, electric_current = uom::si::electric_current::ampere, time = uom::si::time::second, thermodynamic_temperature = uom::si::thermodynamic_temperature::kelvin, luminous_intensity = uom::si::luminous_intensity::candela, amount_of_substance = uom::si::amount_of_substance::mole> + 'static), _>`
              found struct `uom::si::Quantity<(dyn uom::si::Dimension<T = uom::typenum::Z0, Th = uom::typenum::Z0, N = uom::typenum::Z0, J = uom::typenum::Z0, M = uom::typenum::PInt<uom::typenum::UInt<uom::typenum::UTerm, uom::typenum::B1>>, I = uom::typenum::Z0, L = uom::typenum::Z0, Kind = (dyn uom::Kind + 'static)> + 'static), dyn uom::si::Units<f32, length = uom::si::length::meter, mass = uom::si::mass::kilogram, electric_current = uom::si::electric_current::ampere, time = uom::si::time::second, thermodynamic_temperature = uom::si::thermodynamic_temperature::kelvin, luminous_intensity = uom::si::luminous_intensity::candela, amount_of_substance = uom::si::amount_of_substance::mole>, _>`
```

Instead, if, using the Rs-Measures crate, we write the statement `let _: Measure<Metre, f32> = Measure::<Gram, f32>::new(0.);`, that tries to assign a mass in grams to a length in metres, the generated error message is:

```text
error[E0308]: mismatched types
  --> src/main.rs:50:34
   |
50 |     let _: Measure<Metre, f32> = Measure::<Gram, f32>::new(0.);
   |            -------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Metre`, found struct `Gram`
   |            |
   |            expected due to this
   |
   = note: expected struct `Measure<Metre, _>`
              found struct `Measure<Gram, _>`
```

From it, it is clear that it was used a Gram unit where a Metre unit was expected (or vice versa).

Instead, if we write the statement `let _ = Measure::<Metre, f32>::new(0.).convert::<Second>();`, that tries to convert a measure in metres into a measure in seconds, the generated error message is:

```text
   --> rs-measures/src/bin/main.rs:104:54
    |
104 |     let _ = Measure::<Metre, f32>::new(0.).convert::<Second>();
    |                                                      ^^^^^^ type mismatch resolving `<Second as MeasurementUnit>::Property == Length`
    |
note: expected this to be `Length`
   --> rs-measures/src/bin/main.rs:22:21
    |
22  |     type Property = Time;
    |                     ^^^^
note: required by a bound in `Measure::<Unit, Number>::convert`
   --> rs-measures/src/bin/main.rs:4:1
    |
4   | define_measure_2d! {}
    | ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Measure::<Unit, Number>::convert`
    = note: this error originates in the macro `rs_measures::define_measure_1d` which comes from the expansion of the macro `define_measure_2d` (in Nightly builds, run with -Z macro-backtrace for more info)
```

From it, it is clear that, for the destination unit of the conversion, a Time unit was specified, but a Length unit was expected.

## Meaningless operations for measure points

Consider the following Rust code:

```rust
let position1 = 1.2;
let position2 = 2.3;
let displacement1 = 0.3;
let position3 = position1 + displacement1; // Meaniningful.
let _ = position1 + position2; // Meaniningless.
let displacement2 = position2 - position1; // Meaniningful.
let _ = position1 * 2.; // Meaniningless.
```

In the fourth statement, a position is incremented by a displacement, obtaining another position. It is a meaningful operation.

In the fifth statement, a position in incremented by another position. It is a meaningless operation, but it is allowed by the compiler.

In the sixth line, the difference between two positions is computed, obtaining the displacement to go from the `position1` to `position2`. It is a meaningful operation.

In the seventh line, a position is multiplied by 2. It is a meaningless operation, but it is allowed by the compiler.

This is avoided by using this code:

```rust
let position1 = MeasurePoint::<Metre, f64>::new(1.2);
let position2 = MeasurePoint::<Metre, f64>::new(2.3);
let displacement1 = Measure::<Metre, f64>::new(0.3);
let position3 = position1 + displacement1; // Allowed.
let position3 = position1 + position2; // Compilation error.
let displacement2 = position2 - position1; // Allowed.
let _ = position1 * 2.; // Compilation error.
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

The second statement correctly converts a temperature variation from the Fahrenheit scale to the Celsius scale. Though, the last statement applies the same formula to convert a temperature point from the Celsius scale to the Fahrenheit scale. It is wrong, because it does not take into account the fact that these two scales have different origins.

This is avoided by using this code:

```rust
let fahrenheit_temperature_variation = Measure::<Fahrenheit, f64>::new(3.);
let celsius_temperature_variation = fahrenheit_temperature_variation
    .convert::<Celsius>();
let fahrenheit_temperature_point = MeasurePoint::<Fahrenheit, f64>::new(3.);
let celsius_temperature_point = fahrenheit_temperature_point.convert::<Celsius>();
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

The third statement computes a value in miles per hours, and it wrongly assigns that value to a variable meant to keep a value in metres per second.

This is avoided by using this code:

```rust
let distance = Measure::<Mile, f64>::new(8.);
let time_elapsed = Measure::<Hour, f64>::new(2.);
let average_speed1: Measure<MilePerHour, f64>
    = distance / time_elapsed; // Allowed.
// or
let average_speed2 = distance / time_elapsed; // Allowed.
let average_speed3: Measure<MetrePerSecond, f64>
    = distance / time_elapsed; // Compilation error.
```

The compilation of the third statement checks that the value assigned has the same derived unit of measurement of the variable that receives that value.

The compilation of the fourth statements infers such unit of measurement.

The compilation of the last statement generates a type-mismatch compilation error.

## Plane (2D) And space (3D) measures

Some physical or geometrical properties are properly described as multidimensional. For example, a displacement in a plane has two dimensions (X and Y), and a displacement in space has three dimensions (X, Y, and Z). They are called “vector measures”. Plane vector measures (a.k.a. 2D measures) have two components, and space vector measures (a.k.a. 3D measures) have three components.

For vector measures, usually there is no reason to use a numeric type and a unit of measurement for one component, and a different numeric type or a different unit of measurement for another component. So, the Rs-Measures crate allows to encapsulate all the components of a vector measure into a single object.

For example, instead of writing this code:

```rust
let force = (Measure::<Newton, f64>::new(3.), Measure::<Newton, f64>::new(4.));
let displacement = (Measure2d::<Metre, f64>::new(7.), Measure2d::<Metre, f64>::new(1.));
```

You can and should write this code:

```rust
let force = Measure2d::<Newton, f64>::new(3., 4.);
let displacement = Measure2d::<Metre, f64>::new(7., 1.);
```

Similarly, it is possible to create 3D measures in space:

```rust
let force = Measure3d::<Newton, f64>::new(3., 4., 5.);
let displacement = Measure3d::<Metre, f64>::new(7., 1., -6.);
```

Points in a plane or in space can be represented by single objects:

```rust
let position_in_plane = MeasurePoint2d::<Metre, f64>::new(3., 4.);
let position_in_space = MeasurePoint3d::<Metre, f64>::new(3., 4., 5.);
```

Dot products and cross products between measures have the right units of measurement:

```rust
let force = Measure2d::<Newton, f64>::new(3., 4.);
let displacement = Measure2d::<Metre, f64>::new(7., 1.);
let work: Measure<Joule, f64> = force * displacement;
let torque: Measure<NewtonMetre, f64> = force.cross_product(displacement);

let force = Measure3d::<Newton, f64>::new(3., 4., 5.);
let displacement = Measure3d::<Metre, f64>::new(7., 1., -6.);
let work: Measure<Joule, f64> = force * displacement;
let torque: Measure3d<NewtonMetre, f64> = force.cross_product(displacement);
```

In the third statement, a 2D force, expressed in newtons, is multiplied by a 2D displacement, expressed in metres. Such multiplication, which uses the "`*`" operator, is the dot product between the two 2D vectors. So, the result is an energy (or work), expressed in joules.

In the fourth statement, the cross product is computed between that force and that displacement, resulting in a torque, expressed in newton-metres.

In the last four statements, similar computations are performed using 3D measures.

Notice that the 2D torque is a scalar (a `Measure`), while the 3D torque is a 3D vector (a `Measure3d`).

## Angles and directions

Angles deserve a special treatment. Consider the following Rust code:

```rust
let mut angular_position = 300.; // In degrees.
let rotation = 400.; // In degrees.
angular_position += rotation;
assert_eq!(angular_position, 700.);
```

The third statement increments an angular position by a rotation. The resulting position is 700 degrees, that is more than one whole turn. It may be what we needed, but it may be we needed instead a position in the circumference, i.e. a directional angle.

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

Though, these formulas are error-prone, and depend on the angular unit of measurement. These error-prone expressions are avoided by using this code:

```rust
let mut angular_position = MeasurePoint::<Degree, f64>::new(300.);
let rotation = Measure::<Degree, f64>::new(400.);

// It increments an unconstrained angular position,
// that is a `MeasurePoint`, by adding a rotation to it.
angular_position += rotation;
assert_eq!(700., angular_position.value);

// The method `to_unsigned_direction` converts the unconstrained angular position
// to a direction type, whose values are constrained to be in the range 0 to 360 degrees.
// Such a type is named `UnsignedDirection`, because it may have only non-negative values.
let unsigned_direction: UnsignedDirection::<Degree, f64> = angular_position.to_unsigned_direction();
assert_eq!(unsigned_direction.value, 340.);

// The method `to_signed_direction` converts the unconstrained angular position
// to a direction type, whose values are constrained to be in the range -180 to 180 degrees.
// Such a type is named `SignedDirection`, because it may have negative or positive values.
let signed_direction: SignedDirection::<Degree, f64> = angular_position.to_signed_direction();
assert_eq!(signed_direction.value, -20.);
```

## Vector and affine transformations

When using vector measures in a plane or in the 3D space, it is quite typical to need to perform vector operations. There are good linear algebra crates, like the `nalgebra` crate, which could be used for this. Though, when using them, it is not possible to keep the measurement unit information of components. Consider the following Rust program, which uses the `nalgebra` crate:

```rust
extern crate nalgebra;
use nalgebra::{Rotation2, Vector2};
fn main() {
    let displacement = Vector2::new(4., 5.);
    let rotation = Rotation2::new(0.01);
    let rotated_displacement = rotation * displacement;
}
```

First, it stores in `displacement` a 2D measure, with no units of measurement. Then, it stores in `rotation` a 2D linear transformation. Then, such rotation is applied to the displacement, using a matrix multiplication, obtaining a transformed 2D measure. Such resulting measure is stored in the variable `rotated_displacement`. Of course, such a variable is meant to have the same unit of measurement of the first variable, but this is not specified by the used types.

Let’s try to attach units to measures, with this code:

```rust
let displacement = Vector2::new(
    Measure::<Metre, f64>::new(4.),
    Measure::<Metre, f64>::new(5.),
);
let rotation = Rotation2::new(Measure::<Radian, f64>::new(0.01));
let rotated_displacement = rotation * displacement;
```

The first statement is allowed, but the second one is not allowed by `nalgebra`.
We can try to use a naked number for the rotation:

```rust
let displacement = Vector2::new(
    Measure::<Metre, f64>::new(4.),
    Measure::<Metre, f64>::new(5.),
);
let rotation = Rotation2::new(0.01);
let rotated_displacement = rotation * displacement;
```

This causes an error on the last statement, as `nalgebra` is not able to apply a linear transformation to a vector of `Measure` values. And even if it were able, probably the resulting value, which is assigned to `rotated_displacement`, wouldn’t have the proper unit of measurement.

Instead, using the Rs-Measures crate, it is possible to compile the following code:

```rust
let displacement = Measure2d::<Metre, f64>::new(4., 5.);
let rotation = LinearMap2d::rotation(
    Measure::<Radian, f64>::new(0.01));
let rotated_displacement = rotation.apply_to(displacement);
```

The first statement creates a 2D measure in metres.

The second statement creates a 2D linear transformation that corresponds to a rotation of 0.01 radians.

The third statement applies that transformation to the measure, returning a value having the same type of the original displacement.

The above code uses the type `LinearMap2d` to manipulate objects of type `Measure2d`. To manipulate objects of type `MeasurePoint2d`, the type `AffineMap2d` should be used instead.

And when working in three dimensions, the types `LinearMap3d` and `AffineMap3d` should be used.
