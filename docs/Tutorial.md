# Tutorial for the library `rs-measures`

This documents explains how to use in Rust software development projects the library `rs-measures`, composed by the two crates `rs-measures` and `units-relation`. To know which are the advantages of using this library, with respect to using raw numbers or other libraries, read the document [Motivation](Motivation.md).

## Creating a tutorial project

Create a rust project using these commands:
```
cargo new measure-tut
cd measure-tut
cargo add rs-measures
cargo add units-relation
```

Open the Web page https://github.com/carlomilanesi/rs-measures/blob/main/units-relation/examples/units/mod.rs.
Copy all its contents in the clipboard.
Create a file named `src/units.rs`, and paste the copied contents into it. Save it.

## Creating a measure variable

Edit the file `src/main.rs`, so that it contains this:
```rust
mod units;
use units::*;
fn main() {
    let distance = Measure::<KiloMetre>::new(100.);
    println!("The distance is {distance}.");
}
```

Run your project. It should print: `The distance is 100 km`. Notice that the characters "` km`" are printed automatically, because of the type of the variable.

The variable named `distance` encapsulates an `f64` number, whose value is `100`. The unit of measurement is not stored in memory. It is just in the type of the variable.

You can access the value of a measure by adding these two statements to your function `main`:
```rust
    let distance_value: f64 = distance.value;
    println!("The distance is {distance_value}.");
```

It will print:
```
The distance is 100 km.
The distance is 100.
```

The second printed line has no unit, because it is a raw number.

This code is valid with `distance_value` having type `f64`, because this is default value type of the `Measure` generic type.
The statement `let distance_value: f32 = distance.value;` would have been illegal.
Though, you can use also `f32` as value type, if you specify it explicitly, like in the following statements:
```rust
    let distance: f32 = Measure::<KiloMetre, f32>::new(100.).value;
```

Currently the library supports only the value types `f32` and `f64`.

In the future, just put the example code as the body of the function `main`.

## Numeric conversions

You can convert the value type, like in these statements:
```rust
    let distance = Measure::<Metre, f32>::new(100.).lossless_into::<f64>();
    let distance: Measure<Metre, f32> = distance.lossy_into::<f32>();
    let distance: Measure<Metre, f64> = distance.lossy_into::<f64>();
```

There are two numeric conversion methods:
* `lossless_into`: It guarantees that no precision is lost in the conversion. In particular, it can be used only to convert from `f32` to `f64`, in addition to the trivial conversions from `f32` to `f32` and from `f64` to `f64`.
* `lossy_into`: It does not guarantee that any precision is lost in the conversion. So, it may be used for any kind of numeric conversion, including from `f64` to `f32`.

## Operations on measures

Whe have already seen that the type `Measure` can be printed.
This means it implements the `Display` trait, and it has the `to_string()` method.
Printing a measure means printing its value followed by a suffix depending on its units of measurement.
For the unit `KiloMeter`, the suffix is ` km`, with a leading space.

Here are the other operations that can be performed on one or two objects of type `Measure` having the same unit and the same value type.
```rust
    let length = Measure::<Metre>::new(7.);
    assert_eq!(length.squared_norm(), 49.);
    // This is equivalent to this:
    assert_eq!(length.value * length.value, 49.);

    assert_eq!(length.normalized().value, 1.);
    // This is equivalent to this:
    assert_eq!(length.value.signum(), 1.);
```

The standard-library method `signum` returns `1.` for a positive argument, including a positive zero, and `-1.` otherwise.
```rust
    let mut length1 = Measure::<Metre>::new(7.);
    let length2 = Measure::<Metre>::new(5.);
    assert_eq!((-length1).value, -7.);
    assert_eq!((length1 + length2).value, 12.);
    assert_eq!((length1 - length2).value, 2.);
    length1 += length2;
    assert_eq!(length1.value, 12.);
    length1 -= length2;
    assert_eq!(length1.value, 7.);
    assert_eq!((length1 * 3.).value, 21.);
    assert_eq!((3. * length1).value, 21.);
    length1 *= 3.;
    assert_eq!(length1.value, 21.);
    assert_eq!((length1 / 3.).value, 7.);
    length1 /= 3.;
    assert_eq!(length1.value, 7.);
    assert_eq!(length1 / length2, 1.4); // When to measures of the same unit are divided, a number is obtained.
    assert!(length1 == length1);
    assert!(length1 > length2);
```

## Conversions between units

You can convert between units of measurement of the same physical or geometrical property using the following code:
```rust
    let distance1 = Measure::<KiloMetre>::new(100.);
    let distance2 = distance1.convert::<Mile>();
    let time1 = Measure::<Hour>::new(2.);
    let time2 = time1.convert::<Minute>();
    println!("The distance {distance1} and {distance2} are equivalent.");
    println!("The times {time1} and {time2} are equivalent.");
```

It will print:
```
The distance 100 km and 62.15040397762586 mi are equivalent.
The times 2 h and 120 min are equivalent.
```

You cannot convert between units of measurement representing different physical or geometrical property:
```rust
    let _ = Measure::<KiloMetre>::new(100.).convert::<Hour>(); // ILLEGAL
```

## Other operations available for measures

Three other features are available for measures:
* The member `value`, which is just the inner value of the measure. Its type is `f64` or `f32`.
* The method `squared_norm()`, which returns `value` multiplied by itself. It has the same type of `value`.
* The method `normalized()`, it returns a measure with the same sign, but absolute value `1`. Therefore, it is `1` for positive values, and `-1` for negative values. Regarding the zero value, actually floating-point numbers can have a positive zero value or a negative zero value; in the first case, `normalized()` returns `1`, and in the second case, it returns `-1`.

## Absolute values

For some kinds of measures, does not make sense to add two values of the same type, or to multiply a value by a number. For example, does not make sense to sum the temperature at which water freezes to the temperature at which water boils. To show another example, it does not make sense to multiply by 2 the date of today.

This happens because they are absolute measures, not relative measures, like the ones shown in the previous sections. Mathematically speaking, such measures are *points* of an *affine space*; they are not vectors. To avoid using points instead of vectors or vice versa, instead of using the generic type `Measure`, the generic type `MeasurePoint` should be used, like in the following code:
```
let point1 = MeasurePoint<Celsius>:new(10);
let variation = Measure<Celsius>:new(2);
let point2 = point1 + variation;
println!("When increasing temperature {point1} by {variation}, we reach temperature {point2}.");
```
It will print:
```
When increasing temperature at 10°C by 2°C, we reach temperature at 12°C.
```

When a measure point is printed, the three characters "`at `" are prefixed to it.

With absolute values, all the following operations are forbidden:
```
let point1 = MeasurePoint<Celsius>::new(10);
let point2 = MeasurePoint<Celsius>::new(2);
let _ = point1 + point2; // ILLEGAL
let _ = point1 * 3.; // ILLEGAL
```

# Define your-own units of measurement

The use of the module `units.rs` and `relations.rs` is useful for learning and for experimenting. Though, it is not recommended for production use, for the following reasons:
* Such large files increase your code base.
* Such large files increase compilation time.
* Such files use words or output prefix you don't like.
* Such files have names which conflicts with some names used in your project.
* Such files miss some properties or units or relations you need.

Therefore, the suggested procedure for production code is the following one:
* Create two empty files for your project, named `units.rs` and `relations.rs`.
* Search the provided files with the same names for the properties, the units, and the relations you need, or most similar to what you need. Copy and paste them into your own files.
* Edit you files `units.rs` and `relations.rs` according your needs.

To create a property means simply to define an empty `struct` with the desired name. For example: `pub struct Information;`.

To create a unit for a property you first should decide, in case there will several units for that property, which is the *main* unit. For example, if we want to create the units `Bit` and `Byte` for property `Information`, we can decide that `Bit` is the main unit for you project.

To create the main unit for a property, define a `struct` like this one:
```rust
pub struct Bit;
impl MeasurementUnit for Bit {
    type Property = Information;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b";
}
```

In the first and second lines, there must be the named of the unit (in this example, it is `Bit`).

In the third line, the property must be specified (in this example, it is `Information`).

In the fourth line, the ratio to the main unit must be specified (in this example, it is `1.`, because this is the main unit).

In the fifth line, the offset of the origin with respect to the origin of the main unit must be specified (in this example, it is `0.`, because this is the main unit). Almost all units have a zero offset. The exceptions are the temperature scales and the calendars. This property is not to be used to define different geometrical coordinate systems, because OFFSET must be a compile-time constant, and, in case of plane or space measures, it must be the same for X, Y, and Z.

In the sixth line, the SUFFIX to use when the measure is converted to a string. Consider that, to have that suffix separated from the number, you must begin it with a space.

Then, if the unit has property `Angle`, a declaration like the following one must be added:
```rust
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}
```

In the first line, the name of the unit must be specified (in this example, it is `Degree`).

In the second line, the number of times this unit is contained in a full turn must be specified (in this example it is `360.`, because in a turn there are 360 degrees).

Then, consider that for some property, like length, force, or electric field strength, it makes sense to have two-dimensional measures in a plane or three-dimensional measures in the space. For others, like time, mass or temperature, it does not make sense. The properties of the first kind are named *vector properties*, and the properties of the second kind are named *scalar properties*.

To allow multidimensional measures only for the properties for which it makes sense, whenever you define a unit for a vector property, you should add a declaration tike the following one:
```rust
impl VectorMeasurementUnit for Inch {}
```

This statement specifies that `Inch` is a unit of a vector property, and so you are allowed to define 2D or 3D measures using this unit.

After having defined all the needed units of measurement, in case you have units of different properties, and you want to perform multiplications or divisions between measures, you must define relations among units.

You can search the edit the file `relations.rs`, so that it contains statements like this one:
```rust
define_units_relation! {BitPerSecond == Bit / Second}
```

It declares that if you divide a measure of one bit by a measure of one second, you get a measure of one bit per second.

Actually, it is equivalent to each of these ones:
```rust
define_units_relation! {Bit == Second * BitPerSecond}
define_units_relation! {Bit == BitPerSecond * Second}
define_units_relation! {Bit / Second == BitPerSecond}
define_units_relation! {Second * BitPerSecond == Bit}
define_units_relation! {BitPerSecond * Second == Bit}
```

Choose just one these 6 possible forms.

It is also possible to have the same unit in a multiplication:
```rust
define_units_relation! {SquareMetre == Metre * Metre}
```

It declares that if you multiply a measure of one metre by a measure of one metre, you get a measure of one square metre. In such a case, it is also allowed to call the method `squared` on a measure in metres, to obtain a measure in square metres, or to call the method `sqrt` on a measure in square metres, to obtain a measure in metres.

For vector properties, you can write these statements:
```rust
define_units_relation! {MetrePerSecond:2 == Metre:2 / Second}
define_units_relation! {MetrePerSecond:3 == Metre:3 / Second}
```

The first statement declares that if you divide a bi-dimensional measure of one metre by a measure of one second, you get a bi-dimensional measure of one metre per second.
The second statement is similar, but for three-dimensional measures.

Notice that if you multiply a bi-dimensional measure by another bi-dimensional measure, or a tri-dimensional measure by another tri-dimensional measure, it means to compute the dot product between them.

Another possible case is this one:
```rust
define_units_relation! {Hertz == 1 / Second}
```

It declares that if you divide the number 1 by a measure of one second, you get a measure of one hertz.

Finally, it is possible to write these statements:
```rust
define_units_relation! {Newton:2 X Metre:2 == NewtonMetre}
define_units_relation! {Newton:3 X Metre:3 == NewtonMetre:3}
```

Notice that, instead of a symbol of multiplication ("`*`") or division ("`/`"), here there is a letter "`X`", representing the cross product.

The first statement declares that if you compute the cross product between a bi-dimensional measure of one newton by a bi-dimensional measure of one metre, you get a (uni-dimensional) measure of one newton-metre.
The second statement is similar, but for three-dimensional measures, resulting in a three-dimensional measure of one newton-metre.

These definitions allow to write statements like this one:
```rust
let m: Measure3d<NewtonMetre> = Measure3d::<Newton>::new(1., 2., 3.)
    .cross_product(Measure3d::<Metre>::new(4., 5., 6.));
```
