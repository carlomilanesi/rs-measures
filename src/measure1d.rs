#[macro_export]
macro_rules! define_measure1d {
    {} => {
        use rs_measures::traits::{ArithmeticOps, LossyFrom, MeasurementUnit};
        use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
        use std::fmt;
        use std::marker::PhantomData;

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
            pub fn convert<DestUnit: MeasurementUnit<Quantity = Unit::Quantity>>(
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

        // measure * number
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Mul<Number> for Measure<Number, Unit> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        // measure *= number
        impl<Number: ArithmeticOps, Unit> MulAssign<Number> for Measure<Number, Unit> {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        // measure / number
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Div<Number> for Measure<Number, Unit> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.value / n)
            }
        }

        // measure /= number
        impl<Number: ArithmeticOps, Unit> DivAssign<Number> for Measure<Number, Unit> {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        // measure + measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // measure += measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // measure - measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // measure -= measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for Measure<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }

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
            pub fn convert<DestUnit: MeasurementUnit<Quantity = Unit::Quantity>>(
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

        // measure point + measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // measure point += measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // measure point - measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // measure point -= measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for MeasurePoint<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }
    };
}
