#[macro_export]
macro_rules! inner_define_unsigned_direction {
    {} => {
        pub struct UnsignedDirection<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps>
            UnsignedDirection<Unit, Number>
        {
            /// Returns the only value that in the current Unit represents `x` and
            /// is between minus half cycle included and plus half cycle excluded.
            fn normalize(x: Number) -> Number {
                let one_cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let x2 = x % one_cycle;
                if x2 >= Number::ZERO {
                    x2
                } else {
                    x2 + one_cycle
                }
            }

            /// UnsignedDirection::new(Number) -> UnsignedDirection
            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            /// UnsignedDirection::from_measure_point(MeasurePoint) -> UnsignedDirection
            pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                Self::new(m.value)
            }

            pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.value)
            }

            pub fn to_signed_direction(self) -> SignedDirection<Unit, Number> {
                SignedDirection::<Unit, Number>::new(self.value)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> UnsignedDirection<DestUnit, Number> {
                UnsignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> UnsignedDirection<Unit, DestNumber> {
                UnsignedDirection::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> UnsignedDirection<Unit, DestNumber> {
                UnsignedDirection::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // Unsigned direction + angle measure -> Unsigned direction
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Unsigned direction += angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps>
            AddAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self + other;
            }
        }

        // Unsigned direction - angle measure -> Unsigned direction
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Unsigned direction -= angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps>
            SubAssign<Measure<Unit, Number>> for UnsignedDirection<Unit, Number>
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self - other;
            }
        }

        // unsigned direction - unsigned direction -> angle measure
        impl<Unit: AngleMeasurementUnit, Number: ArithmeticOps> Sub<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            type Output = Measure<Unit, Number>;
            fn sub(self, other: UnsignedDirection<Unit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let half_cycle = cycle * Number::HALF;
                Self::Output::new(if diff > half_cycle {
                    diff - cycle
                } else if diff < -half_cycle {
                    diff + cycle
                } else {
                    diff
                })
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialEq<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            fn eq(&self, other: &UnsignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number>
        {
            fn partial_cmp(&self, other: &UnsignedDirection<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for UnsignedDirection<Unit, Number> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit, Number: ArithmeticOps> Copy for UnsignedDirection<Unit, Number> {}

        // format!("{}", UnsignedDirection)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display
            for UnsignedDirection<Unit, Number>
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }

        // format!("{:?}", UnsignedDirection)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for UnsignedDirection<Unit, Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in 0°..360°)")
            }
        }
    };
}
