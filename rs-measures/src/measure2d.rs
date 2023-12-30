#[macro_export]
macro_rules! define_measure_2d {
    {} => {
        rs_measures::define_measure_1d!();
        use rs_measures::traits::CrossProduct;

        // Measure 2d

        pub struct Measure2d<Unit, Number = f64> {
            pub x: Number,
            pub y: Number,
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Measure2d<Unit, Number> {
            /// measure 2d :: new(number, number) -> measure 2d
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            /// measure 2d .x() -> measure
            pub fn x(self) -> Measure<Unit, Number> { Measure::<Unit, Number>::new(self.x) }

            /// measure 2d .x() -> measure
            pub fn y(self) -> Measure<Unit, Number> { Measure::<Unit, Number>::new(self.y) }

            /// measure 2d .convert() -> measure 2d
            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure2d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure2d::<DestUnit, Number> {
                    x: self.x * factor,
                    y: self.y * factor,
                    phantom: PhantomData,
                }
            }

            /// measure 2d .lossy_into() -> measure 2d
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    phantom: PhantomData,
                }
            }

            /// measure 2d .lossy_into() -> measure 2d
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure2d<Unit, DestNumber> {
                Measure2d::<Unit, DestNumber> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    phantom: PhantomData,
                }
            }

            /// measure 2d .squared_norm() -> number
            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y
            }

            /// measure 2d .normalized() -> number
            pub fn normalized(self) -> Self {
                let k = Number::ONE / self.squared_norm().sqrt();
                Self::new(self.x * k, self.y * k)
            }

            /// Measure2d::from_direction(AnglePoint) -> Measure2d
            pub fn from_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(direction: MeasurePoint<AngleUnit, Number>) -> Self {
                let (y, x) = direction.convert::<Radian>().value.sin_cos();
                Self::new(x, y)
            }

            /// Measure2d.signed_direction() -> SignedDirection
            pub fn signed_direction<AngleUnit: MeasurementUnit<Property = Angle>>(self) -> SignedDirection<AngleUnit, Number> {
                SignedDirection::<Radian, Number>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
            }

            /// Measure2d.unsigned_direction() -> UnsignedDirection
            pub fn unsigned_direction<AngleUnit: MeasurementUnit<Property = Angle>>(self) -> UnsignedDirection<AngleUnit, Number> {
                UnsignedDirection::<Radian, Number>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
            }
        }

        // -Measure2d -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Neg for Measure2d<Unit, Number> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y)
            }
        }

        // Measure2d + Measure2d -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Add<Measure2d<Unit, Number>>
            for Measure2d<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // Measure2d += Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> AddAssign<Measure2d<Unit, Number>> for Measure2d<Unit, Number> {
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // Measure2d - Measure2d -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<Measure2d<Unit, Number>>
            for Measure2d<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // Measure2d -= Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> SubAssign<Measure2d<Unit, Number>> for Measure2d<Unit, Number> {
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        // Measure2d * Number -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Mul<Number> for Measure2d<Unit, Number> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n)
            }
        }

        // Measure2d *= Number
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> MulAssign<Number> for Measure2d<Unit, Number> {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
            }
        }

        // f64 * Measure2d -> Measure2d
        impl<Unit: VectorMeasurementUnit> Mul<Measure2d<Unit, f64>> for f64 {
            type Output = Measure2d<Unit, f64>;
            fn mul(self, other: Measure2d<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y)
            }
        }

        // f32 * Measure2d -> Measure2d
        impl<Unit: VectorMeasurementUnit> Mul<Measure2d<Unit, f32>> for f32 {
            type Output = Measure2d<Unit, f32>;
            fn mul(self, other: Measure2d<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y)
            }
        }

        // Measure2d / Number -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Div<Number> for Measure2d<Unit, Number> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n)
            }
        }

        // Measure2d /= Number
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> DivAssign<Number> for Measure2d<Unit, Number> {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
            }
        }

        // Measure2d == Measure2d -> bool
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> PartialEq<Measure2d<Unit, Number>> for Measure2d<Unit, Number> {
            fn eq(&self, other: &Measure2d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        // Measure2d.clone() -> Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Clone for Measure2d<Unit, Number> {
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure2d = Measure2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Copy for Measure2d<Unit, Number> { }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> fmt::Display for Measure2d<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }

        // Measure point 2d

        pub struct MeasurePoint2d<Unit, Number = f64> {
            pub x: Number,
            pub y: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> MeasurePoint2d<Unit, Number> {
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            pub fn x(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.x) }

            pub fn y(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.y) }

            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
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
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Add<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure point += measure
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> AddAssign<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        {
            fn add_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure point - measure
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure point -= measure
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> SubAssign<Measure2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number>
        {
            fn sub_assign(&mut self, other: Measure2d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        // measure point 2d - measure point 2d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint2d<Unit, Number>>
            for MeasurePoint2d<Unit, Number> {
            type Output = Measure2d<Unit, Number>;
            fn sub(self, other: MeasurePoint2d<Unit, Number>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y)
            }
        }

        /// weighted_midpoint_2d(measure point 2d, measure point 2d, weight) -> measure point 2d
        pub fn weighted_midpoint_2d<Unit: VectorMeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>, p2: MeasurePoint2d<Unit, Number>, weight1: Number) -> MeasurePoint2d<Unit, Number>
        {
            let weight2 = Number::ONE - weight1;
            MeasurePoint2d::<Unit, Number>::new(
                p1.x * weight1 + p2.x * weight2,
                p1.y * weight1 + p2.y * weight2,
            )
        }

        /// midpoint_2d(measure point 2d, measure point 2d) -> measure point 2d
        pub fn midpoint_2d<Unit: VectorMeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint2d<Unit, Number>, p2: MeasurePoint2d<Unit, Number>) -> MeasurePoint2d<Unit, Number>
        {
            MeasurePoint2d::<Unit, Number>::new(
                (p1.x + p2.x) * Number::HALF,
                (p1.y + p2.y) * Number::HALF,
            )
        }

        /// barycentric_combination_2d(array of 2d measure points, array of weights) -> 2d measure point
        pub fn barycentric_combination_2d<Unit: VectorMeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint2d<Unit, Number>], weights: &[Number]) -> MeasurePoint2d<Unit, Number>
        {
            MeasurePoint2d::<Unit, Number>::new(
                points.iter().zip(weights).map(|(p, &w)| p.x * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.y * w).sum(),
            )
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> PartialEq<MeasurePoint2d<Unit, Number>> for MeasurePoint2d<Unit, Number> {
            fn eq(&self, other: &MeasurePoint2d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Clone for MeasurePoint2d<Unit, Number> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Copy for MeasurePoint2d<Unit, Number> { }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint2d<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }

        // Linear map 2d

        pub struct LinearMap2d<Number: ArithmeticOps> {
            c: [[Number; 2]; 2],
        }

        impl<Number: ArithmeticOps> LinearMap2d<Number> {
            pub fn new(coefficients: [[Number; 2]; 2]) -> Self {
                Self {
                    c: coefficients,
                }
            }

            // No translations

            //// Rotations

            // Rotation by an angle measure.
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>>(angle: Measure<AngleUnit, Number>) -> Self {
                Self::rotation_by_radians(angle.convert::<Radian>().value)
            }

            pub fn rotation_at_right() -> Self {
                Self { c: [[Number::ZERO, Number::ONE], [-Number::ONE, Number::ZERO]] }
            }

            pub fn rotation_at_left() -> Self {
                Self { c: [[Number::ZERO, -Number::ONE], [Number::ONE, Number::ZERO]] }
            }

            //// Projections

            // Projection onto a line identified by a measure point angle.
            pub fn projection_by_point_angle<Unit: AngleMeasurementUnit<Property = Angle>>(angle: MeasurePoint<Unit, Number>) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a signed direction.
            pub fn projection_by_signed_direction<Unit: AngleMeasurementUnit<Property = Angle>>(direction: SignedDirection<Unit, Number>) -> Self {
                Self::projection_by_radians(direction.convert::<Radian>().value)
            }


            // Projection onto a line identified by an unsigned direction.
            pub fn projection_by_unsigned_direction<Unit: AngleMeasurementUnit<Property = Angle>>(angle: UnsignedDirection<Unit, Number>) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a unit plane vector.
            // Precondition: unit_v.squared_norm().value == 1
            pub fn projection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::projection_by_cos_sin(v.x, v.y)
            }

            //// Reflections

            // Reflection over a line identified by a point angle.
            pub fn reflection_by_point_angle<AngleUnit: AngleMeasurementUnit<Property = Angle>>(angle: MeasurePoint<AngleUnit, Number>) -> Self {
                Self::reflection_by_radians(angle.convert::<Radian>().value)
            }

            // Reflection over a line identified by a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(direction: SignedDirection<AngleUnit, Number>) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(direction: UnsignedDirection<AngleUnit, Number>) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by a unit plane vector.
            // Precondition: v.squared_norm() == 1
            pub fn reflection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Unit, Number>) -> Self {
                Self::reflection_by_cos_sin(v.x, v.y)
            }

            //// Scaling by two factors.

            pub fn scaling(kx: Number, ky: Number) -> Self {
                Self { c: [[kx, Number::ZERO], [Number::ZERO, ky]] }
            }

            //// Inversion

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE / (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]);
                Self { c: [
                    [
                        self.c[1][1] * inv_determinant,
                        self.c[0][1] * -inv_determinant,
                    ],
                    [
                        self.c[1][0] * -inv_determinant,
                        self.c[0][0] * inv_determinant,
                    ],
                ]}
            }

            // Composition of two plane linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(
                &self, other: &LinearMap2d<Number>) -> Self {
                Self { c: [
                    [
                        other.c[0][0] * self.c[0][0] +
                        other.c[0][1] * self.c[1][0],
                        other.c[0][0] * self.c[0][1] +
                        other.c[0][1] * self.c[1][1],
                    ],
                    [
                        other.c[1][0] * self.c[0][0] +
                        other.c[1][1] * self.c[1][0],
                        other.c[1][0] * self.c[0][1] +
                        other.c[1][1] * self.c[1][1],
                    ],
                ]}
            }

            pub fn apply_to<Unit: VectorMeasurementUnit>(
                &self, m: Measure2d<Unit, Number>) -> Measure2d<Unit, Number> {
                    Measure2d::<Unit, Number>::new(
                        self.c[0][0] * m.x + self.c[0][1] * m.y,
                        self.c[1][0] * m.x + self.c[1][1] * m.y,
                    )
            }

            fn rotation_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self { c: [
                    [ cos_a, -sin_a ],
                    [ sin_a, cos_a ],
                ]}
            }

            fn projection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                Self { c: [
                    [ cos_a * cos_a, cos_a * sin_a ],
                    [ sin_a * cos_a, sin_a * sin_a ],
                ]}
            }

            fn projection_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self::projection_by_cos_sin(cos_a, sin_a)
            }

            fn reflection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                let one = Number::ONE;
                let two = Number::ONE + Number::ONE;
                Self { c: [
                    [ two * cos_a * cos_a - one, two * cos_a * sin_a ],
                    [ two * cos_a * sin_a, two * sin_a * sin_a - one ],
                ]}
            }

            fn reflection_by_radians(radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::reflection_by_cos_sin(cos_a, sin_a)
            }
        }

        // It receives a matrix of numbers, and a column index,
        // processes the numbers of the specified column,
        // and returns an array of the lined up formatted numbers.
        fn format_column<const RowCount: usize, const ColumnCount: usize, Number: ArithmeticOps>(
            matrix: &[[Number; ColumnCount]; RowCount],
            column_index: usize,
        ) -> [String; RowCount] {
            const EMPTY_STRING: String = String::new();
            let mut padded_cells = [EMPTY_STRING; RowCount];
            let mut max_dot_pos = 0;
            let mut max_fractional_len = 0;
            let mut formatted_cells = [EMPTY_STRING; RowCount];
            let mut dot_positions = [0; RowCount];
            for row_index in 0..RowCount {
                formatted_cells[row_index] = format!("{}", matrix[row_index][column_index]);
                dot_positions[row_index] = if let Some(pos) =
                    formatted_cells[row_index].bytes().position(|c| c == b'.') {
                        pos
                    } else {
                        formatted_cells[row_index].len()
                    };
                max_dot_pos = core::cmp::max(max_dot_pos, dot_positions[row_index]);
                max_fractional_len = core::cmp::max(
                    max_fractional_len,
                    formatted_cells[row_index].len() - dot_positions[row_index],
                );
            }
            for row_index in 0..RowCount {
                padded_cells[row_index] = format!("{}{}{}",
                    " ".repeat(max_dot_pos - dot_positions[row_index]),
                    formatted_cells[row_index],
                    " ".repeat(max_fractional_len + dot_positions[row_index] - formatted_cells[row_index].len()),
                );
            }
            padded_cells
        }

        fn format_matrix<const RowCount: usize, const ColumnCount: usize, Number: ArithmeticOps>(
            matrix: &[[Number; ColumnCount]; RowCount],
            unit_suffix: &str,
        ) -> String {
            const EMPTY_STRING: String = String::new();
            let mut rows = [EMPTY_STRING; RowCount];
            for column_index in 0..ColumnCount {
                let column = format_column::<RowCount, ColumnCount, Number>(matrix, column_index);
                for row_index in 0..RowCount {
                    if column_index == 0 { rows[row_index] += "["; }
                    rows[row_index] += &column[row_index];
                    rows[row_index] += if column_index < ColumnCount - 1 { " " } else { "]" };
                    if row_index == 0 && column_index == ColumnCount - 1 { rows[row_index] += unit_suffix; }
                }
            }
            rows.join("\n")
        }

        impl<Number: ArithmeticOps> fmt::Display for LinearMap2d<Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", format_matrix::<2, 2, Number>(&self.c, ""))
            }
        }

        // Affine map 2d

        pub struct AffineMap2d<Unit: MeasurementUnit, Number: ArithmeticOps> {
            c: [[Number; 3]; 2],
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> AffineMap2d<Unit, Number> {
            pub fn new(coefficients: [[Number; 3]; 2]) -> Self {
                Self {
                    c: coefficients,
                    phantom: PhantomData,
                }
            }

            // Unit conversion.
            pub fn convert<DestUnit: VectorMeasurementUnit>(&self) -> AffineMap2d<DestUnit, Number> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                AffineMap2d::<DestUnit, Number>::new([
                    [
                        self.c[0][0],
                        self.c[0][1],
                        self.c[0][2] * factor,
                    ],
                    [
                        self.c[1][0],
                        self.c[1][1],
                        self.c[1][2] * factor,
                    ],
                ])
            }

            // Translation.
            pub fn translation(v: Measure2d<Unit, Number>) -> Self {
                Self::new([
                    [
                        Number::ONE,
                        Number::ZERO,
                        v.x,
                    ],
                    [
                        Number::ZERO,
                        Number::ONE,
                        v.y,
                    ],
                ])
            }

            // Rotation about a point by an angle measure.
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                angle: Measure<AngleUnit, Number>) -> Self {
                    Self::rotation_by_radians(
                    fixed_point.x, fixed_point.y,
                    angle.convert::<Radian>().value)
            }

            pub fn rotation_at_right(
                fixed_point: MeasurePoint2d<Unit, Number>) -> Self {
                    Self::right_rotation_by_sin(
                    fixed_point.x, fixed_point.y,
                    -Number::ONE)
            }

            pub fn rotation_at_left(
                fixed_point: MeasurePoint2d<Unit, Number>) -> Self {
                    Self::right_rotation_by_sin(
                        fixed_point.x, fixed_point.y,
                    Number::ONE)
            }

            // Projections

            // Projection onto a line identified by a fixed point
            // and a point angle.
            pub fn projection_by_point_angle<AngleUnit: MeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: MeasurePoint<AngleUnit, Number>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a signed direction.
            pub fn projection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: SignedDirection<AngleUnit, Number>) -> Self
            {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and an unsigned direction.
            pub fn projection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: UnsignedDirection<AngleUnit, Number>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn projection_by_unit_vector(
                fixed_point: MeasurePoint2d<Unit, Number>,
                uv: Measure2d<Unit, Number>) -> Self {
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    uv.x, uv.y)
            }

            // Reflections

            // Reflection over a line identified by a fixed point
            // and a point angle.
            pub fn reflection_by_point_angle<AngleUnit: MeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: MeasurePoint<AngleUnit, Number>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: SignedDirection<AngleUnit, Number>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Unit, Number>,
                direction: UnsignedDirection<AngleUnit, Number>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn reflection_by_unit_vector(
                fixed_point: MeasurePoint2d<Unit, Number>,
                uv: Measure2d<Unit, Number>) -> Self {
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    uv.x, uv.y)
            }

            // Scaling by two factors from a fixed point.
            pub fn scaling(fixed_point: MeasurePoint2d<Unit, Number>,
                kx: Number, ky: Number) -> Self {
                Self::new([
                    [ kx, Number::ZERO, fixed_point.x * (Number::ONE - kx) ],
                    [ Number::ZERO, ky,  fixed_point.y * (Number::ONE - ky) ],
                ])
            }

            pub fn inverted(&self) -> Self {
                let inv_determinant = Number::ONE / (self.c[0][0] * self.c[1][1] - self.c[0][1] * self.c[1][0]);
                Self::new([
                    [
                        self.c[1][1] * inv_determinant,
                        self.c[0][1] * -inv_determinant,
                        (self.c[0][1] * self.c[1][2] -
                            self.c[0][2] * self.c[1][1]) * inv_determinant,
                    ],
                    [
                        self.c[1][0] * -inv_determinant,
                        self.c[0][0] * inv_determinant,
                        (self.c[0][2] * self.c[1][0] -
                            self.c[0][0] * self.c[1][2]) * inv_determinant,
                    ]
                ])
            }

            // Composition of two plane affine transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn combined_with(&self,
                other: &AffineMap2d<Unit, Number>) -> Self {
                    Self::new([
                    [
                        other.c[0][0] * self.c[0][0] +
                        other.c[0][1] * self.c[1][0],
                        other.c[0][0] * self.c[0][1] +
                        other.c[0][1] * self.c[1][1],
                        other.c[0][0] * self.c[0][2] +
                        other.c[0][1] * self.c[1][2] +
                        other.c[0][2],
                    ],
                    [
                        other.c[1][0] * self.c[0][0] +
                        other.c[1][1] * self.c[1][0],
                        other.c[1][0] * self.c[0][1] +
                        other.c[1][1] * self.c[1][1],
                        other.c[1][0] * self.c[0][2] +
                        other.c[1][1] * self.c[1][2] +
                        other.c[1][2],
                    ]
                ])
            }

            pub fn apply_to(
                &self, m: MeasurePoint2d<Unit, Number>) -> MeasurePoint2d<Unit, Number> {
                    MeasurePoint2d::<Unit, Number>::new(
                        self.c[0][0] * m.x + self.c[0][1] * m.y + self.c[0][2],
                        self.c[1][0] * m.x + self.c[1][1] * m.y + self.c[1][2],
                    )
                }

            fn rotation_by_radians(fp_x: Number, fp_y: Number, radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::new([
                    [ cos_a, -sin_a, fp_x - cos_a * fp_x + sin_a * fp_y ],
                    [ sin_a, cos_a, fp_y - sin_a * fp_x - cos_a * fp_y ],
                ])
            }

            fn right_rotation_by_sin(fp_x: Number, fp_y: Number, sine: Number) -> Self {
                Self::new([
                    [ Number::ZERO, -sine, fp_x + sine * fp_y ],
                    [ sine, Number::ZERO, fp_y - sine * fp_x ],
                ])
            }

            fn projection_by_cos_sin(fp_x: Number, fp_y: Number, cos_a: Number, sin_a: Number) -> Self {
                let cc = cos_a * cos_a;
                let cs = cos_a * sin_a;
                let ss = sin_a * sin_a;
                let sxmcy = sin_a * fp_x - cos_a * fp_y;
                Self::new([
                    [ cc, cs, sin_a * sxmcy ],
                    [ cs, ss, -cos_a * sxmcy ],
                ])
            }

            fn reflection_by_cos_sin(fp_x: Number, fp_y: Number, cos_a: Number, sin_a: Number) -> Self {
                let c2ms2 = cos_a * cos_a - sin_a * sin_a;
                let two = Number::ONE + Number::ONE;
                let cs_bis = two * cos_a * sin_a;
                let sxmcy_bis = two * (sin_a * fp_x - cos_a * fp_y);
                Self::new([
                    [ c2ms2, cs_bis, sin_a * sxmcy_bis ],
                    [ cs_bis, -c2ms2, -cos_a * sxmcy_bis],
                ])
            }
        }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> fmt::Display for AffineMap2d<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", format_matrix::<2, 3, Number>(&self.c, Unit::SUFFIX))
            }
        }
    };
}
