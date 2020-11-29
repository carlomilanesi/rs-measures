use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt;

pub trait FromF64 {
    fn from_f64(n: f64) -> Self;
}
impl FromF64 for f32 {
    fn from_f64(n: f64) -> Self {
        n as f32
    }
}
impl FromF64 for f64 {
    fn from_f64(n: f64) -> Self {
        n
    }
}

// /*
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}
impl Sqrt for f32 {
    type Output = f32;
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Sqrt for f64 {
    type Output = f64;
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait CubicRoot {
    type Output;
    fn cubic_root(self) -> Self::Output;
}
impl CubicRoot for f32 {
    type Output = f32;
    fn cubic_root(self) -> Self {
        self.powf(1. / 3.)
    }
}
impl CubicRoot for f64 {
    type Output = f64;
    fn cubic_root(self) -> Self {
        self.powf(1. / 3.)
    }
}

pub trait ArithmeticOps:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + FromF64
    + Sqrt<Output = Self>
    + CubicRoot<Output = Self>
    + fmt::Display
    + Clone
    + Copy
    + PartialOrd
    + PartialEq
where
    Self: std::marker::Sized,
{
}

// /*
impl<T> ArithmeticOps for T where
    T: Add<T, Output = T>
        + AddAssign<T>
        + Sub<T, Output = T>
        + SubAssign<T>
        + Mul<T, Output = T>
        + MulAssign<T>
        + Div<T, Output = T>
        + DivAssign<T>
        + FromF64
        + Sqrt<Output = Self>
        + CubicRoot<Output = Self>
        + fmt::Display
        + Clone
        + Copy
        + PartialOrd
        + PartialEq
{
}

/*
trait Vector<Number>:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Number, Output = Self>
    + MulAssign<Number>
    + Div<Number, Output = Self>
    + DivAssign<Number>
    + Sized
{
}

trait Affine<Number, V: Vector<Number>>:
    Add<V, Output = Self> + AddAssign<V> + Sub<V, Output = Self> + SubAssign<V> + Sized
{
}
*/

pub trait LossyFrom<Source> {
    fn lossy_from(n: Source) -> Self;
}

impl LossyFrom<f32> for f32 {
    fn lossy_from(n: f32) -> Self {
        n
    }
}
impl LossyFrom<f32> for f64 {
    fn lossy_from(n: f32) -> Self {
        n as Self
    }
}
impl LossyFrom<f64> for f32 {
    fn lossy_from(n: f64) -> Self {
        n as Self
    }
}
impl LossyFrom<f64> for f64 {
    fn lossy_from(n: f64) -> Self {
        n
    }
}

pub trait CrossProduct<Rhs = Self> {
    type Output;
    #[must_use]
    fn cross_product(self, rhs: Rhs) -> Self::Output;
}

pub trait MeasurementUnit {
    type Quantity;
    const RATIO: f64;
    const OFFSET: f64;
    const SUFFIX: &'static str;
}

pub trait AngleMeasurementUnit: MeasurementUnit {
    const TURN_FRACTION: f64;
}
