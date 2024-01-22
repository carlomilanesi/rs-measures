#[macro_export]
macro_rules! inner_define_affine_map_3d {
    {} => {
        pub struct AffineMap3d<Unit: MeasurementUnit, Number: ArithmeticOps = f64> {
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

        // format!("{}", AffineMap3d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for AffineMap3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<3, 4, Number>(&self.c, Unit::SUFFIX)
                )
            }
        }

        // format!("{:?}", AffineMap3d)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for AffineMap3d<Unit, Number>
        where
            Unit::Property: VectorProperty,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    rs_measures::matrix_utils::format_matrix::<3, 4, Number>(&self.c, Unit::SUFFIX)
                )
            }
        }
    };
}
