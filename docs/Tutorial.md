# Tutorial for the library `rs-measures`

This documents explains how to use in Rust software development projects the library `rs-measures`, composed by the two crates `rs-measures` and `units-relation`. To know which are the advantages of using this library, with respect to using other libraries or naked numbers, read the document [Motivation](Motivation.md).

## Creating a tutorial project

Create a rust project using these commands:
```
cargo new measures-tut
cd measures-tut
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

Run your project. It should print: `The distance is 100 km`.
Notice that the characters "` km`" are printed automatically, because of the type of the variable.

The variable named `distance` encapsulates an `f64` number, whose value is `100`.
The unit of measurement is not stored in memory.
It is just in the type of the variable.

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

The second printed line has no unit, because it is a naked number.

This code is valid with `distance_value` having type `f64`, because `f64` is the default value type of the generic type `Measure`.
The statement `let distance_value: f32 = distance.value;` would have been illegal.
Though, you can use also `f32` as value type, if you specify it explicitly, like in the following statement:
```rust
    let distance: f32 = Measure::<KiloMetre, f32>::new(100.).value;
```

Currently the library supports only the value types `f32` and `f64`.

## Numeric conversions

From here on, to run the example code, just put it as the body of the function `main`.

You can convert the value type of a measure, like in these statements:
```rust
    let distance = Measure::<Metre, f32>::new(100.).lossless_into::<f64>(); // From f64 to f32
    let distance: Measure<Metre, f32> = distance.lossy_into::<f32>(); // From f32 to f32
    let distance: Measure<Metre, f64> = distance.lossy_into::<f64>(); // From f64 to f64
```

There are two numeric conversion methods:
* `lossless_into`: It guarantees that no precision is lost in the conversion. So, it can be used only to convert from `f32` to `f64`, in addition to the trivial conversions from `f32` to `f32` and from `f64` to `f64`.
* `lossy_into`: It does not guarantee that any precision is lost in the conversion. So, it may be used for any kind of numeric conversions, including from `f64` to `f32`.

## Operations on measures

Whe have already seen that the type `Measure` can be printed.
This means it implements the trait `Display`, and it has the method `to_string()`.
Printing a measure means printing its value followed by a suffix depending on its units of measurement.
For the unit `KiloMeter`, the suffix is `" km"`, with a leading space.

Measures implement also the trait `Debug`, having the same behavior of the the trait `Display`.
So, you can write:
```rust
    #[allow(dead_code)]
    #[derive(Debug)]
    struct TimeElapsed {
        hours: Measure<Hour>,
        minutes: Measure<Minute>,
    }
    let time_elapsed = TimeElapsed {
        hours: Measure::<Hour>::new(3.),
        minutes: Measure::<Minute>::new(17.),
    };
    println!("{time_elapsed:?}");
```

It will print: `TimeElapsed { hours: 3 h, minutes: 17 min }`.

Here are the other operations that can be performed on one or two objects of type `Measure`, provided they have the same unit and the same value type.
```rust
    let length = Measure::<Metre>::new(-7.);
    assert_eq!(length.squared_norm(), 49.);
    // It is equivalent to this:
    assert_eq!(length.value * length.value, 49.);

    assert_eq!(length.normalized().value, -1.);
    // It is equivalent to this:
    assert_eq!(length.value.signum(), -1.);
```

The standard-library method `signum` returns `1.` for a positive argument, including a positive zero, and `-1.` otherwise, including a negative zero.

Here are other arithmetic operations:
```rust
    let mut length1 = Measure::<Metre>::new(7.);
    let length2 = Measure::<Metre>::new(5.);
    assert_eq!((-length1).value, -7.); // Negation
    assert_eq!((length1 + length2).value, 12.); // Sum of two measures
    assert_eq!((length1 - length2).value, 2.); // Subtraction between two measures
    length1 += length2; // Increment of a measure
    assert_eq!(length1.value, 12.);
    length1 -= length2; // Decrement of a measure
    assert_eq!(length1.value, 7.);
    assert_eq!((length1 * 3.).value, 21.); // Multiplication of a measure by a number
    assert_eq!((3. * length1).value, 21.); // Multiplication of a number by a measure
    length1 *= 3.; // Upscaling of a measure
    assert_eq!(length1.value, 21.);
    assert_eq!((length1 / 3.).value, 7.); // Division of a measure by a number
    length1 /= 3.; // Downscaling of a measure
    assert_eq!(length1.value, 7.);
    assert_eq!(length1 / length2, 1.4); // Division between two measures of the same unit, obtaining a number
```

Here are some comparison operations:
```rust
    let length1 = Measure::<Metre>::new(7.);
    let length2 = Measure::<Metre>::new(5.);
    assert!(length1 == length1); // Equality
    assert!(length1 > length2); // Ordering
```

## Conversions between units

You can convert between units of measurement of the same physical or geometrical property using the following code:
```rust
    let distance1 = Measure::<KiloMetre>::new(100.);
    let distance2 = distance1.convert::<Mile>();
    let time1 = Measure::<Hour>::new(2.);
    let time2 = time1.convert::<Minute>();
    println!("The distances {distance1} and {distance2} are equivalent.");
    println!("The time spans {time1} and {time2} are equivalent.");
```

It will print:
```
The distances 100 km and 62.15040397762586 mi are equivalent.
The time spans 2 h and 120 min are equivalent.
```

You cannot convert between units of measurement representing different physical or geometrical properties:
```rust
    let _ = Measure::<KiloMetre>::new(100.).convert::<Hour>(); // ILLEGAL
    let _ = Measure::<Mile>::new(100.).convert::<SquareMile>(); // ILLEGAL
```

## Absolute values

Does it make sense to sum the temperature at which water freezes to the temperature at which water boils? No, it doesn't.

Does it make sense to multiply by 2 the date of today? No, it doesn't.

For some kinds of measures, to add two values of the same type or to multiply a value by a number do not make sense.

This happens because they are absolute measures, not relative measures, like the ones shown in the previous sections.

Mathematically speaking, such absolute measures are *points* of an *affine space*, while the relative measures, like lengths, temperature variations, and time durations are *vectors* of a *vector space*.
To avoid using points instead of vectors or vice versa, instead of using the generic type `Measure`, the generic type `MeasurePoint` should be used, like in the following code:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let variation = Measure::<Celsius>::new(2.);
    let point2 = point1 + variation;
    println!("When increasing temperature {point1} by {variation}, we reach temperature {point2}.");
```

It will print:
```
When increasing temperature at 10 °C by 2 °C, we reach temperature at 12 °C.
```

When a measure point is printed, the three characters "`at `" are prefixed to it.

With absolute values, the following operations are forbidden:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    let _ = point1 + point2; // ILLEGAL
    let _ = point1 * 3.; // ILLEGAL
```

On the other hand, some operations are specific of absolute values:
```rust
    let point1 = MeasurePoint::<Celsius>::new(10.);
    let point2 = MeasurePoint::<Celsius>::new(2.);
    let variation = point2 - point1; // Variation from point1 to point2.
    assert_eq!(variation.value, -8.);

    // We can compute the average between two points.
    assert_eq!(midpoint(point1, point2).value, 6.);

    // We can compute a weighted average between two points,
    // when the first point has a weight of 25% (equivalent to 0.25),
    // and consequently the second point has a weight of 75% (equivalent to 0.75).
    // This is a generalization of the function `midpoint`,
    // which is equivalent to `weighted_midpoint` with a weight of `0.5`.
    assert_eq!(weighted_midpoint(point1, point2, 0.25).value, 4.);

    // Having a slice of several points, each with its own weight,
    // we can compute the weighted average of all of them,
    // named *barycentric combination*.
    // To make sense, the sum of all the weights (0.1 + 0.4 + 0.5) must be 1.
    let point3 = MeasurePoint::<Celsius>::new(-6.);
    let center: MeasurePoint<Celsius> =
        barycentric_combination(&[point1, point2, point3], &[0.1, 0.4, 0.5]);
    assert_eq!(center.value, -1.2);
```

## Angles

With the library `rs-measures`, you can also handle angles.
Consider a screw mechanism, that can be turned by several cycles (or revolutions).
```rust
    let mut position = MeasurePoint::<Cycle>::new(3.);
    let rotation = Measure::<Degree>::new(90.).convert::<Cycle>();
    position += rotation;
    assert_eq!(position.value, 3.25);
```

The initial position of the screw is 3 cycles. We turn it by 90 degrees, that is a quarter of a cycle, and so we get a position of three cycles and one quarter.

For some applications, such unconstrained angular positions and rotations are useful.
Though, for some other applications, angles are used just to indicate a direction, or a spherical coordinate.
For such cases, when a full cycle is completed, we reach the same position than before.
It is a kind of *modulo* arithmetics.

This is supported by this library, as shown in this code:
```rust
    let mut direction = UnsignedDirection::<Degree>::new(350.);
    let rotation = Measure::<Degree>::new(30.);
    direction += rotation;
    assert_eq!(direction.value, 20.);
```

When 30 is added to 350, you get 380, but because we want to restrict our angles in the range 0 to 360 degrees, that value becomes 20 degrees.

The name `UnsignedDirection` contains the word `Direction` because it is meant to represent directions in the plane, not positions of a screw.

And it contains the word `Unsigned` because its values are constrained to be non-negative numbers.

Another common convention to represent directions is to use the range from -180 to 180 degrees. For example, it is the one used to represent geographical coordinates.

Here is an example using such a type:
```rust
    let mut direction = SignedDirection::<Degree>::new(160.);
    let rotation = Measure::<Degree>::new(30.);
    direction += rotation;
    assert_eq!(direction.value, -170.);
```

In this case, we used the type `SignedDirection`, and so when its value is incremented from 160 to 190 degrees, that value becomes -170 degrees.

So, we have seen that there are three types of direction: `MeasurePoint<Unit>`, where `Unit` is an angular unit of measurement, `UnsignedDirection`, and `SignedDirection`.
To convert between them, of course, it is possible to pass through the naked angle number, by accessing their field `value`.
Though, there are also some functions that allow a direct conversion:
```rust
    let angle_measure_point = MeasurePoint::<Degree>::new(-362.);
    assert_eq!(angle_measure_point.value, -362.);

    // From MeasurePoint to SignedDirection, -362. becomes -2.
    let signed_direction = SignedDirection::from_measure_point(angle_measure_point);
    assert_eq!(signed_direction.value, -2.);

    // From MeasurePoint to UnsignedDirection, -362. becomes 358.
    let unsigned_direction = UnsignedDirection::from_measure_point(angle_measure_point);
    assert_eq!(unsigned_direction.value, 358.);

    // From UnsignedDirection to MeasurePoint, 358. remains 358.
    let mp2 = unsigned_direction.to_measure_point();
    assert_eq!(mp2.value, 358.);

    // From UnsignedDirection to SignedDirection, 358. becomes -2.
    let sd2 = unsigned_direction.to_signed_direction();
    assert_eq!(sd2.value, -2.);

    // From SignedDirection to MeasurePoint, -2. remains -2.
    let mp3 = signed_direction.to_measure_point();
    assert_eq!(mp3.value, -2.);

    // From SignedDirection to UnsignedDirection, -2. becomes 358.
    let ud2 = signed_direction.to_unsigned_direction();
    assert_eq!(ud2.value, 358.);
```

The next section explain how to use 2D measures in a plane or 3D measures in space.
If you are not interested in them, you can jump to the section [Mixed-unit operations](#mixed-unit-operations).

## Working in a plane

If our measures represent vectors or points in a plane, they must be represented by two numbers, which may be its norm and direction or its X and Y components.
The rs-library uses the latter representation, like this example shows:
```rust
    let position1 = MeasurePoint2d::<Metre>::new(4., 7.);
    let displacement = Measure2d::<Metre>::new(2., -12.);
    let position2 = position1 + displacement;
    println!("After I moved from position {position1} in the plane by {displacement}, my position was {position2}.");
```

It will print:
```
After I moved from position at (4, 7) m in the plane by (2, -12) m, my position was at (6, -5) m.
```

Notice that relative 2D-measures are encapsulated in the type `Measure2d`, and absolute 2D-measures in the type `MeasurePoint2d`.

They are printed as a tuple of numbers, followed by a single unit symbol.
Absolute measures are precede by "`at `".

Actually such objects contain just two numbers. You can access them as shown in this code:
```rust
    let position = Measure2d::<Metre>::new(4., 7.);
    let x_value: f64 = position.x;
    let y_value: f64 = position.y;
    let x_measure: Measure::<Metre> = position.x();
    let y_measure: Measure::<Metre> = position.y();
    println!("The numeric components of {position} are {x_value} and {y_value}.");
    println!("Its measure components are {x_measure} and {y_measure}.");
```

It will print:
```
The numeric components of (4, 7) m are 4 and 7.
Its measure components are 4 m and 7 m.
```

Many operations available for 1D-measures are also available for 2D-measures, with some reasonable changes.
Here are some examples:
```rust
    let m = Measure2d::<Metre>::new(3., 4.);
    let squared_norm = m.squared_norm();
    let norm = squared_norm.sqrt();
    println!("The squared norm of {m} is {squared_norm}; its norm is {norm}.");
```

It will print:
```
The squared norm of (3, 4) m is 25; its norm is 5.
```

The method `m.squared_norm()` returns the value of the formula `m.x * m.x + m.y * m.y`.
According with the Pythagorean theorem, this value is the square of the length of the vector.
Such a value can be useful by itself, or it can be used to compute the length of the vector, a.k.a. its *norm*, by computing its square root.

```rust
    let m = Measure2d::<Metre>::new(3., 4.);
    let normalized = m.normalized();
    let norm_of_normalized = normalized.squared_norm().sqrt();
    println!("The norm of {normalized} is {norm_of_normalized}.");
```

It will print:
```
The norm of (0.6000000000000001, 0.8) m is 1.
```

The method `normalized` returns a vector having the same direction of the original vector, but with norm 1.

The vector whose all components are zero, is known as the *zero vector*.
It cannot really be normalized:
```rust
fn main() {
    let zero_vector = Measure2d::<Metre>::new(0., 0.);
    let norm_of_zero_vector = zero_vector.squared_norm().sqrt();
    let normalized_zero_vector = zero_vector.normalized();
    println!("The norm of {zero_vector} is {norm_of_zero_vector}; by normalizing it, we get {normalized_zero_vector}.");
}
```

It will print:
```
The norm of (0, 0) m is 0; by normalizing it, we get (NaN, NaN) m.
```

Its norm is zero, and if we try to normalize it, we obtain a vector whose components are `NaN` (not-a-number).

2D measures can be compared for equality, but not for order:
```rust
    let m1 = Measure2d::<Metre>::new(3., 4.);
    let m2 = Measure2d::<Metre>::new(4., 3.);
    assert!(m1 != m2);

    // binary operation `<` cannot be applied to type `units::Measure2d<units::Metre>`
    // assert!(m1 < m2);
```

## Converting a direction to a `Measure2d` and conversely

As we said before, a 2D measure is associated to a direction in a plane; though, also the types `MeasurePoint` of an angle unit, `UnsignedDirection` and `SignedDirection` are associated to a direction in a plane.

Therefore we can, in a way, conversions between them:
```rust
    let downward = MeasurePoint::<Degree>::new(-90.);
    println!("The direction downward (that is {downward}) can be used to construct");
    println!("a 2D measure having any unit and norm 1.");
    let m = Measure2d::<MilePerHour>::from_direction(downward);
    println!("Using miles per hour, we get {m}.");
```

It will print:
```
The direction downward (that is at -90 deg) can be used to construct
a 2D measure having any unit and norm 1.
Using miles per hour, we get (0.00000000000000006123233995736766, -1) mi/h.
```

So, the function `from_direction` gets a `MeasurePoint` having an angle as its unit, and constructs a `Measure2d` of norm 1.
Because any measure must have a unit, the unit of the constructed 2D vector must be specified or inferred from the statement in which this function is used.

Conversely, any object of type Measure2d can be converted to an `UnsignedDirection` by the method `unsigned_direction`, and it can be converted to a `SignedDirection` by the method `signed_direction`.
Also in this case the unit of the destination angle must be specified or inferred:
```rust
    let m = Measure2d::<MilePerHour>::new(0., -10.);
    let signed_direction = m.signed_direction::<Degree>();
    println!("A 2D measure can be used to construct its signed direction {signed_direction}");
    let unsigned_direction = m.unsigned_direction::<Degree>();
    println!("or its unsigned direction {unsigned_direction}.");
```

It will print:
```
A 2D measure can be used to construct its signed direction at -90 deg (in -180°..180°)
or its unsigned direction at 270 deg (in 0°..360°).
```

## Working in the 3D space

Similarly to 2D measures, 3D measures are supported:
```rust
    let position1 = MeasurePoint3d::<Metre>::new(4., 7., -3.);
    let displacement = Measure3d::<Metre>::new(2., -12., -2.);
    let position2 = position1 + displacement;
    println!("After I moved from position {position1} in the space");
    println!("by {displacement}, my position was {position2}.");
```

It will print:
```
After I moved from position at (4, 7, -3) m in the space
by (2, -12, -2) m, my position was at (6, -5, -5) m.
```

Notice that relative 3D-measures are encapsulated in the type `Measure3d`, and absolute 2D-measures in the type `MeasurePoint3d`.

Of course, the member `z` and the function `z()` have been added.

They are very similar to 2D measures, excluding the conversions from and to directions.

## Linear transformations in a plane

When working with vectors or points, some geometrical operations are quite common.

First, let's see how a rotation of a vector in a plane can be performed:
```rust
    let m1 = Measure2d::<Metre>::new(3., 1.);
    let angle = Measure::<Degree>::new(20.);
    let rotation = LinearMap2d::rotation(angle);
    let m2 = rotation.apply_to(m1);
    println!("If measure {m1} is rotated counterclockwise by {angle},");
    println!("measure {m2} is obtained.");
```

It will print:
```
If measure (3, 1) m is rotated counterclockwise by 20 deg,
measure (2.4770577190320564, 1.9657530507629146) m is obtained.
```

To rotate a vector by an angle, we don't use a method of the vector, nor a method of the angle.
Instead, we create an object specialized in 2D linear transformation, whose type is `LinearMap2d`.
Rotations of vectors in a plane happen to be 2D linear transformations, and so we use such an object.

To create a specific linear transformation representing a rotation, the method `rotation` is used.
It receives an angle, and creates an object capable of rotating any vector by that angle.

To rotate a vector using a linear transformation, the method `apply_to` is called, and the vector is specified to it.

How is implemented a 2D linear transformation?
It is just a 2x2 matrix of numbers:
```rust
    let angle = Measure::<Degree>::new(20.);
    let rotation = LinearMap2d::rotation(angle);
    println!("A rotation by {angle} is this matrix:\n{rotation}.");
```

It will print:
```
A rotation by 20 deg is this matrix:
[0.9396926207859084 -0.3420201433256687]
[0.3420201433256687  0.9396926207859084].
```

Linear transformation do not depend on units of measurement.

There are linear transformation objects to perform other kinds of transformations:
```rust
    let line_vector = Measure2d::<Unspecified>::new(3., 1.);
    let unit_vector = line_vector.normalized();
    let projection = LinearMap2d::projection_by_unit_vector(unit_vector);
    let m1 = Measure2d::<MilePerHour>::new(2., 5.);
    let m2 = projection.apply_to(m1);
    println!("If vector {m1} is projected onto the line whose direction");
    println!("is that of the vector {line_vector},");
    println!("the vector {m2} is obtained.");
```

It will print:
```
If vector (2, 5) mi/h is projected onto the line whose direction
is that of the vector (3, 1),
the vector (3.3, 1.1) mi/h is obtained.
```

We have a planar measure used to specify a direction. Its unit is `Unspecified`; it is a kind of dummy unit, to use when we don't need a real unit of measurement.

Then, we need to normalize it, otherwise the result will make no sense.

Then, we create a linear transformation using that unit vector, by calling the method `projection_by_unit_vector`.

Then, we create the planar measure that we want to project.

And finally, we apply the linear transformation to the planar measure, obtaining another measure having the same unit.

Here is the complete list of methods of `LinearMap2d`, which can be used to create linear transformations in a plane:
* `new(coefficients: [[Number; 2]; 2])`: It allows to construct any 2D linear transformation, if you know its 4 coefficients.
* `scaling(kx: Number, ky: Number)`: It returns a transformation which multiplies X coordinates by `kx` and Y coordinates by `ky`. To have an isotropic scaling, `kx` must be equal to `ky`.
* `rotation(angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle.
* `rotation_at_left()`: It is equivalent to `rotation(Measure::<Degree>::new(90.))`. Though, it has no rounding and it is more efficient.
* `rotation_at_right()`: It is equivalent to `rotation(Measure::<Degree>::new(-90.))`. Though, it has no rounding and it is more efficient.
* `projection_by_point_angle(direction: MeasurePoint)`: It returns a transformation which projects vectors onto the line having the direction specified by an absolute angle.
* `projection_by_unsigned_direction(direction: UnsignedDirection)`: Similar to `projection_by_point_angle`, but using an `UnsignedDirection`.
* `projection_by_signed_direction(direction: SignedDirection)`: Similar to `projection_by_point_angle`, but using a `SignedDirection`.
* `projection_by_unit_vector(unit_vector: Measure2d)`: Similar to `projection_by_point_angle`, but using a `Measure2d` of any unit, but having norm one.
* `reflection_by_point_angle(direction: MeasurePoint)`: It returns a transformation which reflects (a.k.a. *mirrors*) over the line having the direction specified by an absolute angle.
* `reflection_by_unsigned_direction(direction: UnsignedDirection)`: Similar to `reflection_by_point_angle`, but using an `UnsignedDirection`.
* `reflection_by_signed_direction(direction: SignedDirection)`: Similar to `reflection_by_point_angle`, but using a `SignedDirection`.
* `reflection_by_unit_vector(unit_vector: Measure2d)`: Similar to `reflection_by_point_angle`, but using a `Measure2d` of any unit, but having norm one.

## Manipulating transformations

As we have seen, you have a transformation object, you can use it by calling its method `apply_to`.
Tough, you can also do other operations on a transformation object:
* `inverted()`: It returns a 2d linear transformation which has the opposite effect of the original transformation. For example, the inverse of a clockwise rotation is a counterclockwise rotation; and the inverse of a scaling which double the size is a scaling which halves the size.
* `combined_with(map: &LinearMap2d)`: It returns a 2D linear transformation which is equivalent to applying first the argument `map` of the call,and then the transformation on which this function is called. Notice that transformations are non commutative, and so `lm1.combined_with(&lm2)` is not equal to `lm2.combined_with(&lm1)`.

## Affine transformations in a plane

2D linear transformation can operate only on 2d vectors, i.e. values of type `Measure2d`.
Though, you may need to transform some points in a plane.
Here is how to rotate a point in a plane around another specified point:

```rust
    let mp1 = MeasurePoint2d::<Metre>::new(3., 1.);
    let fixed_point = MeasurePoint2d::<Metre>::new(1., 2.);
    let angle = Measure::<Degree>::new(20.);
    let rotation = AffineMap2d::rotation(fixed_point, angle);
    let mp2 = rotation.apply_to(mp1);
    println!("If measure point {mp1} is rotated counterclockwise by {angle}");
    println!("around measure point {fixed_point},");
    println!("measure point {mp2} is obtained.");
```

It will print:
```
If measure point at (3, 1) m is rotated counterclockwise by 20 deg
around measure point at (1, 2) m,
measure point at (3.2214053848974853, 1.744347665865429) m is obtained.
```

The first thing to notice is that the type `AffineMap2d` is used, instead of the type `LinearMap2d`. This is because, to transform a point measure, an affine transformation is needed.

For many affine transformations it is needed to specify a fixed point in the transformation. Such a point must have the same unit of measurement of the point we want to transform, and so also the objects of type `AffineMap2d` will be characterized by that unit.
You cannot use an `AffineMap2d<KiloMetre>` to transform a `MeasurePoint2d<Mile>`.

This introduces the need to change the unit of an affine transformation, like this code does:
```rust
    let fixed_point = MeasurePoint2d::<Metre, f32>::new(2., 3.);
    let angle = Measure::<Degree, f32>::new(20.);
    let rotation_in_metres = AffineMap2d::rotation(fixed_point, angle);
    println!("The rotation matrix in metres is:");
    println!("{rotation_in_metres}");
    let rotation_in_yards = AffineMap2d::rotation(fixed_point.convert::<Yard>(), angle);
    println!("The rotation matrix in yards is:");
    println!("{rotation_in_yards}");
    let rotation_converted_from_yards_to_metres = rotation_in_yards.convert::<Metre>();
    println!("The rotation matrix converted from yards to metres is:");
    println!("{rotation_converted_from_yards_to_metres}");
```

It will print:
```
The rotation matrix in metres is:
[0.9396926  -0.34202012  1.1466751] m
[0.34202012  0.9396926  -0.5031183]
The rotation matrix in yards is:
[0.9396926  -0.34202012  1.254019 ] yd
[0.34202012  0.9396926  -0.5502167]
The rotation matrix converted from yards to metres is:
[0.9396926  -0.34202012  1.146675 ] m
[0.34202012  0.9396926  -0.5031181]
```

Here you can see that affine matrices are 2x3 matrices of numbers, and they have a unit of measurement.

The matrix in yards differs from the one in metres only for its third column.
When this matrix is converted to metres, it becomes equal to the first matrix, with a small rounding error.

An available affine transformation which ha no sense for linear transformations is *translation*.
Here is an example of it:
```rust
    let displacement = Measure2d::<Metre>::new(7., -12.);
    let translation = AffineMap2d::translation(displacement);
    let mp1 = MeasurePoint2d::<Metre>::new(3., 1.);
    let mp2 = translation.apply_to(mp1);
    println!("If measure point {mp1} is translated by {displacement},");
    println!("measure point {mp2} is obtained.");
    let mp3 = mp1 + displacement;
    println!("If such displacement is just added to the measure point,");
    println!("measure point {mp3} is obtained.");
```

It will print:
```
If measure point at (3, 1) m is translated by (7, -12) m,
measure point at (10, -11) m is obtained.
If such displacement is just added to the measure point,
measure point at (10, -11) m is obtained.
```

As you can see, the result is the same of just adding the displacement.
Therefore, translation transformations are actually useful only when they are combined with other affine transformations.

Here is the complete list of methods of `AffineMap2d`, which can be used to create affine transformations in a plane:
* `new(coefficients: [[Number; 3]; 2])`: It allows to construct any 2D affine transformation, if you know its 6 coefficients.
* `translation(displacement: Measure2d)`: It returns a transformation which adds the specified vector to measure points.
* `scaling(fixed_point: MeasurePoint2d, kx: Number, ky: Number)`: It returns a transformation which multiplies by the specified factors the differences with the specified fixed point.
* `rotation(fixed_point: MeasurePoint2d, angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle, around the specified fixed point.
* `rotation_at_left(fixed_point: MeasurePoint2d)`: It is equivalent to `rotation(fixed_point: MeasurePoint2d, Measure::<Degree>::new(90.))`. Though, it has no rounding and it is more efficient.
* `rotation_at_right(fixed_point: MeasurePoint2d)`: It is equivalent to `rotation(fixed_point: MeasurePoint2d, Measure::<Degree>::new(-90.))`. Though, it has no rounding and it is more efficient.
* `projection_by_point_angle(fixed_point: MeasurePoint2d, direction: MeasurePoint)`: It returns a transformation which projects vectors onto the line going through the specified fixed point and having the direction specified by an absolute angle.
* `projection_by_unsigned_direction(fixed_point: MeasurePoint2d, direction: UnsignedDirection)`: Similar to `projection_by_point_angle`, but using an `UnsignedDirection`.
* `projection_by_signed_direction(fixed_point: MeasurePoint2d, direction: SignedDirection)`: Similar to `projection_by_point_angle`, but using a `SignedDirection`.
* `projection_by_unit_vector(fixed_point: MeasurePoint2d, unit_vector: Measure2d)`: Similar to `projection_by_point_angle`, but using a `Measure2d` of any unit, but having norm one.
* `reflection_by_point_angle(fixed_point: MeasurePoint2d, direction: MeasurePoint)`: It returns a transformation which reflects (a.k.a. *mirrors*) over the line going through the specified fixed point and having the direction specified by an absolute angle.
* `reflection_by_unsigned_direction(fixed_point: MeasurePoint2d, direction: UnsignedDirection)`: Similar to `reflection_by_point_angle`, but using an `UnsignedDirection`.
* `reflection_by_signed_direction(fixed_point: MeasurePoint2d, direction: SignedDirection)`: Similar to `reflection_by_point_angle`, but using a `SignedDirection`.
* `reflection_by_unit_vector(fixed_point: MeasurePoint2d, unit_vector: Measure2d)`: Similar to `reflection_by_point_angle`, but using a `Measure2d` of any unit, but having norm one.

## Transformations in 3d space

Regarding linear transformations of `Measure3d` objects and affine transformations of `MeasurePoint3d` objects, little is to be explained, because they are quite similar to the corresponding planar transformation, replacing `2d` with `3d`, and specifying the additional Z component when required.

The differences to take into account are these:
* There are no specific rotation at right or rotation at left.
* Line directions are specified only by 3D unit vectors, not by measure point angles, nor by unsigned directions, nor by signed directions.
* In addition to projections and reflections about a line, projections onto a plane and reflections over a plane are possible. Such planes are specified by a unit vector orthogonal to them.

Here is the complete list of methods of `LinearMap3d`, which can be used to create linear transformations in the space:
* `new(coefficients: [[Number; 3]; 3])`: It allows to construct any 3D linear transformation, if you know its 9 coefficients.
* `scaling(kx: Number, ky: Number, kz: Number)`: It returns a transformation which multiplies X coordinates by `kx`, Y coordinates by `ky`, and Z coordinates by `kz`. To have an isotropic scaling, `kx` must be equal to `ky` and to `kz`.
* `rotation(unit_vector: Measure3d, angle: Measure)`: It returns a transformation which rotates vectors counterclockwise by the specified angle around the axis specified by a unit vector.
* `projection_onto_line(unit_vector: Measure3d)`: It returns a transformation which projects vectors onto the line having the direction specified by a unit vector.
* `projection_onto_plane(unit_vector: Measure3d)`: It returns a transformation which projects vectors onto the plane specified by its orthogonal unit vector.
* `reflection_over_line(unit_vector: Measure3d)`: It returns a transformation which reflects vectors over the line having the direction specified by a unit vector.
* `reflection_over_plane(unit_vector: Measure3d)`: It returns a transformation which reflects vectors over the plane specified by its orthogonal unit vector.

Here is the complete list of methods of `AffineMap3d`, which can be used to create affine transformations in the space:
* `new(coefficients: [[Number; 4]; 3])`: It allows to construct any 3D affine transformation, if you know its 12 coefficients.
* `translation(displacement: Measure3d)`: It returns a transformation which adds the specified vector to measure points.
* `scaling(fixed_point: MeasurePoint3d, kx: Number, ky: Number, kz: Number)`: It returns a transformation which multiplies by the specified factors the differences with the specified fixed point.
* `rotation(fixed_point: MeasurePoint3d, unit_vector: Measure3d, angle: Measure)`: It returns a transformation which rotates points counterclockwise by the specified angle, around the axis going through the specified fixed point and having the direction specified by a unit vector.
* `projection_onto_line(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which projects points onto the line going through the specified fixed point and having the direction specified by a unit vector.
* `projection_onto_plane(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which projects points onto the plane going through the specified fixed point and having the specified orthogonal unit vector.
* `reflection_over_line(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which reflects points over the line going through the specified fixed point and having the direction specified by a unit vector.
* `reflection_over_plane(fixed_point: MeasurePoint3d, unit_vector: Measure3d)`: It returns a transformation which reflects points over the plane going through the specified fixed point and having the specified orthogonal unit vector.

## Mixed-unit operations

So far, we have never multiplied one measure by another, nor divided two measures having different units.
Though, using the library rs-measures, it is allowed to write:
```rust
    let length = Measure::<Metre>::new(10.);
    let time = Measure::<Second>::new(4.);
    let velocity: Measure<MetrePerSecond> = length / time;
    println!("If I move by a distance of {length} in a time span of {time},");
    println!("I have an average speed of {velocity}.");
```

It will print:
```
If I move by a distance of 10 m in a time span of 4 s,
I have an average speed of 2.5 m/s.
```

When a measure having unit `Metre` is divided by a measure having unit `Second`, the result is a measure having unit `MetrePerSecond`.

Also square roots are supported:
```rust
    use rs_measures::traits::Sqrt;
    let length = Measure::<Metre>::new(10.);
    let width = Measure::<Metre>::new(4.);
    let area: Measure<SquareMetre> = length * width;
    println!("If a rectangle has length {length} and width {width},");
    println!("its area is {area}.");
    let side: Measure<Metre> = area.sqrt();
    println!("If a square had that area, its side would be {side}.");
```

It will print:
```
If a rectangle has length 10 m and width 4 m,
its area is 40 m².
If a square had that area, its side would be 6.324555320336759 m.
```

When a measure having unit `Metre` is multiplied by a measure having the same unit, the result is a measure having unit `SquareMetre`.
And when the square root is extracted from a measure having unit `SquareMetre`, the result is a measure having unit `Metre`.

Such operations are available also in two or three dimensions:
```rust
    let distance = Measure3d::<Metre>::new(10., 20., 40.);
    let time = Measure::<Second>::new(4.);
    let velocity: Measure3d<MetrePerSecond> = distance / time;
    println!("If I move by a distance of {distance} in a time span of {time},");
    println!("I have an average speed of {velocity}.");
```

It will print:
```
If I move by a distance of (10, 20, 40) m in a time span of 4 s,
I have an average speed of (2.5, 5, 10) m/s.
```

And for some units of measurement, several kinds of multiplication are possible, depending on the dimension of the space.
For example, using force and displacement, in one dimension, we can have just a simple multiplication.
```rust
    let force = Measure::<Newton>::new(10.);
    let distance = Measure::<Metre>::new(4.);
    let work: Measure<Joule> = force * distance;
    println!("If a force of {force} is applied to an object,");
    println!("while it moves in the same direction by {distance},");
    println!("a work of {work} is performed.");
```

It will print:
```
If a force of 10 N is applied to an object,
while it moves in the same direction by 4 m,
a work of 40 J is performed.
```

But in a plane we can have two kinds of multiplication: that are the dot-product and the cross-product:
```rust
    use rs_measures::traits::CrossProduct;
    let force = Measure2d::<Newton>::new(10., 20.);
    let distance = Measure2d::<Metre>::new(4., 3.);
    let work: Measure<Joule> = force * distance;
    let torque: Measure<NewtonMetre> = force.cross_product(distance);
    println!("If, in a plane, a force of {force} is applied to an object,");
    println!("while it moves in that plane by {distance},");
    println!("a work of {work} is performed.");
    println!("If that force is applied to a point");
    println!("which is detached from a pivot by {distance},");
    println!("that force causes a torque of {torque}.");
```

It will print:
```
If, in a plane, a force of (10, 20) N is applied to an object,
while it moves in that plane by (4, 3) m,
a work of 100 J is performed.
If that force is applied to a point
which is detached from a pivot by (4, 3) m,
that force causes a torque of -50 N·m.
```

Notice that the operator `*`, when applied to two 2D vectors, computes the dot-product between them. Instead, to compute the cross-product, the method `cross_product` is called. It needs the declaration:
```
    use rs_measures::traits::CrossProduct;
```

In physics, while the dot-product between a force and a displacement represents a work (or energy), and so that expression returns a measure in `Joule`, a cross-product between a force and a displacement represents a torque (or moment of force), and so that expression returns a measure in `NewtonMetre`; it is a another unit of measurement, incompatible with `Joule`.

In 3D space we have this:
```rust
    use rs_measures::traits::CrossProduct;
    let force = Measure3d::<Newton>::new(10., 20., 0.4);
    let distance = Measure3d::<Metre>::new(4., 3., -5.);
    let work: Measure<Joule> = force * distance;
    let torque: Measure3d<NewtonMetre> = force.cross_product(distance);
    println!("In space, if a force of {force} is applied to an object,");
    println!("while it moves by {distance},");
    println!("a work of {work} is performed.");
    println!("If that force is applied to a point");
    println!("which is detached from a pivot by {distance},");
    println!("that force causes a torque of {torque}.");
```

It will print:
```
In space, if a force of (10, 20, 0.4) N is applied to an object,
while it moves by (4, 3, -5) m,
a work of 98 J is performed.
If that force is applied to a point
which is detached from a pivot by (4, 3, -5) m,
that force causes a torque of (-101.2, 51.6, -50) N·m.
```

Also in this case the dot-product return a scalar, having type `Measure<Joule>`.

Though, regarding cross-product, the result is different.
In a plane, the torque is always perpendicular to the plane, and so we are not interested in its X and Y components, which are always zero. Therefore, that expression returns a scalar, containing the only interesting component of the torque, Z.

Instead, in 3D space, the torque is a vector which could have any direction, and so the cross-product returns a value of type `Measure3d`.

## How units of measurement and their relationships are defined

So far, we have used types, functions, and units of measurement without needing to define them.
Actually, our small program begins by importing all there is in the module `units`, defined by yourself at the beginning of this tutorial.

Let's see its contents.

### Defining measure types

It begins with this statement:
```rust
rs_measures::define_1d_2d_3d! {}
```

This is a macro invocation which expands to the code which defines these types:
* `Measure`
* `MeasurePoint`
* `UnsignedDirections`
* `SignedDirections`.
* `Measure2d`
* `MeasurePoint2d`
* `LinearMap2d`
* `AffineMap2d`.
* `Measure3d`
* `MeasurePoint3d`
* `LinearMap3d`
* `AffineMap3d`.


Therefore, if there is the need to work in 1D, in 2D, in 3D and using angular directions, the macro `define_1d_2d_3d` should be used. It will declare every kind of measure, directions and transformations.

Though, if only some of such types are needed, you can use some other macros, with reduce the number of declared types, and so the resulting code is quicker to compile, and developers are not bothered by unneeded types.

If only 1D and 3D measures and transformations are needed, with no angular directions, it is better to use the macro `define_1d_3d`. It will declare only 1D and 3D measures and transformations, skipping 2D measures and transformations and angular directions.

If only 1D and 2D measures and transformations are needed, possibly with angular directions, it is better to use the macro `define_1d_2d`. It will declare only 1D and 2D measures and transformations, and the angular directions, skipping 3D measures and transformation.

If only 1D measures and angular directions are needed, it is better to use the macro `define_1d_and_directions`. It will define only 1D measures and the angular directions, skipping both 2D and 3D measures and transformations.

And at last, if only 1D measures are needed, it is better to use the macro `define_1d`. It will define only 1D measures, skipping 2D and 3D measures and transformations, and the angular directions.

Actually these definitions do not define all the units of measurement we used in our examples.

The only predefined unit of measurement is `Radian` which is used for angles. Every other unit must be explicitly defined.

### Defining properties

Actually, if you go on reading the file `units.rs`, you will see at lines 4 and 5:
```rust
pub struct Acceleration;
impl VectorProperty for Acceleration {}
```

It defines a *property*. A (physical or geometrical) property is something you want to measure, and, for such a purpose, it needs one or more units of measurement.

Its main purpose is to avoid unit conversions which do not make sense.
For example, `Mile`, `Inch`, `NanoMetre` and `KiloMetre` are all units of the property `Length` and so it will be allowed to convert between two of them. Instead, `MetrePerSquareSecond` is a unit of the property `Acceleration`, and so it will be forbidden to convert from `Mile` or `MetrePerSquareSecond` or conversely.

For some properties, it makes sense to have a 2D measure or 3D measure, and for others it does make no sense. For example, length, velocity, force, torque, electric field strength, and magnetic field strength are *vector properties*. Instead mass, temperature, time, and electric charge are *scalar properties*.

It is useful to forbid creating vectors for scalar properties.
To such a purpose, the statement `impl VectorProperty for Acceleration {}` just marks the property `Acceleration` as a vector property.
Later, any attempt to create a 2D or 3D measure with a unit of this property will be allowed.
For example, this is allowed: `Measure3d<MetrePerSquareSecond>`.

Instead, the declaration of time should simply be this, without any `impl` statement:
```rust
pub struct Time;
```

Later, any attempt to create a 2D or 3D measure with a unit of this property will generate a compilation error.
For example, this is not allowed: `Measure3d<Second>`.

### Defining units of measurement

At line 7 of file `units.rs` there is:
```rust
pub struct MetrePerSquareSecond;
```

It defines a unit of measurement.

The struct by itself has no content, because it is used just for its type, not for its value.
Its type is decorated in the following lines:
```rust
impl MeasurementUnit for MetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}
```

The field `Property` asserts the property of this unit.
As we said, we mean `MetrePerSquareSecond` to be an `Acceleration`.

For every property, there should be in your mind a base unit.
Let's assume `MetrePerSquareSecond` is the base unit for acceleration, according with the SI international standard.
The field `RATIO` specifies how many base units for its property are contained in this unit.
Being this the base unit, its `RATIO` is `1.`.

It is not required that the base unit is actually defined.
For example, we could have defined just this unit for acceleration:
```rust
impl MeasurementUnit for CentiMetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1e-2;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s\u{b2}";
}
```

For the unit `CentiMetrePerSquareSecond`, `RATIO` is `1e-2`, meaning that one hundredth of metre per square second is equivalent to one centimetre per square second.
For the unit `Mile`, `RATIO` is `1609.`, meaning that 1609 metres is equivalent to one mile.

The field `OFFSET` is needed for units having an origin different from the origin of the base unit.
It has almost always a zero value, because almost all units have the same origin that their base unit.
In `units.rs`, it has different values only for temperature degrees.
Actually, it represents the displacement from the origin of the base unit, represented in units of the base unit, to reach the origin of the current unit.

For example, for the unit `Fahrenheit`, it is `273.15 - 32. * 5. / 9.`.
The base unit for temperature is `Kelvin`.
Starting from the origin of the Kelvin scale, if we add `273.15` Kelvin degrees (reaching the origin of the Celsius scale), and then we subtract `32. * 5. / 9.` Kelvin degrees, we reach the origin of the `Fahrenheit` scale.

Another possible use of `OFFSET` is for calendars, because different ways to represent times can have different origins.

Instead, it shouldn't be used for vector units, like `Length`, because it makes no sense to define the same `OFFSET` for all components of a vector.

The field `SUFFIX` is a string which is printed after the numeric value, whenever a measure is printed or converted to string.

### Defining units of angles

Angles are a particular property.

The property `Angle` is the only predefined property.
The unit `Radian` is its base unit and, as we said, it is the only predefined unit.

You can define other angle units, but, to allow their use as directions, there declaration must be a bit different than the declaration of other units.
Whenever an angle unit is defined, also a definition like this is required:
```rust
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}
```

First of all, it declares that `Degree` is a kind of angle, but in addition it specifies how many units of this kind are contained in a cycle.

For the unit `Cycle`, `CYCLE_FRACTION` is `1.`, and for `Radian` it is `std::f64::consts::TAU` (which is approximately 6.28);

### Defining relationships among units

With the above declarations you can define measures, points, directions, and transformations, you can make conversions between units of the same property, you can compute additions, subtractions and divisions between measures of the same unit, and you can multiply or divide a measure by a number.

Though, you still cannot compute multiplications or divisions involving measures of different properties, like `Measure<Newton> * Measure<Metre>`.

To do that, you need to teach it to your application.

How to do this is shown in the file `units.rs` after line 2490.

For example, statement at line 2677 is this:
```rust
define_units_relation! {Joule == Newton * Metre}
```

It is a call to the procedural macro defined in the create `units-relation`.

It expands to four functions:
* One function allows you to multiply a `Measure<Newton>` by a `Measure<Meter>` getting a `Measure<Joule>`.
* One function allows you to multiply a `Measure<Meter>` by a `Measure<Newton>` getting a `Measure<Joule>`.
* One function allows you to divide a `Measure<Joule>` by a `Measure<Meter>` getting a `Measure<Newton>`.
* One function allows you to multiply a `Measure<Joule>` by a `Measure<Newton>` getting a `Measure<Meter>`.

It is followed, in file `units.rs`, by these statements:
```rust
define_units_relation! {Joule == Newton:2 * Metre:2}
define_units_relation! {Joule == Newton:3 * Metre:3}
```

They allow the same operations, but, respectively, in a plane or in the 3D space.

To specify the need to compute the cross-product, you can write:
```rust
define_units_relation! {NewtonMetre == Newton:2 X Metre:2}
```

It allows the operations `Measure2d<Newton>.cross_product(Measure2d<Metre>)` and `Measure2d<Metre>.cross_product(Measure2d<Newton>)`, both returning a `Measure<NewtonMetre>`. Notice that the return value type is a scalar.

The following expression is similar, but for 3D space.
```rust
define_units_relation! {NewtonMetre:3 == Newton:3 X Metre:3}
```

It allows the operations `Measure3d<Newton>.cross_product(Measure3d<Metre>)` and `Measure3d<Metre>.cross_product(Measure3d<Newton>)`, both returning a `Measure3d<NewtonMetre>`. Notice that the return value type is a 3D vector.

You can also write an expression like this one:
```rust
define_units_relation! {Watt == Joule / Second}
```

It is equivalent to the following one, but it may appear more intuitive:
```rust
define_units_relation! {Joule == Watt * Second}
```

In a couple of cases, you need just to specify that a unit is the opposite of another one. In such a case, you can write something like this:
```rust
define_units_relation! {Siemens == 1 / Ohm}
```

Only the value `1` is allowed here.

## Creating a custom file `units.ts`

The file `units.ts` is quite useful for learning, for experimenting, and for copying&pasting useful definitions.
Though, it is not recommended for production use, for the following reasons:
* Such large file increases your code base.
* Such large file increases compilation time.
* Such file uses words or output suffix you may dislike. For example, if you prefer, you could replace `Length` with `Space`, or `Metre` with `Meter`, or `" m"` with `" [m]"`.
* Such files have names which conflicts with some names already used in your project.
* Such files miss some properties or units or relations you may need.

Therefore, the suggested procedure for production code is the following one:
* Create your own file `units.rs` for your project.
* Add as its first statement `rs_measures::define_1d! {}`, possibly replacing `1d` with `1d_and_directions`, `1d_2d`, `1d_3d`, or `1d_2d_3d`, in case you need the types defined by such macros.
* Search the provided example file for the properties, the units, and the relations you need, or the ones most similar to what you need. Copy and paste them into your own file.
* Edit your file according to your needs.

To create a property means simply to define an empty `struct` with the desired name. For example: `pub struct Information;`.
If you need to define 2D or 3D measures having units of such a property, specify that it is a vector property, by implementing the empty trait `VectorProperty`.

To create a unit for a property, first you should decide, in case there will several units for that property, which of them is the base unit. For example, if you want to create the units `Bit` and `Byte` for property `Information`, you must decide which of them will be the base unit for you project, and which the derived one.

And then use the macro `define_units_relation!` to define possible relationship between units, preceded by this statement:
```rust
use units_relation::define_units_relation;
```

Here is an example of contents for a file `units.ts`;
```rust
rs_measures::define_1d! {}

pub struct Information;
pub struct Bit;
impl MeasurementUnit for Bit {
    type Property = Information;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b";
}

pub struct Byte;
impl MeasurementUnit for Byte {
    type Property = Information;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B";
}

pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct InformationRate;

pub struct BitPerSecond;
impl MeasurementUnit for BitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b/s";
}

pub struct BytePerSecond;
impl MeasurementUnit for BytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B/s";
}

use units_relation::define_units_relation;
define_units_relation! {BitPerSecond == Bit / Second}
define_units_relation! {BytePerSecond == Byte / Second}
```

Then, you can write a file `main.rs`, having this contents:
```rust
mod units;
use units::{Bit, Byte, Measure, Second};
fn main() {
    let info_size = Measure::<Byte>::new(2700.);
    let time = Measure::<Second>::new(5.1);
    println!("The transmission rate is {}", info_size.convert::<Bit>() / time);
}
```

Happy measuring!
