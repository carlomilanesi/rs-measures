#[macro_export]
macro_rules! inner_define_linear_map_3d {
    {} => {
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

        // format!("{}", LinearMap3d)
        impl<Number: ArithmeticOps> fmt::Display for LinearMap3d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<3, 3, Number>(&self.c, "")
                )
            }
        }

        // format!("{:?}", LinearMap3d)
        impl<Number: ArithmeticOps> fmt::Debug for LinearMap3d<Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<3, 3, Number>(&self.c, "")
                )
            }
        }
    };
}
