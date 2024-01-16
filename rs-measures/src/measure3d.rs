#[macro_export]
macro_rules! define_measure_3d {
    {} => {
        rs_measures::define_measure_2d!();

        // Measure3d

        pub struct Measure3d<Unit, Number: ArithmeticOps = f64> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            /// measure 3d :: new(number, number, number) -> measure 3d
            pub const fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }

            /// measure 3d .x() -> measure
            pub const fn x(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.x)
            }

            /// measure 3d .y() -> measure
            pub const fn y(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.y)
            }

            /// measure 3d .z() -> measure
            pub const fn z(self) -> Measure<Unit, Number> {
                Measure::<Unit, Number>::new(self.z)
            }

            /// measure 3d .convert() -> measure 3d
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure3d::<DestUnit, Number> {
                    x: self.x * factor,
                    y: self.y * factor,
                    z: self.z * factor,
                    phantom: PhantomData,
                }
            }

            /// measure 3d .lossless_into() -> measure 3d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure3d<Unit, DestNumber> {
                Measure3d::<Unit, DestNumber> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    z: DestNumber::from(self.z),
                    phantom: PhantomData,
                }
            }

            /// measure 3d .lossy_into() -> measure 3d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure3d<Unit, DestNumber> {
                Measure3d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    z: DestNumber::lossy_from(self.z),
                    phantom: PhantomData,
                }
            }

            /// measure 3d .squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            /// measure 3d .normalized() -> number
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new(self.x * k, self.y * k, self.z * k)
            }
        }

        // -Measure3d -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Neg for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }

        // Measure3d * Number -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Mul<Number> for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n, self.z * n)
            }
        }

        // Measure3d *= Number
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> MulAssign<Number> for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
                self.z *= n;
            }
        }

        // f64 * Measure3d -> Measure3d
        impl<Unit: MeasurementUnit> Mul<Measure3d<Unit, f64>> for f64
        where
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, f64>;
            fn mul(self, other: Measure3d<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // f32 * Measure3d -> Measure3d
        impl<Unit: MeasurementUnit> Mul<Measure3d<Unit, f32>> for f32
        where
            Unit::Property: VectorProperty,
        {
            type Output = Measure3d<Unit, f32>;
            fn mul(self, other: Measure3d<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // Measure3d / Number -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Div<Number> for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                let factor = Number::ONE / n;
                Self::new(self.x * factor, self.y * factor, self.z * factor)
            }
        }

        // Measure3d /= Number
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> DivAssign<Number> for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
                self.z /= n;
            }
        }

        // Measure3d + Measure3d -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // Measure3d += Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> AddAssign<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // Measure3d - Measure3d -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // Measure3d -= Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> SubAssign<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        // MeasurePoint3d

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

        // Measure3d == Measure3d -> bool
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> PartialEq<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn eq(&self, other: &Measure3d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        // Measure3d.clone() -> Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Clone for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure3d = Measure3d
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Copy for Measure3d<Unit, Number> where
            Unit::Property: VectorProperty
        {
        }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for Measure3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
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

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
        }

        // Linear map 3d

        pub struct LinearMap3d<Number: ArithmeticOps> {
            c: [[Number; 3]; 3],
        }

        impl<Number: ArithmeticOps> LinearMap3d<Number> {
            pub const fn new(coefficients: [[Number; 3]; 3]) -> Self {
                Self { c: coefficients }
            }

            // No translations

            // Rotations

            // Rotation by an angle measure around a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>, AxisUnit: MeasurementUnit>(
                angle: Measure<AngleUnit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self
            where
                AxisUnit::Property: VectorProperty,
            {
                Self::rotation_by_radians_around_unit_vector(
                    angle.convert::<Radian>().value,
                    unit_vector.x,
                    unit_vector.y,
                    unit_vector.z,
                )
            }

            // Projections

            // Projection onto a line identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_line<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                Self {
                    c: [
                        [
                            unit_vector.x * unit_vector.x,
                            unit_vector.y * unit_vector.x,
                            unit_vector.z * unit_vector.x,
                        ],
                        [
                            unit_vector.x * unit_vector.y,
                            unit_vector.y * unit_vector.y,
                            unit_vector.z * unit_vector.y,
                        ],
                        [
                            unit_vector.x * unit_vector.z,
                            unit_vector.y * unit_vector.z,
                            unit_vector.z * unit_vector.z,
                        ],
                    ],
                }
            }

            // Projection onto a plane whose normal is identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_plane<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                Self {
                    c: [
                        [
                            Number::ONE - unit_vector.x * unit_vector.x,
                            -unit_vector.y * unit_vector.x,
                            -unit_vector.z * unit_vector.x,
                        ],
                        [
                            -unit_vector.x * unit_vector.y,
                            Number::ONE - unit_vector.y * unit_vector.y,
                            -unit_vector.z * unit_vector.y,
                        ],
                        [
                            -unit_vector.x * unit_vector.z,
                            -unit_vector.y * unit_vector.z,
                            Number::ONE - unit_vector.z * unit_vector.z,
                        ],
                    ],
                }
            }

            // Reflections

            // Reflection over a line identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_line<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                let two = Number::ONE + Number::ONE;
                Self {
                    c: [
                        [
                            two * unit_vector.x * unit_vector.x - Number::ONE,
                            two * unit_vector.y * unit_vector.x,
                            two * unit_vector.z * unit_vector.x,
                        ],
                        [
                            two * unit_vector.x * unit_vector.y,
                            two * unit_vector.y * unit_vector.y - Number::ONE,
                            two * unit_vector.z * unit_vector.y,
                        ],
                        [
                            two * unit_vector.x * unit_vector.z,
                            two * unit_vector.y * unit_vector.z,
                            two * unit_vector.z * unit_vector.z - Number::ONE,
                        ],
                    ],
                }
            }

            // Reflection over a plane whose normal is identified by a unit vector.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_plane<Unit: MeasurementUnit>(
                unit_vector: Measure3d<Unit, Number>,
            ) -> Self {
                let minus_two = -(Number::ONE + Number::ONE);
                Self {
                    c: [
                        [
                            minus_two * unit_vector.x * unit_vector.x + Number::ONE,
                            minus_two * unit_vector.y * unit_vector.x,
                            minus_two * unit_vector.z * unit_vector.x,
                        ],
                        [
                            minus_two * unit_vector.x * unit_vector.y,
                            minus_two * unit_vector.y * unit_vector.y + Number::ONE,
                            minus_two * unit_vector.z * unit_vector.y,
                        ],
                        [
                            minus_two * unit_vector.x * unit_vector.z,
                            minus_two * unit_vector.y * unit_vector.z,
                            minus_two * unit_vector.z * unit_vector.z + Number::ONE,
                        ],
                    ],
                }
            }

            // Scaling by three factors.

            pub fn scaling(kx: Number, ky: Number, kz: Number) -> Self {
                Self {
                    c: [
                        [kx, Number::ZERO, Number::ZERO],
                        [Number::ZERO, ky, Number::ZERO],
                        [Number::ZERO, Number::ZERO, kz],
                    ],
                }
            }

            // Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE
                    / (self.c[0][0] * (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1])
                        - self.c[0][1] * (self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0])
                        + self.c[0][2] * (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]));
                Self {
                    c: [
                        [
                            (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1]) * inv_determinant,
                            -(self.c[0][1] * self.c[2][2] - self.c[0][2] * self.c[2][1]) * inv_determinant,
                            (self.c[0][1] * self.c[1][2] - self.c[0][2] * self.c[1][1]) * inv_determinant,
                        ],
                        [
                            -(self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0]) * inv_determinant,
                            (self.c[0][0] * self.c[2][2] - self.c[0][2] * self.c[2][0]) * inv_determinant,
                            -(self.c[0][0] * self.c[1][2] - self.c[0][2] * self.c[1][0]) * inv_determinant,
                        ],
                        [
                            (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]) * inv_determinant,
                            -(self.c[0][0] * self.c[2][1] - self.c[0][1] * self.c[2][0]) * inv_determinant,
                            (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]) * inv_determinant,
                        ],
                    ],
                }
            }

            // Composition of spacial linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &LinearMap3d<Number>) -> Self {
                Self {
                    c: [
                        [
                            other.c[0][0] * self.c[0][0]
                                + other.c[0][1] * self.c[1][0]
                                + other.c[0][2] * self.c[2][0],
                            other.c[0][0] * self.c[0][1]
                                + other.c[0][1] * self.c[1][1]
                                + other.c[0][2] * self.c[2][1],
                            other.c[0][0] * self.c[0][2]
                                + other.c[0][1] * self.c[1][2]
                                + other.c[0][2] * self.c[2][2],
                        ],
                        [
                            other.c[1][0] * self.c[0][0]
                                + other.c[1][1] * self.c[1][0]
                                + other.c[1][2] * self.c[2][0],
                            other.c[1][0] * self.c[0][1]
                                + other.c[1][1] * self.c[1][1]
                                + other.c[1][2] * self.c[2][1],
                            other.c[1][0] * self.c[0][2]
                                + other.c[1][1] * self.c[1][2]
                                + other.c[1][2] * self.c[2][2],
                        ],
                        [
                            other.c[2][0] * self.c[0][0]
                                + other.c[2][1] * self.c[1][0]
                                + other.c[2][2] * self.c[2][0],
                            other.c[2][0] * self.c[0][1]
                                + other.c[2][1] * self.c[1][1]
                                + other.c[2][2] * self.c[2][1],
                            other.c[2][0] * self.c[0][2]
                                + other.c[2][1] * self.c[1][2]
                                + other.c[2][2] * self.c[2][2],
                        ],
                    ],
                }
            }

            pub fn apply_to<Unit: MeasurementUnit>(
                &self,
                m: Measure3d<Unit, Number>,
            ) -> Measure3d<Unit, Number>
            where
                Unit::Property: VectorProperty,
            {
                Measure3d::<Unit, Number>::new(
                    self.c[0][0] * m.x + self.c[0][1] * m.y + self.c[0][2] * m.z,
                    self.c[1][0] * m.x + self.c[1][1] * m.y + self.c[1][2] * m.z,
                    self.c[2][0] * m.x + self.c[2][1] * m.y + self.c[2][2] * m.z,
                )
            }

            fn rotation_by_radians_around_unit_vector(
                a: Number,
                ux: Number,
                uy: Number,
                uz: Number,
            ) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                let one_minus_cos_a = Number::ONE - cos_a;
                Self {
                    c: [
                        [
                            cos_a + ux * ux * one_minus_cos_a,
                            ux * uy * one_minus_cos_a - uz * sin_a,
                            ux * uz * one_minus_cos_a + uy * sin_a,
                        ],
                        [
                            uy * ux * one_minus_cos_a + uz * sin_a,
                            cos_a + uy * uy * one_minus_cos_a,
                            uy * uz * one_minus_cos_a - ux * sin_a,
                        ],
                        [
                            uz * ux * one_minus_cos_a - uy * sin_a,
                            uz * uy * one_minus_cos_a + ux * sin_a,
                            cos_a + uz * uz * one_minus_cos_a,
                        ],
                    ],
                }
            }
        }

        impl<Number: ArithmeticOps> fmt::Display for LinearMap3d<Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", format_matrix::<3, 3, Number>(&self.c, ""))
            }
        }

        // Affine map 3d

        pub struct AffineMap3d<Unit: MeasurementUnit, Number: ArithmeticOps> {
            c: [[Number; 4]; 3],
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> AffineMap3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            pub const fn new(coefficients: [[Number; 4]; 3]) -> Self {
                Self {
                    c: coefficients,
                    phantom: PhantomData,
                }
            }

            // Unit conversion.
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> AffineMap3d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                AffineMap3d::<DestUnit, Number>::new([
                    [
                        self.c[0][0],
                        self.c[0][1],
                        self.c[0][2],
                        self.c[0][3] * factor,
                    ],
                    [
                        self.c[1][0],
                        self.c[1][1],
                        self.c[1][2],
                        self.c[1][3] * factor,
                    ],
                    [
                        self.c[2][0],
                        self.c[2][1],
                        self.c[2][2],
                        self.c[2][3] * factor,
                    ],
                ])
            }

            // Translation

            pub fn translation(v: Measure3d<Unit, Number>) -> Self {
                Self::new([
                    [Number::ONE, Number::ZERO, Number::ZERO, v.x],
                    [Number::ZERO, Number::ONE, Number::ZERO, v.y],
                    [Number::ZERO, Number::ZERO, Number::ONE, v.z],
                ])
            }

            // Rotations

            // Rotation by an angle measure around a unit vector
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>, AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
                angle: Measure<AngleUnit, Number>,
            ) -> Self
            where
                AxisUnit::Property: VectorProperty,
            {
                let fpx = fixed_point.x;
                let fpy = fixed_point.y;
                let fpz = fixed_point.z;
                let ux = unit_vector.x;
                let uy = unit_vector.y;
                let uz = unit_vector.z;
                let a = angle.convert::<Radian>().value;
                let (sin_a, cos_a) = a.sin_cos();
                let one_minus_cos_a = Number::ONE - cos_a;
                let c00 = cos_a + ux * ux * one_minus_cos_a;
                let c01 = ux * uy * one_minus_cos_a - uz * sin_a;
                let c02 = ux * uz * one_minus_cos_a + uy * sin_a;
                let c10 = uy * ux * one_minus_cos_a + uz * sin_a;
                let c11 = cos_a + uy * uy * one_minus_cos_a;
                let c12 = uy * uz * one_minus_cos_a - ux * sin_a;
                let c20 = uz * ux * one_minus_cos_a - uy * sin_a;
                let c21 = uz * uy * one_minus_cos_a + ux * sin_a;
                let c22 = cos_a + uz * uz * one_minus_cos_a;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Projections

            // Projection onto a line identified by a unit vector
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_line<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let fpx = fixed_point.x;
                let fpy = fixed_point.y;
                let fpz = fixed_point.z;
                let ux = unit_vector.x;
                let uy = unit_vector.y;
                let uz = unit_vector.z;
                Self::new([
                    [
                        ux * ux,
                        uy * ux,
                        uz * ux,
                        fpx - fpx * ux * ux - fpy * uy * ux - fpz * uz * ux,
                    ],
                    [
                        ux * uy,
                        uy * uy,
                        uz * uy,
                        fpy - fpx * ux * uy - fpy * uy * uy - fpz * uz * uy,
                    ],
                    [
                        ux * uz,
                        uy * uz,
                        uz * uz,
                        fpz - fpx * ux * uz - fpy * uy * uz - fpz * uz * uz,
                    ],
                ])
            }

            // Projection onto a plane whose normal is identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn projection_onto_plane<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let fpx = fixed_point.x;
                let fpy = fixed_point.y;
                let fpz = fixed_point.z;
                let ux = unit_vector.x;
                let uy = unit_vector.y;
                let uz = unit_vector.z;
                let c00 = Number::ONE - unit_vector.x * unit_vector.x;
                let c01 = -unit_vector.y * unit_vector.x;
                let c02 = -unit_vector.z * unit_vector.x;
                let c10 = -unit_vector.x * unit_vector.y;
                let c11 = Number::ONE - unit_vector.y * unit_vector.y;
                let c12 = -unit_vector.z * unit_vector.y;
                let c20 = -unit_vector.x * unit_vector.z;
                let c21 = -unit_vector.y * unit_vector.z;
                let c22 = Number::ONE - unit_vector.z * unit_vector.z;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Reflections

            // Reflection over a line identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_line<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let two = Number::ONE + Number::ONE;
                let fpx = fixed_point.x;
                let fpy = fixed_point.y;
                let fpz = fixed_point.z;
                let ux = unit_vector.x;
                let uy = unit_vector.y;
                let uz = unit_vector.z;
                let c00 = two * ux * ux - Number::ONE;
                let c01 = two * uy * ux;
                let c02 = two * uz * ux;
                let c10 = two * ux * uy;
                let c11 = two * uy * uy - Number::ONE;
                let c12 = two * uz * uy;
                let c20 = two * ux * uz;
                let c21 = two * uy * uz;
                let c22 = two * uz * uz - Number::ONE;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Reflection over a plane whose normal is identified by a unit vector.
            // applied to a point.
            // Precondition: unit_vector.squared_norm().value == 1
            pub fn reflection_over_plane<AxisUnit: MeasurementUnit>(
                fixed_point: MeasurePoint3d<Unit, Number>,
                unit_vector: Measure3d<AxisUnit, Number>,
            ) -> Self {
                let minus_two = -(Number::ONE + Number::ONE);
                let fpx = fixed_point.x;
                let fpy = fixed_point.y;
                let fpz = fixed_point.z;
                let ux = unit_vector.x;
                let uy = unit_vector.y;
                let uz = unit_vector.z;
                let c00 = minus_two * unit_vector.x * unit_vector.x + Number::ONE;
                let c01 = minus_two * unit_vector.y * unit_vector.x;
                let c02 = minus_two * unit_vector.z * unit_vector.x;
                let c10 = minus_two * unit_vector.x * unit_vector.y;
                let c11 = minus_two * unit_vector.y * unit_vector.y + Number::ONE;
                let c12 = minus_two * unit_vector.z * unit_vector.y;
                let c20 = minus_two * unit_vector.x * unit_vector.z;
                let c21 = minus_two * unit_vector.y * unit_vector.z;
                let c22 = minus_two * unit_vector.z * unit_vector.z + Number::ONE;
                Self::new([
                    [c00, c01, c02, fpx - fpx * c00 - fpy * c01 - fpz * c02],
                    [c10, c11, c12, fpy - fpx * c10 - fpy * c11 - fpz * c12],
                    [c20, c21, c22, fpz - fpx * c20 - fpy * c21 - fpz * c22],
                ])
            }

            // Scaling by three factors from a point.

            pub fn scaling(
                fixed_point: MeasurePoint3d<Unit, Number>,
                kx: Number,
                ky: Number,
                kz: Number,
            ) -> Self {
                Self::new([
                    [
                        kx,
                        Number::ZERO,
                        Number::ZERO,
                        fixed_point.x * (Number::ONE - kx),
                    ],
                    [
                        Number::ZERO,
                        ky,
                        Number::ZERO,
                        fixed_point.y * (Number::ONE - ky),
                    ],
                    [
                        Number::ZERO,
                        Number::ZERO,
                        kz,
                        fixed_point.z * (Number::ONE - kz),
                    ],
                ])
            }

            // Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE
                    / (self.c[0][0] * (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1])
                        - self.c[0][1] * (self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0])
                        + self.c[0][2] * (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]));
                let c00 = (self.c[1][1] * self.c[2][2] - self.c[1][2] * self.c[2][1]) * inv_determinant;
                let c01 = -(self.c[0][1] * self.c[2][2] - self.c[0][2] * self.c[2][1]) * inv_determinant;
                let c02 = (self.c[0][1] * self.c[1][2] - self.c[0][2] * self.c[1][1]) * inv_determinant;
                let c10 = -(self.c[1][0] * self.c[2][2] - self.c[1][2] * self.c[2][0]) * inv_determinant;
                let c11 = (self.c[0][0] * self.c[2][2] - self.c[0][2] * self.c[2][0]) * inv_determinant;
                let c12 = -(self.c[0][0] * self.c[1][2] - self.c[0][2] * self.c[1][0]) * inv_determinant;
                let c20 = (self.c[1][0] * self.c[2][1] - self.c[1][1] * self.c[2][0]) * inv_determinant;
                let c21 = -(self.c[0][0] * self.c[2][1] - self.c[0][1] * self.c[2][0]) * inv_determinant;
                let c22 = (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]) * inv_determinant;
                Self::new([
                    [
                        c00,
                        c01,
                        c02,
                        -c00 * self.c[0][3] - c01 * self.c[1][3] - c02 * self.c[2][3],
                    ],
                    [
                        c10,
                        c11,
                        c12,
                        -c10 * self.c[0][3] - c11 * self.c[1][3] - c12 * self.c[2][3],
                    ],
                    [
                        c20,
                        c21,
                        c22,
                        -c20 * self.c[0][3] - c21 * self.c[1][3] - c22 * self.c[2][3],
                    ],
                ])
            }

            // Composition of spacial affine transformations.
            // To apply the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &AffineMap3d<Unit, Number>) -> Self {
                Self::new([
                    [
                        other.c[0][0] * self.c[0][0]
                            + other.c[0][1] * self.c[1][0]
                            + other.c[0][2] * self.c[2][0],
                        other.c[0][0] * self.c[0][1]
                            + other.c[0][1] * self.c[1][1]
                            + other.c[0][2] * self.c[2][1],
                        other.c[0][0] * self.c[0][2]
                            + other.c[0][1] * self.c[1][2]
                            + other.c[0][2] * self.c[2][2],
                        other.c[0][0] * self.c[0][3]
                            + other.c[0][1] * self.c[1][3]
                            + other.c[0][2] * self.c[2][3]
                            + other.c[0][3],
                    ],
                    [
                        other.c[1][0] * self.c[0][0]
                            + other.c[1][1] * self.c[1][0]
                            + other.c[1][2] * self.c[2][0],
                        other.c[1][0] * self.c[0][1]
                            + other.c[1][1] * self.c[1][1]
                            + other.c[1][2] * self.c[2][1],
                        other.c[1][0] * self.c[0][2]
                            + other.c[1][1] * self.c[1][2]
                            + other.c[1][2] * self.c[2][2],
                        other.c[1][0] * self.c[0][3]
                            + other.c[1][1] * self.c[1][3]
                            + other.c[1][2] * self.c[2][3]
                            + other.c[1][3],
                    ],
                    [
                        other.c[2][0] * self.c[0][0]
                            + other.c[2][1] * self.c[1][0]
                            + other.c[2][2] * self.c[2][0],
                        other.c[2][0] * self.c[0][1]
                            + other.c[2][1] * self.c[1][1]
                            + other.c[2][2] * self.c[2][1],
                        other.c[2][0] * self.c[0][2]
                            + other.c[2][1] * self.c[1][2]
                            + other.c[2][2] * self.c[2][2],
                        other.c[2][0] * self.c[0][3]
                            + other.c[2][1] * self.c[1][3]
                            + other.c[2][2] * self.c[2][3]
                            + other.c[2][3],
                    ],
                ])
            }

            pub fn apply_to(&self, m: MeasurePoint3d<Unit, Number>) -> MeasurePoint3d<Unit, Number> {
                MeasurePoint3d::<Unit, Number>::new(
                    self.c[0][0] * m.x + self.c[0][1] * m.y + self.c[0][2] * m.z + self.c[0][3],
                    self.c[1][0] * m.x + self.c[1][1] * m.y + self.c[1][2] * m.z + self.c[1][3],
                    self.c[2][0] * m.x + self.c[2][1] * m.y + self.c[2][2] * m.z + self.c[2][3],
                )
            }
        }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for AffineMap3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    format_matrix::<3, 4, Number>(&self.c, Unit::SUFFIX)
                )
            }
        }
    }
}
