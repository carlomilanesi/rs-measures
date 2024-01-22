#[macro_export]
macro_rules! inner_define_linear_map_2d {
    {} => {
        pub struct LinearMap2d<Number: ArithmeticOps> {
            c: [[Number; 2]; 2],
        }

        impl<Number: ArithmeticOps> LinearMap2d<Number> {
            pub const fn new(coefficients: [[Number; 2]; 2]) -> Self {
                Self { c: coefficients }
            }

            // No translations

            //// Rotations

            // Rotation by an angle measure.
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>>(
                angle: Measure<AngleUnit, Number>,
            ) -> Self {
                Self::rotation_by_radians(angle.convert::<Radian>().value)
            }

            pub fn rotation_at_right() -> Self {
                Self {
                    c: [[Number::ZERO, Number::ONE], [-Number::ONE, Number::ZERO]],
                }
            }

            pub fn rotation_at_left() -> Self {
                Self {
                    c: [[Number::ZERO, -Number::ONE], [Number::ONE, Number::ZERO]],
                }
            }

            //// Projections

            // Projection onto a line identified by a measure point angle.
            pub fn projection_by_point_angle<Unit: AngleMeasurementUnit<Property = Angle>>(
                angle: MeasurePoint<Unit, Number>,
            ) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a signed direction.
            pub fn projection_by_signed_direction<Unit: AngleMeasurementUnit<Property = Angle>>(
                direction: SignedDirection<Unit, Number>,
            ) -> Self {
                Self::projection_by_radians(direction.convert::<Radian>().value)
            }

            // Projection onto a line identified by an unsigned direction.
            pub fn projection_by_unsigned_direction<Unit: AngleMeasurementUnit<Property = Angle>>(
                angle: UnsignedDirection<Unit, Number>,
            ) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a unit plane vector.
            // Precondition: unit_v.squared_norm().value == 1
            pub fn projection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::projection_by_cos_sin(v.x, v.y)
            }

            //// Reflections

            // Reflection over a line identified by a point angle.
            pub fn reflection_by_point_angle<AngleUnit: AngleMeasurementUnit<Property = Angle>>(
                angle: MeasurePoint<AngleUnit, Number>,
            ) -> Self {
                Self::reflection_by_radians(angle.convert::<Radian>().value)
            }

            // Reflection over a line identified by a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(
                direction: SignedDirection<AngleUnit, Number>,
            ) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(
                direction: UnsignedDirection<AngleUnit, Number>,
            ) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by a unit plane vector.
            // Precondition: v.squared_norm() == 1
            pub fn reflection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::reflection_by_cos_sin(v.x, v.y)
            }

            //// Scaling by two factors.

            pub fn scaling(kx: Number, ky: Number) -> Self {
                Self {
                    c: [[kx, Number::ZERO], [Number::ZERO, ky]],
                }
            }

            //// Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant =
                    Number::ONE / (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]);
                Self {
                    c: [
                        [
                            self.c[1][1] * inv_determinant,
                            self.c[0][1] * -inv_determinant,
                        ],
                        [
                            self.c[1][0] * -inv_determinant,
                            self.c[0][0] * inv_determinant,
                        ],
                    ],
                }
            }

            // Composition of two plane linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self, other: &LinearMap2d<Number>) -> Self {
                Self {
                    c: [
                        [
                            other.c[0][0] * self.c[0][0] + other.c[0][1] * self.c[1][0],
                            other.c[0][0] * self.c[0][1] + other.c[0][1] * self.c[1][1],
                        ],
                        [
                            other.c[1][0] * self.c[0][0] + other.c[1][1] * self.c[1][0],
                            other.c[1][0] * self.c[0][1] + other.c[1][1] * self.c[1][1],
                        ],
                    ],
                }
            }

            pub fn apply_to<Unit: MeasurementUnit>(
                &self,
                m: Measure2d<Unit, Number>,
            ) -> Measure2d<Unit, Number>
            where
                Unit::Property: VectorProperty,
            {
                Measure2d::<Unit, Number>::new(
                    self.c[0][0] * m.x + self.c[0][1] * m.y,
                    self.c[1][0] * m.x + self.c[1][1] * m.y,
                )
            }

            fn rotation_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self {
                    c: [[cos_a, -sin_a], [sin_a, cos_a]],
                }
            }

            fn projection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                Self {
                    c: [
                        [cos_a * cos_a, cos_a * sin_a],
                        [sin_a * cos_a, sin_a * sin_a],
                    ],
                }
            }

            fn projection_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self::projection_by_cos_sin(cos_a, sin_a)
            }

            fn reflection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                let one = Number::ONE;
                let two = Number::ONE + Number::ONE;
                Self {
                    c: [
                        [two * cos_a * cos_a - one, two * cos_a * sin_a],
                        [two * cos_a * sin_a, two * sin_a * sin_a - one],
                    ],
                }
            }

            fn reflection_by_radians(radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::reflection_by_cos_sin(cos_a, sin_a)
            }
        }

        // format!("{}", LinearMap2d)
        impl<Number: ArithmeticOps> fmt::Display for LinearMap2d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<2, 2, Number>(&self.c, "")
                )
            }
        }

        // format!("{:?}", LinearMap2d)
        impl<Number: ArithmeticOps> fmt::Debug for LinearMap2d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<2, 2, Number>(&self.c, "")
                )
            }
        }
    };
}
