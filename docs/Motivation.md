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

This is avoided by this code:

```rust
let mass1 = Measure::<f64, KiloGram>::new(1.2);
let mut mass2: Measure<f64, Pound>;
mass2 = mass1; // Compilation error.
fn set_mass(val: Measure<f64, Pound>) { /* ... */ }
set_mass(mass1); // Compilation error.
```

The third and the last statements cause compilation errors for type mismatch, as the values are encapsulated in objects whose types is characterized by a unit of measurement, and any value must have the same type of the variable to which it is assigned.

## Bad unit conversions

Consider the following Rust code:

```rust
let mass1 = 1.2; // kilograms
let mut mass2: f64; // pounds
mass2 = mass1 * 0.45359237; // Wrong conversion operation; it should be a division.
mass2 = mass1 / 453.59237; // Wrong conversion ratio.
```

The third statement tries to convert a value from kilograms to pounds, but it uses a multiplication instead of a division. Also the fourth statement tries to do the same unit conversion, but uses a wrong divisor.

This is avoided by the following code code:

```rust
let mass1 = Measure::<f64, KiloGram>::new(1.2);
let mut mass2: Measure<f64, Pound>;
mass2 = mass1.convert();
// or
let mass3 = mass1.convert::<Pound>();
```

The third statement simply invokes the `convert` method. The correct formula is inferred using the type of the destination variable. The last statement must specify the destination unit of measurement, as the destination variable does not specify a type.

## User-specified properties and units of measurement

Some existing libraries, for example the UOM crate, solve the previous issues by defining a set of standard units, typically taken from the SI international standard, and, at run-time, they convert all input values from the user-chosen units to such standard units, and they convert all output values from such standard units to user-chosen units.

This technique is uncomfortable, for the following reasons:

1. It forces programmers to use units that are different from the ones actually seen by users. For example, a developer wants to use everywhere only pounds. But if he tries to explore with a debugger the contents of a length object, he finds a value in kilograms.
2. It may introduce rounding errors.
3. It introduces a small conversion overhead for every I/O operation. Using Rs-Measures, if the input is always in pounds, the number is always stored in pounds, and the output is always in pounds, no conversion is needed.

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

As you can see, even the value in grams and the value in pounds are different from the original values.

The corresponding program using Rs-Measures is this:

```rust
using measures::{Measure, units::{Gram, KiloGram, Pound}};
fn main() {
    let mass1 = Measure::<f32, Gram>::new(8030.);
    let mass2 = Measure::<f32, Pound>::new(27.);
    println!("{}, {}, {}.", mass1, mass1.convert::<KiloGram>(), mass2);
}
```

It will print `8030 g, 8.030001 kg, 27 lb.`.

Only the value in kilograms is not exact, because its value is stored internally in grams, and the conversion to kilograms introduced a rounding error.

In addition, trying to define in a library all possible properties and units has the following drawbacks:

1. It makes the library unnecessarily heavy to compile, as the resulting types are very complex.
2. Because of the complexity of the resulting types, the compiler error messages are quite verbose.
3. If the library is not extensible, it may be incomplete, as there are some little used units that some application may require and that have not been included in the library. If it is extensible, it is usually somewhat difficult to do.
4. Forces programmers to use a specific natural language to name the properties and their units of measurements, instead of the natural language of the programmer. For example, some people prefer to write "metre" and other people prefer "meter".

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

Instead, if, using the Rs-Measures crate, we write the statement `let _: Measure<f32, KiloGram> = Measure::<f32, Gram>::new(0.);`, that tries to assign a mass in grams to a mass in kilograms, the generated error message is:

```text
error[E0308]: mismatched types
  --> src/main.rs:84:37
   |
50 |     let _: Measure<f32, KiloGram> = Measure::<f32, Gram>::new(0.);
   |            ----------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `KiloGram`, found struct `Gram`
   |            |
   |            expected due to this
   |
   = note: expected struct `Measure<_, KiloGram>`
              found struct `Measure<_, Gram>`
```

From it, it is clear that it was used a Gram where a KiloGram was expected.

Instead, if we write the statement `let _ = Measure::<f32, Gram>::new(0.).convert::<Second>();`, that tries to convert a measure in grams to a measure in seconds, the generated error message is:

```text
error[E0271]: type mismatch resolving `<Second as rs_measures::traits::MeasurementUnit>::Property == Mass`
  --> src/main.rs:85:43
   |
85 |     let _ = Measure::<f32, Gram>::new(0.).convert::<Second>();
   |                                           ^^^^^^^ expected struct `Time`, found struct `Mass`
```

From it, it is clear that it was used a Mass where a Time was expected.

Therefore, it is better to allow the application programmer to define the units of measurement he/she is going to use in his/her software application.

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

This is avoided by this code:

```rust
let position1 = MeasurePoint::<f64, Metre>::new(1.2);
let position2 = MeasurePoint::<f64, Metre>::new(2.3);
let displacement1 = Measure::<f64, Metre>::new(0.3);
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

The second statement correctly converts a temperature variation from the Celsius scale to the Fahrenheit scale. Though, the last statement applies the same formula to convert a temperature point from the Celsius scale to the Fahrenheit scale. It is wrong because it does not take into account the fact the these two scales have different origins.

This is avoided by this code:

```rust
let fahrenheit_temperature_variation = Measure::<f64, Fahrenheit>::new(3.);
let celsius_temperature_variation = fahrenheit_temperature_variation
    .convert::<Celsius>();
let fahrenheit_temperature_point = MeasurePoint::<f64, Fahrenheit>::new(3.);
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

The third statement computes a value in miles per hours and wrongly it assigns that value to a variable meant to keep a value in metres per second.

This is avoided by this code:

```rust
let distance = Measure::<f64, Mile>::new(8.);
let time_elapsed = Measure::<f64, Hour>::new(2.);
let average_speed1: Measure<f64, MilePerHour>
    = distance / time_elapsed; // Allowed.
// or
let average_speed2 = distance / time_elapsed; // Allowed.
let average_speed3: Measure<f64, MetrePerSecond>
    = distance / time_elapsed; // Compilation error.
```

The compilation of the third statement checks the value assigned has the same derived unit of measurement of the variable that receives that value.
The compilation of the fourth statements infers such unit of measurement.
The compilation of the last statement generates a type mismatch compilation error.

## Plane (2D) And space (3D) measures

Some physical or geometrical properties are properly described as multidimensional. For example, a displacement in a plane has two dimensions (X and Y), and a displacement in space has three dimensions (X, Y, and Z). They are called “vector measures”. Plane vector measures (a.k.a. 2D measures) have two components, and space vector measures (a.k.a. 3D measures) have three components.

For vector measures, usually there is no reason to use a numeric type and a unit of measurement for one component, and a different numeric type or a different unit of measurement for another component. So, the Rs-Measures crate allows to encapsulate all the components of a vector measure into a single object.
For example, instead of writing this code:

```rust
let force = (Measure::<f64, Newton>::new(3.), Measure::<f64, Newton>::new(4.));
let displacement = (Measure2d::<f64, Metre>::new(7.), Measure2d::<f64, Metre>::new(1.));
```

You can and should write this code:

```rust
let force = Measure2d::<f64, Newton>::new(3., 4.);
let displacement = Measure2d::<f64, Metre>::new(7., 1.);
```

Similarly, it is possible to create 3D measures in space:

```rust
let force = Measure3d::<f64, Newton>::new(3., 4., 5.);
let displacement = Measure3d::<f64, Metre>::new(7., 1., -6.);
```

Points in a plane or in space can be represented by single objects:

```rust
let position_in_plane = MeasurePoint2d::<f64, Metre>::new(3., 4.);
let position_in_space = MeasurePoint3d::<f64, Metre>::new(3., 4., 5.);
```

Dot products and cross products between measures have the right units of measurement:

```rust
let force = Measure2d::<f64, Newton>::new(3., 4.);
let displacement = Measure2d::<f64, Metre>::new(7., 1.);
let work: Measure<f64, Joule> = force * displacement;
let torque: Measure<f64, NewtonMetre> = force.cross_product(displacement);

let force = Measure3d::<f64, Newton>::new(3., 4., 5.);
let displacement = Measure3d::<f64, Metre>::new(7., 1., -6.);
let work: Measure<f64, Joule> = force * displacement;
let torque: Measure3d<f64, NewtonMetre> = force.cross_product(displacement);
```

In the third statement, a 2D force expressed in newtons is multiplied by a 2D displacement expressed in metres, resulting in an energy (or work), expressed in joules. Such multiplication is the dot product between the 2D vectors.

In the fourth statement, the cross product is computed between that force and that displacement, resulting in a torque, expressed in newton-metres.

In the last four statements, similar computations are performed using 3D measures.

Notice that the 2D torque is a scalar (a `Measure`), while the 3D torque is a 3D vector (a `Measure3d`).

## Angles and directions

Angles deserve a special treatment. Consider the following Rust code:

```rust
let mut angular_position = 300.; // In degrees.
let rotation = 400.; // In degrees.
angular_position += rotation;
assert_eq!(700., angular_position);

let direction360 = angular_position % 360.;
assert_eq!(340., direction360);

let direction180 = (angular_position + 180.) % 360. - 180.;
assert_eq!(-20., direction180);
```

The third statement increments an angular position by a rotation. The resulting position is 700 degrees, that is more than one whole turn. It may be what we needed, but we may need instead a position in the circumference, i.e. a directional angle.

There are two widely used conventions to specify a direction: using an angle from 0 to 360 degrees, or an angle from -180 to +180 degrees. The former case is computed by the fifth statement, and the latter case is computed by the seventh statement.

Though, these formulas are error-prone, and depend on the angular unit of measurement. These error-prone expressions are avoided by this code:

```rust
let mut angular_position = MeasurePoint::<f64, Degree>::new(300.);
let rotation = Measure::<f64, Degree>::new(400.);
angular_position += rotation;
assert_eq!(700., angular_position.value);

let direction360_1 = angular_position.to_unsigned_direction();
assert_eq!(340., direction360_1.value);
let direction360_2 = UnsignedDirection::<f64, Degree>::new(300.) + rotation;
assert_eq!(340., direction360_2.value);

let direction180_1 = angular_position.to_signed_direction();
assert_eq!(-20., direction180_1.value);
let direction180_2 = SignedDirection::<f64, Degree>::new(300.) + rotation;
assert_eq!(-20., direction180_2.value);
```

The third statement increments an unconstrained angular position, that is a `MeasurePoint`, by adding a rotation to it.

Then, the `UnsignedDirection` type is used. It encapsulates a value constrained in the range 0 to 360 degrees.

Such type is used in two statements. In the former, the unconstrained angular position is converted to an `UnsignedDirection`. In the latter, an `UnsignedDirection` object is constructed from a number, and then it is incremented by the rotation. In both cases, the inner value is 340.

Then, the `SignedDirection` type is used. It encapsulate a value constrained in the range -180 to +180 degrees. Such type is used in two statements, similar to those seen before. In both cases, the inner value is -20.

## Vector and affine transformations

Using external linear algebra crates, it is not possible to keep the measurement unit information of numbers. Consider the following Rust program, that uses the `nalgebra` crate:

```rust
extern crate nalgebra;
use nalgebra::{Rotation2, Vector2};
fn main() {
    let displacement = Vector2::new(4., 5.);
    let rotation = Rotation2::new(0.01);
    let rotated_displacement = rotation * displacement;
}
```

It stores in `displacement` a 2D measure, with no unit of measurement. Then it stores in `rotation` a 2D linear transformation. Then such rotation is applied to the displacement, using a matrix multiplication, obtaining a transformed 2D measure. Such resulting measure is stored in the `rotated_displacement` variable. Such variable is meant to have the same unit of measurement of the first variable.

Let’s try to attach units to measures, with this code:

```rust
let displacement = Vector2::new(
    Measure::<f64, Metre>::new(4.),
    Measure::<f64, Metre>::new(5.),
);
let rotation = Rotation2::new(Measure::<f64, Radian>::new(0.01));
let rotated_displacement = rotation * displacement;
```

The first statement is allowed, but the second one is not allowed.
We can try to use a naked number for the rotation:

```rust
let displacement = Vector2::new(
    Measure::<f64, Metre>::new(4.),
    Measure::<f64, Metre>::new(5.),
);
let rotation = Rotation2::new(0.01);
let rotated_displacement = rotation * displacement;
```

This causes an error on the last statement, as `nalgebra` is not able to apply a linear transformation to a vector of `Measure` values. And even if it were able, probably the resulting value, that is assigned to `rotated_displacement`, wouldn’t have the proper unit of measurement.

Instead, using the Rs-Measures crate, it is possible to compile the following code:

```rust
let displacement = Measure2d::<f64, Metre>::new(4., 5.);
let rotation = LinearMap2d::rotation(
    Measure::<f64, Radian>::new(0.01));
let rotated_displacement = rotation.apply_to(displacement);
```

The first statement creates a 2D measure in metres.

The second statement creates a 2D linear transformation that corresponds to a rotation of 0.01 radians.

The third statement applies that transformation to the measure, returning a value having the same type of the original displacement.

To manipulate objects of type `MeasurePoint2d`, the type `AffineMap2d` should be used.

And working in three dimensions, the types `LinearMap3d` and `AffineMap3d` can be used.
