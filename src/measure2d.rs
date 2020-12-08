//use rs_measures::angle::Radian;
#[macro_export]
macro_rules! define_measure2d {
    {} => {

        // Measure2d

        #[derive(Debug)]
        pub struct Measure2d<Number, Unit> {
            pub x: Number,
            pub y: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Measure2d<Number, Unit> {
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            pub fn x(self) -> Measure<Number, Unit> { Measure::<Number, Unit>::new(self.x) }

            pub fn y(self) -> Measure<Number, Unit> { Measure::<Number, Unit>::new(self.y) }

            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure2d<Number, DestUnit> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure2d::<Number, DestUnit> {
                    x: self.x * factor,
                    y: self.y * factor,
                    phantom: PhantomData,
                }
            }

            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure2d<DestNumber, Unit> {
                Measure2d::<DestNumber, Unit> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    phantom: PhantomData,
                }
            }

            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure2d<DestNumber, Unit> {
                Measure2d::<DestNumber, Unit> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    phantom: PhantomData,
                }
            }

            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y
            }

            pub fn normalized(self) -> Self {
                let k = self.squared_norm().sqrt();
                Self::new(self.x / k, self.y / k)
            }

            pub fn signed_direction<AngleUnit: MeasurementUnit<Property = Angle>>(self) -> SignedDirection<Number, AngleUnit> {
                SignedDirection::<Number, Radian>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
            }

            pub fn unsigned_direction<AngleUnit: MeasurementUnit<Property = Angle>>(self) -> UnsignedDirection<Number, AngleUnit> {
                UnsignedDirection::<Number, Radian>::new(self.y.atan2(self.x)).convert::<AngleUnit>()
            }
        }

        // -measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Neg for Measure2d<Number, Unit> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y)
            }
        }
        // measure * number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Mul<Number> for Measure2d<Number, Unit> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n)
            }
        }

        // measure *= number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> MulAssign<Number> for Measure2d<Number, Unit> {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
            }
        }

        // measure / number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Div<Number> for Measure2d<Number, Unit> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n)
            }
        }

        // measure /= number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> DivAssign<Number> for Measure2d<Number, Unit> {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
            }
        }

        // measure + measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Add<Measure2d<Number, Unit>>
            for Measure2d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure += measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> AddAssign<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn add_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure - measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<Measure2d<Number, Unit>>
            for Measure2d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure -= measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> SubAssign<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn sub_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> PartialEq<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn eq(&self, other: &Measure2d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Clone for Measure2d<Number, Unit> {
            fn clone(&self) -> Self {
                Measure2d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Copy for Measure2d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> fmt::Display for Measure2d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }

        // MeasurePoint2d

        #[derive(Debug)]
        pub struct MeasurePoint2d<Number, Unit> {
            pub x: Number,
            pub y: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> MeasurePoint2d<Number, Unit> {
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }

            pub fn x(self) -> MeasurePoint<Number, Unit> { MeasurePoint::<Number, Unit>::new(self.x) }

            pub fn y(self) -> MeasurePoint<Number, Unit> { MeasurePoint::<Number, Unit>::new(self.y) }

            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint2d<Number, DestUnit> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint2d::<Number, DestUnit> {
                    x: self.x * factor + offset,
                    y: self.y * factor + offset,
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint2d<DestNumber, Unit> {
                MeasurePoint2d::<DestNumber, Unit> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint2d<DestNumber, Unit> {
                MeasurePoint2d::<DestNumber, Unit> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    phantom: PhantomData,
                }
            }
        }

        // measure point + measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Add<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure point += measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> AddAssign<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            fn add_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure point - measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure point -= measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> SubAssign<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            fn sub_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        // measure point - measure point
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<MeasurePoint2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit> {
            type Output = Measure2d<Number, Unit>;
            fn sub(self, other: MeasurePoint2d<Number, Unit>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y)
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> MeasurePoint2d<Number, Unit> {
            // weighted_midpoint(MeasurePoint2d, MeasurePoint2d, weight2) -> MeasurePoint2d
            pub fn weighted_midpoint(self,
                other: Self, other_weight: Number) -> Self
            {
                let self_weigth = Number::ONE - other_weight;
                Self::new(
                    self.x * self_weigth + other.x * other_weight,
                    self.y * self_weigth + other.y * other_weight,
                )
            }

            // midpoint(MeasurePoint2d, MeasurePoint2d) -> MeasurePoint2d
            pub fn midpoint(self, other: Self) -> Self
            {
                let two = Number::ONE + Number::ONE;
                Self::new(
                    (self.x + other.x) / two,
                    (self.y + other.y) / two,
                )
            }
        }

        // barycentric_combination_2d([MeasurePoint2d], [Number]) -> MeasurePoint2d
        pub fn barycentric_combination_2d<Number: ArithmeticOps, Unit: VectorMeasurementUnit>(
            points: &[MeasurePoint2d<Number, Unit>], weights: &[Number]) -> MeasurePoint2d<Number, Unit>
        {
            MeasurePoint2d::<Number, Unit>::new(
                points.iter().zip(weights).map(|(p, &w)| p.x * w).sum(),
                points.iter().zip(weights).map(|(p, &w)| p.y * w).sum(),
            )
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> PartialEq<MeasurePoint2d<Number, Unit>> for MeasurePoint2d<Number, Unit> {
            fn eq(&self, other: &MeasurePoint2d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Clone for MeasurePoint2d<Number, Unit> {
            fn clone(&self) -> Self {
                MeasurePoint2d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Copy for MeasurePoint2d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> fmt::Display for MeasurePoint2d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }

        // Unsigned direction

        #[derive(Debug)]
        pub struct UnsignedDirection<Number, Unit> {
            value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> UnsignedDirection<Number, Unit> {
            // Returns the only value that in the current Unit represents `x`, and
            // is between minus half turn included and plus half turn excluded.
            fn normalize(x: Number) -> Number {
                let one_turn = Number::from_f64(Unit::TURN_FRACTION);
                let x2 = x % one_turn;
                if x2 >= Number::ZERO { x2 } else { x2 + one_turn }
            }

            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            pub fn to_measure_point(self) -> MeasurePoint<Number, Unit> { MeasurePoint::<Number, Unit>::new(self.value) }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> UnsignedDirection<Number, DestUnit> {
                UnsignedDirection::<Number, DestUnit> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> UnsignedDirection<DestNumber, Unit> {
                UnsignedDirection::<DestNumber, Unit> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> UnsignedDirection<DestNumber, Unit> {
                UnsignedDirection::<DestNumber, Unit> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // Unsigned direction + angle measure
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> Add<Measure<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Unsigned direction += angle measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // Unsigned direction - angle measure
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> Sub<Measure<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Unsigned direction -= angle measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }

        // Unsigned direction - Unsigned direction
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit> Sub<UnsignedDirection<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            type Output = Measure<Number, Unit>;
            fn sub(self, other: UnsignedDirection<Number, Unit>) -> Self::Output {
                let mut diff = self.value - other.value;
                let turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = turn / (Number::ONE + Number::ONE);
                Self::Output::new(if diff > half_turn { diff - turn } else if diff < -half_turn { diff + turn } else { diff })
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialEq<UnsignedDirection<Number, Unit>> for UnsignedDirection<Number, Unit> {
            fn eq(&self, other: &UnsignedDirection<Number, Unit>) -> bool {
                self.value == other.value
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialOrd<UnsignedDirection<Number, Unit>> for UnsignedDirection<Number, Unit> {
            fn partial_cmp(&self, other: &UnsignedDirection<Number, Unit>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for UnsignedDirection<Number, Unit> {
            fn clone(&self) -> Self {
                UnsignedDirection::<Number, Unit> {
                    value: self.value,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for UnsignedDirection<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display
            for UnsignedDirection<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{} (in 0°-360°)", self.value, Unit::SUFFIX)
            }
        }

        // Signed direction

        #[derive(Debug)]
        pub struct SignedDirection<Number, Unit> {
            value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> SignedDirection<Number, Unit> {
            // Returns the only value that in the current Unit represents `x`, and
            // is between 0 included and one turn excluded.
            fn normalize(x: Number) -> Number {
                let one_turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = one_turn / (Number::ONE + Number::ONE);
                let x2 = (x + half_turn) % one_turn;
                if x2 >= Number::ZERO { x2 - half_turn } else { x2 + half_turn }
            }

            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            pub fn to_measure_point(self) -> MeasurePoint<Number, Unit> { MeasurePoint::<Number, Unit>::new(self.value) }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> SignedDirection<Number, DestUnit> {
                SignedDirection::<Number, DestUnit> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> SignedDirection<DestNumber, Unit> {
                SignedDirection::<DestNumber, Unit> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> SignedDirection<DestNumber, Unit> {
                SignedDirection::<DestNumber, Unit> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // Signed direction + angle measure
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> Add<Measure<Number, Unit>>
            for SignedDirection<Number, Unit> {
            type Output = Self;
            fn add(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Signed direction += angle measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure<Number, Unit>>
            for SignedDirection<Number, Unit> {
            fn add_assign(&mut self, other: Measure<Number, Unit>) {
                self.value += other.value;
            }
        }

        // Signed direction - angle measure
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit<Property = Angle>> Sub<Measure<Number, Unit>>
            for SignedDirection<Number, Unit> {
            type Output = Self;
            fn sub(self, other: Measure<Number, Unit>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Signed direction -= angle measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure<Number, Unit>>
            for SignedDirection<Number, Unit> {
            fn sub_assign(&mut self, other: Measure<Number, Unit>) {
                self.value -= other.value;
            }
        }

        // Signed direction - Signed direction
        impl<Number: ArithmeticOps, Unit: AngleMeasurementUnit> Sub<SignedDirection<Number, Unit>>
            for SignedDirection<Number, Unit> {
            type Output = Measure<Number, Unit>;
            fn sub(self, other: SignedDirection<Number, Unit>) -> Self::Output {
                let mut diff = self.value - other.value;
                let turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = turn / (Number::ONE + Number::ONE);
                Self::Output::new(if diff > half_turn { diff - turn } else if diff < -half_turn { diff + turn } else { diff })
            }
        }


        impl<Number: ArithmeticOps, Unit> PartialEq<SignedDirection<Number, Unit>> for SignedDirection<Number, Unit> {
            fn eq(&self, other: &SignedDirection<Number, Unit>) -> bool {
                self.value == other.value
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialOrd<SignedDirection<Number, Unit>> for SignedDirection<Number, Unit> {
            fn partial_cmp(&self, other: &SignedDirection<Number, Unit>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for SignedDirection<Number, Unit> {
            fn clone(&self) -> Self {
                SignedDirection::<Number, Unit> {
                    value: self.value,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for SignedDirection<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display
            for SignedDirection<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{} (in -180°-180°)", self.value, Unit::SUFFIX)
            }
        }

        #[derive(Debug)]
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
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>>(angle: Measure<Number, AngleUnit>) -> Self {
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
            pub fn projection_by_angle_point<Unit: AngleMeasurementUnit<Property = Angle>>(angle: MeasurePoint<Number, Unit>) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a signed direction.
            pub fn projection_by_signed_direction<Unit: AngleMeasurementUnit<Property = Angle>>(direction: SignedDirection<Number, Unit>) -> Self {
                Self::projection_by_radians(direction.convert::<Radian>().value)
            }


            // Projection onto a line identified by an unsigned direction.
            pub fn projection_by_unsigned_direction<Unit: AngleMeasurementUnit<Property = Angle>>(angle: UnsignedDirection<Number, Unit>) -> Self {
                Self::projection_by_radians(angle.convert::<Radian>().value)
            }

            // Projection onto a line identified by a unit plane vector.
            // Precondition: unit_v.squared_norm().value == 1
            pub fn projection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Number, Unit>) -> Self {
                Self::projection_by_cos_sin(v.x, v.y)
            }

            //// Reflections

            // Reflection over a line identified by a point angle.
            pub fn reflection_by_angle_point<AngleUnit: AngleMeasurementUnit<Property = Angle>>(angle: MeasurePoint<Number, AngleUnit>) -> Self {
                Self::reflection_by_radians(angle.convert::<Radian>().value)
            }

            // Reflection over a line identified by a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(direction: SignedDirection<Number, AngleUnit>) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(direction: UnsignedDirection<Number, AngleUnit>) -> Self {
                Self::reflection_by_radians(direction.convert::<Radian>().value)
            }

            // Reflection over a line identified by a unit plane vector.
            // Precondition: v.squared_norm() == 1
            pub fn reflection_by_unit_vector<Unit: MeasurementUnit>(v: Measure2d<Number, Unit>) -> Self {
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
                    ]
                ]}
            }

            // Composition of two plane linear transformations.
            // Applying the resulting transformation is equivalent to apply first
            // `other` and then `self`.
            pub fn apply_after(
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
                &self, m: Measure2d<Number, Unit>) -> Measure2d<Number, Unit> {
                    Measure2d::<Number, Unit>::new(
                        self.c[0][0] * m.x + self.c[0][1] * m.y,
                        self.c[1][0] * m.x + self.c[1][1] * m.y,
                    )
            }

            fn rotation_by_radians(a: Number) -> Self {
                let (sin_a, cos_a) = a.sin_cos();
                Self { c: [
                    [
                        cos_a,
                        -sin_a,
                    ],
                    [
                        sin_a,
                        cos_a,
                    ]
                ]}
            }

            fn projection_by_cos_sin(cos_a: Number, sin_a: Number) -> Self {
                Self { c: [
                    [
                        cos_a * cos_a,
                        cos_a * sin_a,
                    ],
                    [
                        sin_a * cos_a,
                        sin_a * sin_a,
                    ]
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
                    [
                        two * cos_a * cos_a - one,
                        two * cos_a * sin_a,
                    ],
                    [
                        two * cos_a * sin_a,
                        two * sin_a * sin_a - one,
                    ]
                ]}
            }

            fn reflection_by_radians(radians: Number) -> Self {
                let (sin_a, cos_a) = radians.sin_cos();
                Self::reflection_by_cos_sin(cos_a, sin_a)
            }
        }

        // Affine_map2

        #[derive(Debug)]
        pub struct AffineMap2d<Number: ArithmeticOps, Unit: MeasurementUnit> {
            pub c: [[Number; 3]; 2],
            phantom: std::marker::PhantomData<Unit>,
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> AffineMap2d<Number, Unit> {
            pub fn new(coefficients: [[Number; 3]; 2]) -> Self {
                Self {
                    c: coefficients,
                    phantom: PhantomData,
                }
            }

            // Unit conversion.
            pub fn convert<DestUnit: VectorMeasurementUnit>(&self) -> AffineMap2d<Number, DestUnit> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                AffineMap2d::<Number, DestUnit>::new([
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
            pub fn translation(v: Measure2d<Number, Unit>) -> Self {
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
            pub fn rotation<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                angle: Measure<Number, AngleUnit>) -> Self {
                    Self::rotation_by_radians(
                    fixed_point.x, fixed_point.y,
                    angle.convert::<Radian>().value)
            }

            pub fn rotation_at_right(
                fixed_point: MeasurePoint2d<Number, Unit>) -> Self {
                    Self::right_rotation_by_sin(
                    fixed_point.x, fixed_point.y,
                    -Number::ONE)
            }

            pub fn rotation_at_left(
                fixed_point: MeasurePoint2d<Number, Unit>) -> Self {
                    Self::right_rotation_by_sin(
                        fixed_point.x, fixed_point.y,
                    Number::ONE)
            }

            // Projections

            // Projection onto a line identified by a fixed point
            // and a point angle.
            pub fn projection_by_point_angle<AngleUnit: MeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: MeasurePoint<Number, AngleUnit>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a signed direction.
            pub fn projection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: SignedDirection<Number, AngleUnit>) -> Self
            {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and an unsigned direction.
            pub fn projection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: UnsignedDirection<Number, AngleUnit>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Projection onto a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn projection_by_unit_vector(
                fixed_point: MeasurePoint2d<Number, Unit>,
                uv: Measure2d<Number, Unit>) -> Self {
                Self::projection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    uv.x, uv.y)
            }

            // Reflections

            // Reflection over a line identified by a fixed point
            // and a point angle.
            pub fn reflection_by_point_angle<AngleUnit: MeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: MeasurePoint<Number, AngleUnit>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a signed direction.
            pub fn reflection_by_signed_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: SignedDirection<Number, AngleUnit>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and an unsigned direction.
            pub fn reflection_by_unsigned_direction<AngleUnit: AngleMeasurementUnit<Property = Angle>>(fixed_point: MeasurePoint2d<Number, Unit>,
                direction: UnsignedDirection<Number, AngleUnit>) -> Self {
                let (sin_a, cos_a) = direction.convert::<Radian>().value.sin_cos();
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    cos_a, sin_a)
            }

            // Reflection over a line identified by a fixed point
            // and a unit plane vector.
            // Precondition: v.squared_norm().value == 1
            pub fn reflection_by_unit_vector(
                fixed_point: MeasurePoint2d<Number, Unit>,
                uv: Measure2d<Number, Unit>) -> Self {
                Self::reflection_by_cos_sin(
                    fixed_point.x, fixed_point.y,
                    uv.x, uv.y)
            }

            // Scaling by two factors from a fixed point.
            pub fn scaling(fixed_point: MeasurePoint2d<Number, Unit>,
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
            pub fn apply_after(&self,
                other: &AffineMap2d<Number, Unit>) -> Self {
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
                &self, m: MeasurePoint2d<Number, Unit>) -> MeasurePoint2d<Number, Unit> {
                    MeasurePoint2d::<Number, Unit>::new(
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
    };
}
