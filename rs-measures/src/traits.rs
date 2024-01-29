use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
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

pub trait Trigonometry {
    type Output;
    fn cos(self) -> Self::Output;
    fn sin(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn sin_cos(self) -> (Self::Output, Self::Output);
}
impl Trigonometry for f32 {
    type Output = f32;
    fn cos(self) -> Self::Output {
        self.cos()
    }
    fn sin(self) -> Self::Output {
        self.sin()
    }
    fn tan(self) -> Self::Output {
        self.tan()
    }
    fn sin_cos(self) -> (Self::Output, Self::Output) {
        self.sin_cos()
    }
}
impl Trigonometry for f64 {
    type Output = f64;
    fn cos(self) -> Self::Output {
        self.cos()
    }
    fn sin(self) -> Self::Output {
        self.sin()
    }
    fn tan(self) -> Self::Output {
        self.tan()
    }
    fn sin_cos(self) -> (Self::Output, Self::Output) {
        self.sin_cos()
    }
}

pub trait InverseTrigonometry {
    type Output;
    fn acos(self) -> Self::Output;
    fn asin(self) -> Self::Output;
    fn atan2(self, other: Self) -> Self::Output;
}
impl InverseTrigonometry for f32 {
    type Output = f32;
    fn acos(self) -> Self::Output {
        self.acos()
    }
    fn asin(self) -> Self::Output {
        self.asin()
    }
    fn atan2(self, other: Self) -> Self::Output {
        self.atan2(other)
    }
}
impl InverseTrigonometry for f64 {
    type Output = f64;
    fn acos(self) -> Self::Output {
        self.acos()
    }
    fn asin(self) -> Self::Output {
        self.asin()
    }
    fn atan2(self, other: Self) -> Self::Output {
        self.atan2(other)
    }
}

pub trait Decibel {
    fn to_decibel(self) -> Self;
    fn from_decibel(decibel_value: Self) -> Self;
}
impl Decibel for f32 {
    fn to_decibel(self) -> Self {
        self.log10() * 10.
    }
    fn from_decibel(decibel_value: Self) -> Self {
        10_f32.powf(decibel_value * 0.1)
    }
}
impl Decibel for f64 {
    fn to_decibel(self) -> Self {
        self.log10() * 10.
    }
    fn from_decibel(decibel_value: Self) -> Self {
        10_f64.powf(decibel_value * 0.1)
    }
}

pub trait HasZero {
    const ZERO: Self;
}
impl HasZero for f32 {
    const ZERO: Self = 0.;
}
impl HasZero for f64 {
    const ZERO: Self = 0.;
}

pub trait HasOne {
    const ONE: Self;
}
impl HasOne for f32 {
    const ONE: Self = 1.;
}
impl HasOne for f64 {
    const ONE: Self = 1.;
}

pub trait HasHalf {
    const HALF: Self;
}
impl HasHalf for f32 {
    const HALF: Self = 0.5;
}
impl HasHalf for f64 {
    const HALF: Self = 0.5;
}

pub trait HasSign {
    fn signum(self) -> Self;
}
impl HasSign for f32 {
    fn signum(self) -> Self {
        self.signum()
    }
}
impl HasSign for f64 {
    fn signum(self) -> Self {
        self.signum()
    }
}

pub trait ArithmeticOps:
    Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + FromF64
    + Sqrt<Output = Self>
    + CubicRoot<Output = Self>
    + Trigonometry<Output = Self>
    + InverseTrigonometry<Output = Self>
    + Decibel
    + HasZero
    + HasOne
    + HasHalf
    + HasSign
    + std::iter::Sum
    + fmt::Display
    + fmt::Debug
    + Clone
    + Copy
    + PartialOrd
    + PartialEq
where
    Self: std::marker::Sized,
{
}

impl<T> ArithmeticOps for T where
    T: Neg<Output = Self>
        + Add<T, Output = T>
        + AddAssign<T>
        + Sub<T, Output = Self>
        + SubAssign<T>
        + Mul<T, Output = Self>
        + MulAssign<T>
        + Div<T, Output = Self>
        + DivAssign<T>
        + Rem<T, Output = Self>
        + FromF64
        + Sqrt<Output = Self>
        + CubicRoot<Output = Self>
        + Trigonometry<Output = Self>
        + InverseTrigonometry<Output = Self>
        + Decibel
        + HasZero
        + HasOne
        + HasHalf
        + HasSign
        + std::iter::Sum
        + fmt::Display
        + fmt::Debug
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
    type Property;
    const RATIO: f64;
    const OFFSET: f64;
    const SUFFIX: &'static str;
}

pub trait AngleMeasurementUnit: MeasurementUnit {
    const CYCLE_FRACTION: f64;
}

pub trait VectorProperty {}
