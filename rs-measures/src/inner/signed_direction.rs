#[macro_export]
macro_rules! inner_define_signed_direction {
    {} => {
        pub struct SignedDirection<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps>
            SignedDirection<Unit, Number>
        {
            /// Returns the only value that in the current Unit represents `x`, and
            /// is between 0 included and one cycle excluded.
            fn normalize(x: Number) -> Number {
                let one_cycle = Number::from_f64(Unit::CYCLE_FRACTION);
                let half_cycle = one_cycle * Number::HALF;
                let x2 = (x + half_cycle) % one_cycle;
                if x2 >= Number::ZERO {
                    x2 - half_cycle
                } else {
                    x2 + half_cycle
                }
            }

            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                Self::new(m.value)
            }

            pub const fn to_measure_point(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.value)
            }

            pub fn to_unsigned_direction(self) -> UnsignedDirection<Unit, Number> {
                UnsignedDirection::<Unit, Number>::new(self.value)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> SignedDirection<DestUnit, Number> {
                SignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> SignedDirection<Unit, DestNumber> {
                SignedDirection::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> SignedDirection<Unit, DestNumber> {
                SignedDirection::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // Signed direction + angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for SignedDirection<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Signed direction += angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps>
            AddAssign<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self + other;
            }
        }

        // Signed direction - angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for SignedDirection<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Signed direction -= angle measure
        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: AngleMeasurementUnit<Property = Angle>,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                *self = *self - other;
            }
        }

        // Signed direction - Signed direction
        impl<AngleUnit: AngleMeasurementUnit, Number: ArithmeticOps> Sub<SignedDirection<AngleUnit, Number>>
            for SignedDirection<AngleUnit, Number>
        where
            AngleUnit: AngleMeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Measure<AngleUnit, Number>;
            fn sub(self, other: SignedDirection<AngleUnit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let cycle = Number::from_f64(AngleUnit::CYCLE_FRACTION);
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

        impl<Unit, Number> PartialEq<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn eq(&self, other: &SignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number> PartialOrd<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn partial_cmp(&self, other: &SignedDirection<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number> Clone for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit, Number: ArithmeticOps> Copy for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
        }

        // format!("{}", SignedDirection)
        impl<Unit, Number> fmt::Display for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in -180°..180°)")
            }
        }

        // format!("{:?}", SignedDirection)
        impl<Unit, Number> fmt::Debug for SignedDirection<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)?;
                formatter.write_str(" (in -180°..180°)")
            }
        }
    };
}
