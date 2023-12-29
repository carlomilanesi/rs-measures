# Rs-Measures Tutorial

This documents explains how to use the rust crate Rs-Measures. To know which are the advantages of using this crate, with respect to using raw numbers or other crates, read the document [Motivation](Motivation.md).

## Using already-defined units of measurement

Create a rust project with these commands:
```
cargo new tut1
cd tut1
cargo add rs-measures
```

Download the file https://github./carlomilanesi/rs-measures/rs-measures/src/bin/common/units.rs, and move it to the folder `src/`.


Edit the file `src/main.rs`, so that it contains this:
```
mod units;
use rs_measures::angle::Radian;
use units::*;

fn main() {
    let distance = Measure::<KiloMetre>::new(100.);
    println!("The distance is {distance}.");
}
```

Run your project. It should print: `The distance is 100 km`. Notice that the characters "` km`" are printed automatically because of the type of the value.

The variable named `distance` encapsulate an `f64` number, whose value is `100`. The type of that variable is `Measure<KiloMetrePerHour, f64>`, which represents a *relative measure* (i.e. a *variation*).

Mathematically speaking, it is a *vector* of a *vector space*, meaning that you can add it to another object of the same type, or you can multiply it by a number. Here are some examples of this. Replace the body of the function `main` with the following code:
```rust
    let distance1 = Measure::<KiloMetre>::new(100.);
    let distance2 = Measure::<KiloMetre>::new(40.);
    println!("The sum of {distance1} and {distance2} is {distance}.", distance = distance1 + distance2);
    println!("Three times {distance1} is {distance}.", distance = distance1 * 3.);
```
It should print:
```
The sum of 100 km and 40 km is 140 km.
Three times 100 km is 300 km.
```

## Conversions between units

You can convert between units of measurement of the same physical or mathematical property using the following code:

```rust
    let distance1 = Measure::<KiloMetre>::new(100.);
    let distance2 = distance1.convert::<Mile>();
    let time1 = Measure::<Hour>::new(2.);
    let time2 = distance1.convert::<Minute>();
    println!("The distance {distance1} and {distance2} are equivalent.");
    println!("The times {time1} and {time2} are equivalent.");
```

You cannot convert between units of measurement representing different physical or mathematical property:
```rust
    let distance = Measure::<KiloMetre>::new(100.).convert::<Hour>(); // ILLEGAL
```

## Inner numeric type

The expression `Measure::<KiloMetre>::new(100.)` is a shorthand of the expression `Measure::<KiloMetre, f64>::new(100.)`, because the default inner type is `f64`.

To have `f32` as inner type, you must be explicit: `Measure::<KiloMetre, f32>::new(100.)`.

In Rust, you cannot assign a value of type `f32` to an object of type `f64` nor vice versa. The same happens with measure objects.

Though, it is possible to convert between measure type differing by the inner type, with the following code:
```
let d1 = Measure::<KiloMetre, f32>::new(100.).lossless_into::<f64>();
let d2 = Measure::<KiloMetre, f32>::new(100.).lossy_into::<f64>();
```

The method `lossy_into` is always allowed, even if the destination type has less precision than the source type, and so some information could be lost by this conversion.

Instead, the method `lossless_into` is allowed only when the destination type has the same or better precision than the source type, and so it is guaranteed that no information will be lost by this conversion.

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
It should print:
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
