#[macro_export]
macro_rules! inner_define_measure_point_2d {
    {} => {
        pub struct MeasurePoint2d<Unit, Number = f64> {
            pub x: Number,
            pub y: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            pub const fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            pub const fn x(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.x)
            }

            pub const fn y(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.y)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint2d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint2d::<DestUnit, Number> {
                    x: self.x * factor + offset,
                    y: self.y * factor + offset,
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint2d<Unit, DestNumber> {
                MeasurePoint2d::<Unit, DestNumber> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint2d<Unit, DestNumber> {
                MeasurePoint2d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    phantom: PhantomData,
                }
            }
        }

        // measure point + measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure point += measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> AddAssign<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure point - measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure point -= measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> SubAssign<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        // measure point 2d - measure point 2d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Measure2d<Unit, Number>;
            fn sub(self, other: MeasurePoint2d<Unit, Number>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y)
            }
        }

        /// weighted_midpoint_2d(measure point 2d, measure point 2d, weight) -> measure point 2d
        pub fn weighted_midpoint_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint2d::<Unit, Number>::new(
                p1.x * weight1 + p2.x * weight2,
                p1.y * weight1 + p2.y * weight2,
            )
        }

        /// midpoint_2d(measure point 2d, measure point 2d) -> measure point 2d
        pub fn midpoint_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>,
            p2: MeasurePoint2d<Unit, Number>,
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint2d::<Unit, Number>::new((p1.x + p2.x) * Number::HALF, (p1.y + p2.y) * Number::HALF)
        }

        /// barycentric_combination_2d(array of 2d measure points, array of weights) -> 2d measure point
        pub fn barycentric_combination_2d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint2d<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint2d::<Unit, Number>::new(
                points.iter().zip(weights).map(|(p, &w)| p.x * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.y * w).sum(),
            )
        }

        // MeasurePoint2d == MeasurePoint2d -> bool
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> PartialEq<MeasurePoint2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &MeasurePoint2d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        // MeasurePoint2d.clone() -> MeasurePoint2d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Clone for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // MeasurePoint2d = MeasurePoint2d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Copy for MeasurePoint2d<Unit, Number> where
            Unit::Property: VectorProperty
        {
        }

        // format!("{}", MeasurePoint2d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", MeasurePoint2d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for MeasurePoint2d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
