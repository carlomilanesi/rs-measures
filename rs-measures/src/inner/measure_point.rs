#[macro_export]
macro_rules! inner_define_measure_point {
    {} => {
        pub struct MeasurePoint<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            pub const fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint<DestUnit, Number> {
                MeasurePoint::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint<Unit, DestNumber> {
                MeasurePoint::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint<Unit, DestNumber> {
                MeasurePoint::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        impl<Unit, Number> Default for MeasurePoint<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            // It returns the origin.
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        // MeasurePoint + Measure -> MeasurePoint
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // MeasurePoint += Measure
        impl<Unit, Number: ArithmeticOps> AddAssign<Measure<Unit, Number>> for MeasurePoint<Unit, Number> {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // MeasurePoint - Measure -> MeasurePoint
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // MeasurePoint -= Measure
        impl<Unit, Number: ArithmeticOps> SubAssign<Measure<Unit, Number>> for MeasurePoint<Unit, Number> {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // MeasurePoint - MeasurePoint -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint<Unit, Number>>
            for MeasurePoint<Unit, Number>
        {
            type Output = Measure<Unit, Number>;
            fn sub(self, other: MeasurePoint<Unit, Number>) -> Self::Output {
                Self::Output::new(self.value - other.value)
            }
        }

        // weighted_midpoint(MeasurePoint, MeasurePoint, weight) -> MeasurePoint
        pub fn weighted_midpoint<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint<Unit, Number>,
            p2: MeasurePoint<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint<Unit, Number> {
            MeasurePoint::<Unit, Number>::new(p1.value * weight1 + p2.value * (Number::ONE - weight1))
        }

        // midpoint(MeasurePoint, MeasurePoint) -> MeasurePoint
        pub fn midpoint<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint<Unit, Number>,
            p2: MeasurePoint<Unit, Number>,
        ) -> MeasurePoint<Unit, Number> {
            MeasurePoint::<Unit, Number>::new((p1.value + p2.value) * Number::HALF)
        }

        // barycentric_combination([MeasurePoint], [Number]) -> MeasurePoint
        pub fn barycentric_combination<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint<Unit, Number> {
            MeasurePoint::<Unit, Number>::new(points.iter().zip(weights).map(|(p, &w)| p.value * w).sum())
        }

        impl<Unit, Number: ArithmeticOps> PartialEq<MeasurePoint<Unit, Number>>
            for MeasurePoint<Unit, Number>
        {
            fn eq(&self, other: &MeasurePoint<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<MeasurePoint<Unit, Number>>
            for MeasurePoint<Unit, Number>
        {
            fn partial_cmp(&self, other: &MeasurePoint<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for MeasurePoint<Unit, Number> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit, Number: ArithmeticOps> Copy for MeasurePoint<Unit, Number> {}

        // format!("{}", MeasurePoint)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint<Unit, Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", MeasurePoint)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for MeasurePoint<Unit, Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at ")?;
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
