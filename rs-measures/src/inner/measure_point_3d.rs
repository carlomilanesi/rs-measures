#[macro_export]
macro_rules! inner_define_measure_point_3d {
    {} => {
        pub struct MeasurePoint3d<Unit, Number = f64> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            pub const fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }

            pub const fn x(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.x)
            }

            pub const fn y(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.y)
            }

            pub const fn z(self) -> MeasurePoint<Unit, Number> {
                MeasurePoint::<Unit, Number>::new(self.z)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint3d::<DestUnit, Number> {
                    x: self.x * factor + offset,
                    y: self.y * factor + offset,
                    z: self.z * factor + offset,
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint3d<Unit, DestNumber> {
                MeasurePoint3d::<Unit, DestNumber> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    z: DestNumber::from(self.z),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint3d<Unit, DestNumber> {
                MeasurePoint3d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    z: DestNumber::lossy_from(self.z),
                    phantom: PhantomData,
                }
            }
        }

        impl<Unit, Number> Default for MeasurePoint3d<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
            Unit::Property: VectorProperty,
        {
            // It returns the origin.
            fn default() -> Self {
                Self::new(Number::ZERO, Number::ZERO, Number::ZERO)
            }
        }

        // MeasurePoint3d + Measure3d -> MeasurePoint3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // MeasurePoint3d += Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> AddAssign<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // MeasurePoint3d - Measure3d -> MeasurePoint3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // MeasurePoint3d -= Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> SubAssign<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        /// measure point 3d - measure point 3d -> measure 3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, Number>;
            fn sub(self, other: MeasurePoint3d<Unit, Number>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        /// weighted_midpoint_3d(measure point 3d, measure point 3d, weight) -> measure point 3d
        pub fn weighted_midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint3d<Unit, Number>,
            p2: MeasurePoint3d<Unit, Number>,
            weight1: Number,
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint3d::<Unit, Number>::new(
                p1.x * weight1 + p2.x * weight2,
                p1.y * weight1 + p2.y * weight2,
                p1.z * weight1 + p2.z * weight2,
            )
        }

        /// midpoint_3d(measure point 3d, measure point 3d) -> measure point 3d
        pub fn midpoint_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint3d<Unit, Number>,
            p2: MeasurePoint3d<Unit, Number>,
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint3d::<Unit, Number>::new(
                (p1.x + p2.x) * Number::HALF,
                (p1.y + p2.y) * Number::HALF,
                (p1.z + p2.z) * Number::HALF,
            )
        }

        /// barycentric_combination_3d(array of 3d measure points, array of weights) -> 3d measure point
        pub fn barycentric_combination_3d<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint3d<Unit, Number>],
            weights: &[Number],
        ) -> MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            MeasurePoint3d::<Unit, Number>::new(
                points.iter().zip(weights).map(|(p, &w)| p.x * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.y * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.z * w).sum(),
            )
        }

        // MeasurePoint3d == MeasurePoint3d -> bool
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> PartialEq<MeasurePoint3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &MeasurePoint3d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        // MeasurePoint3d.clone() -> MeasurePoint3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Clone for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // MeasurePoint3d = MeasurePoint3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Copy for MeasurePoint3d<Unit, Number> where
            Unit::Property: VectorProperty
        {
        }

        // format!("{}", MeasurePoint3d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", MeasurePoint3d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("at (")?;
                fmt::Display::fmt(&self.x, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.y, formatter)?;
                formatter.write_str(", ")?;
                fmt::Display::fmt(&self.z, formatter)?;
                formatter.write_str(")")?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
