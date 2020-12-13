#[macro_export]
macro_rules! define_measure1d {
    {} => {
        use rs_measures::{angle::{Angle, Radian}, traits::{ArithmeticOps, LossyFrom, MeasurementUnit, AngleMeasurementUnit, VectorMeasurementUnit}};
        use core::ops::{Neg, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
        use std::fmt;
        use std::marker::PhantomData;

        // Measure

        #[derive(Debug)]
        pub struct Measure<Number: ArithmeticOps, Unit> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Measure<Number, Unit> {
            pub fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure<Number, DestUnit> {
                Measure::<Number, DestUnit> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure<DestNumber, Unit> {
                Measure::<DestNumber, Unit> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure<DestNumber, Unit> {
                Measure::<DestNumber, Unit> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialEq<Measure<Number, Unit>> for Measure<Number, Unit> {
            fn eq(&self, other: &Measure<Number, Unit>) -> bool {
                self.value == other.value
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialOrd<Measure<Number, Unit>> for Measure<Number, Unit> {
            fn partial_cmp(&self, other: &Measure<Number, Unit>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for Measure<Number, Unit> {
            fn clone(&self) -> Self {
                Measure::<Number, Unit> {
                    value: self.value,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for Measure<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display for Measure<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.value, Unit::SUFFIX)
            }
        }

        // -Measure -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Neg for Measure<Number, Unit> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        // Measure * Number -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Mul<Number> for Measure<Number, Unit> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        // Measure *= Number
        impl<Number: ArithmeticOps, Unit> MulAssign<Number> for Measure<Number, Unit> {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        // f64 * Measure -> Measure
        impl<Unit: MeasurementUnit> Mul<Measure<f64, Unit>> for f64 {
            type Output = Measure<f64, Unit>;
            fn mul(self, other: Measure<f64, Unit>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // f32 * Measure -> Measure
        impl<Unit: MeasurementUnit> Mul<Measure<f32, Unit>> for f32 {
            type Output = Measure<f32, Unit>;
            fn mul(self, other: Measure<f32, Unit>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // Measure / Number -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Div<Number> for Measure<Number, Unit> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.value / n)
            }
        }

        // Measure /= Number
        impl<Number: ArithmeticOps, Unit> DivAssign<Number> for Measure<Number, Unit> {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        // Measure / Measure -> Number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Div<Measure<Number, Unit>> for Measure<Number, Unit> {
            type Output = Number;
            fn div(self, other: Measure<Number, Unit>) -> Self::Output {
                self.value / other.value
            }
        }

        // Measure + Measure -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Measure += Measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // Measure - Measure -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Measure -= Measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }

        // MeasurePoint

        #[derive(Debug)]
        pub struct MeasurePoint<Number, Unit> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> MeasurePoint<Number, Unit> {
            pub fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint<Number, DestUnit> {
                MeasurePoint::<Number, DestUnit> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint<DestNumber, Unit> {
                MeasurePoint::<DestNumber, Unit> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint<DestNumber, Unit> {
                MeasurePoint::<DestNumber, Unit> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // MeasurePoint + Measure -> MeasurePoint
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // MeasurePoint += measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // MeasurePoint - Measure -> MeasurePoint
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // MeasurePoint -= Measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }

        // MeasurePoint - MeasurePoint -> Measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<MeasurePoint<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            type Output = Measure<Number, Unit>;
            fn sub(self, other: MeasurePoint<Number, Unit>) -> Self::Output {
                Self::Output::new(self.value - other.value)
            }
        }

        // weighted_midpoint(MeasurePoint, MeasurePoint, weight) -> MeasurePoint
        pub fn weighted_midpoint<Number: ArithmeticOps, Unit: MeasurementUnit>(
            p1: MeasurePoint<Number, Unit>, p2: MeasurePoint<Number, Unit>, weight2: Number) -> MeasurePoint<Number, Unit>
        {
            MeasurePoint::<Number, Unit>::new(
                p1.value * (Number::ONE - weight2) + p2.value * weight2)
        }

        // midpoint(MeasurePoint, MeasurePoint) -> MeasurePoint
        pub fn midpoint<Number: ArithmeticOps, Unit: MeasurementUnit>(
            p1: MeasurePoint<Number, Unit>, p2: MeasurePoint<Number, Unit>) -> MeasurePoint<Number, Unit>
        {
            MeasurePoint::<Number, Unit>::new(
                (p1.value + p2.value) / (Number::ONE + Number::ONE))
        }

        // barycentric_combination(int, point1[], Num[]) -> point1
        pub fn barycentric_combination<Number: ArithmeticOps, Unit: MeasurementUnit>(
            points: &[MeasurePoint<Number, Unit>], weights: &[Number]) -> MeasurePoint<Number, Unit>
        {
            MeasurePoint::<Number, Unit>::new(
                points.iter().zip(weights).map(|(p, &w)| p.value * w).sum()
            )
        }

        impl<Number: ArithmeticOps, Unit> PartialEq<MeasurePoint<Number, Unit>> for MeasurePoint<Number, Unit> {
            fn eq(&self, other: &MeasurePoint<Number, Unit>) -> bool {
                self.value == other.value
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialOrd<MeasurePoint<Number, Unit>> for MeasurePoint<Number, Unit> {
            fn partial_cmp(&self, other: &MeasurePoint<Number, Unit>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for MeasurePoint<Number, Unit> {
            fn clone(&self) -> Self {
                MeasurePoint::<Number, Unit> {
                    value: self.value,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for MeasurePoint<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display
            for MeasurePoint<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{}", self.value, Unit::SUFFIX)
            }
        }
    };
}
