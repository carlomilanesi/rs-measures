#[macro_export]
macro_rules! define_measure_1d {
    {} => {
        use rs_measures::{angle::{Angle, Radian}, traits::{Sqrt, ArithmeticOps, LossyFrom, MeasurementUnit, AngleMeasurementUnit, VectorMeasurementUnit}};
        use core::ops::{Neg, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
        use std::fmt;
        use std::marker::PhantomData;

        // Measure

        pub struct Measure<Unit, Number: ArithmeticOps = f64> {
            value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Measure<Unit, Number> {
            /// Measure::new(Number) -> Measure
            pub fn new(value: Number) -> Self {
                Self {
                    value,
                    phantom: PhantomData,
                }
            }

            /// Measure.convert() -> Measure
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure<DestUnit, Number> {
                Measure::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }

            // Measure.lossless_into() -> Measure
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure<Unit, DestNumber> {
                Measure::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }

            // Measure.lossy_into() -> Measure
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure<Unit, DestNumber> {
                Measure::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }

            // Measure.squared_norm() -> Number
            pub fn squared_norm(self) -> Number {
                self.value * self.value
            }

            /// Measure.normalized() -> Number
            pub fn normalized(self) -> Self {
                Self::new(self.value.signum())
            }
        }

        // -Measure -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Neg for Measure<Unit, Number> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        // Measure + Measure -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for Measure<Unit, Number> {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Measure += Measure
        impl<Unit, Number: ArithmeticOps> AddAssign<Measure<Unit, Number>>
            for Measure<Unit, Number> {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // Measure - Measure -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for Measure<Unit, Number> {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Measure -= Measure
        impl<Unit, Number: ArithmeticOps> SubAssign<Measure<Unit, Number>>
            for Measure<Unit, Number> {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // Measure * Number -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Mul<Number> for Measure<Unit, Number> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        // Measure *= Number
        impl<Unit, Number: ArithmeticOps> MulAssign<Number> for Measure<Unit, Number> {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        // f64 * Measure -> Measure
        impl<Unit: MeasurementUnit> Mul<Measure<Unit, f64>> for f64 {
            type Output = Measure<Unit, f64>;
            fn mul(self, other: Measure<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // f32 * Measure -> Measure
        impl<Unit: MeasurementUnit> Mul<Measure<Unit, f32>> for f32 {
            type Output = Measure<Unit, f32>;
            fn mul(self, other: Measure<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.value)
            }
        }

        // Measure / Number -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Div<Number> for Measure<Unit, Number> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.value / n)
            }
        }

        // Measure /= Number
        impl<Unit, Number: ArithmeticOps> DivAssign<Number> for Measure<Unit, Number> {
            fn div_assign(&mut self, n: Number) {
                self.value /= n;
            }
        }

        // Measure / Measure -> Number
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Div<Measure<Unit, Number>> for Measure<Unit, Number> {
            type Output = Number;
            fn div(self, other: Measure<Unit, Number>) -> Self::Output {
                self.value / other.value
            }
        }

        // Measure == Measure -> bool
        impl<Unit, Number: ArithmeticOps> PartialEq<Measure<Unit, Number>> for Measure<Unit, Number> {
            fn eq(&self, other: &Measure<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        // Measure < Measure -> bool
        impl<Unit, Number: ArithmeticOps> PartialOrd<Measure<Unit, Number>> for Measure<Unit, Number> {
            fn partial_cmp(&self, other: &Measure<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        // Measure.clone() -> Measure
        impl<Unit, Number: ArithmeticOps> Clone for Measure<Unit, Number> {
            fn clone(&self) -> Self { *self }
        }

        // Measure = Measure
        impl<Unit, Number: ArithmeticOps> Copy for Measure<Unit, Number> { }

        // format!("{}", Measure)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for Measure<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.value, Unit::SUFFIX)
            }
        }

        // MeasurePoint

        pub struct MeasurePoint<Unit, Number = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> MeasurePoint<Unit, Number> {
            pub fn new(value: Number) -> Self {
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

        // MeasurePoint + Measure -> MeasurePoint
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number> {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // MeasurePoint += Measure
        impl<Unit, Number: ArithmeticOps> AddAssign<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number> {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // MeasurePoint - Measure -> MeasurePoint
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number> {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // MeasurePoint -= Measure
        impl<Unit, Number: ArithmeticOps> SubAssign<Measure<Unit, Number>>
            for MeasurePoint<Unit, Number> {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // MeasurePoint - MeasurePoint -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint<Unit, Number>>
            for MeasurePoint<Unit, Number> {
            type Output = Measure<Unit, Number>;
            fn sub(self, other: MeasurePoint<Unit, Number>) -> Self::Output {
                Self::Output::new(self.value - other.value)
            }
        }

        // weighted_midpoint(MeasurePoint, MeasurePoint, weight) -> MeasurePoint
        pub fn weighted_midpoint<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint<Unit, Number>, p2: MeasurePoint<Unit, Number>, weight2: Number) -> MeasurePoint<Unit, Number>
        {
            MeasurePoint::<Unit, Number>::new(
                p1.value * (Number::ONE - weight2) + p2.value * weight2)
        }

        // midpoint(MeasurePoint, MeasurePoint) -> MeasurePoint
        pub fn midpoint<Unit: MeasurementUnit, Number: ArithmeticOps>(
            p1: MeasurePoint<Unit, Number>, p2: MeasurePoint<Unit, Number>) -> MeasurePoint<Unit, Number>
        {
            MeasurePoint::<Unit, Number>::new(
                (p1.value + p2.value) * Number::HALF)
        }

        // barycentric_combination([MeasurePoint], [Number]) -> MeasurePoint
        pub fn barycentric_combination<Unit: MeasurementUnit, Number: ArithmeticOps>(
            points: &[MeasurePoint<Unit, Number>], weights: &[Number]) -> MeasurePoint<Unit, Number>
        {
            MeasurePoint::<Unit, Number>::new(
                points.iter().zip(weights).map(|(p, &w)| p.value * w).sum()
            )
        }

        impl<Unit, Number: ArithmeticOps> PartialEq<MeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number> {
            fn eq(&self, other: &MeasurePoint<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<MeasurePoint<Unit, Number>> for MeasurePoint<Unit, Number> {
            fn partial_cmp(&self, other: &MeasurePoint<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for MeasurePoint<Unit, Number> {
            fn clone(&self) -> Self { *self }
        }

        impl<Unit, Number: ArithmeticOps> Copy for MeasurePoint<Unit, Number> { }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display
            for MeasurePoint<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{}", self.value, Unit::SUFFIX)
            }
        }

        // UnsignedDirection

        pub struct UnsignedDirection<Unit, Number = f64> {
            value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> UnsignedDirection<Unit, Number> {
            /// Returns the only value that in the current Unit represents `x` and
            /// is between minus half turn included and plus half turn excluded.
            fn normalize(x: Number) -> Number {
                let one_turn = Number::from_f64(Unit::TURN_FRACTION);
                let x2 = x % one_turn;
                if x2 >= Number::ZERO { x2 } else { x2 + one_turn }
            }

            /// UnsignedDirection::new(Number) -> UnsignedDirection
            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            /// UnsignedDirection::from_measure_point(MeasurePoint) -> UnsignedDirection
            pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                Self {
                    value: Self::normalize(m.value),
                    phantom: PhantomData,
                }
            }

            pub fn to_measure_point(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.value) }

            pub fn to_signed_direction(self) -> SignedDirection<Unit, Number> {
                SignedDirection::<Unit, Number>::new(self.value)
            }

            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> UnsignedDirection<DestUnit, Number> {
                UnsignedDirection::<DestUnit, Number> {
                    value: self.value * Number::from_f64(Unit::RATIO / DestUnit::RATIO)
                        + Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO),
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> UnsignedDirection<Unit, DestNumber> {
                UnsignedDirection::<Unit, DestNumber> {
                    value: DestNumber::from(self.value),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> UnsignedDirection<Unit, DestNumber> {
                UnsignedDirection::<Unit, DestNumber> {
                    value: DestNumber::lossy_from(self.value),
                    phantom: PhantomData,
                }
            }
        }

        // Unsigned direction + angle measure -> Unsigned direction
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Add<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number> {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Unsigned direction += angle measure
        impl<Unit, Number: ArithmeticOps> AddAssign<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number> {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // Unsigned direction - angle measure -> Unsigned direction
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number> {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Unsigned direction -= angle measure
        impl<Unit, Number: ArithmeticOps> SubAssign<Measure<Unit, Number>>
            for UnsignedDirection<Unit, Number> {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // Unsigned direction - Unsigned direction -> Unsigned direction
        impl<Unit: AngleMeasurementUnit, Number: ArithmeticOps> Sub<UnsignedDirection<Unit, Number>>
            for UnsignedDirection<Unit, Number> {
            type Output = Measure<Unit, Number>;
            fn sub(self, other: UnsignedDirection<Unit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = turn * Number::HALF;
                Self::Output::new(if diff > half_turn { diff - turn } else if diff < -half_turn { diff + turn } else { diff })
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialEq<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number> {
            fn eq(&self, other: &UnsignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<UnsignedDirection<Unit, Number>> for UnsignedDirection<Unit, Number> {
            fn partial_cmp(&self, other: &UnsignedDirection<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for UnsignedDirection<Unit, Number> {
            fn clone(&self) -> Self { *self }
        }

        impl<Unit, Number: ArithmeticOps> Copy for UnsignedDirection<Unit, Number> { }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display
            for UnsignedDirection<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{} (in 0°..360°)", self.value, Unit::SUFFIX)
            }
        }

        // SignedDirection

        pub struct SignedDirection<Unit, Number = f64> {
            value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> SignedDirection<Unit, Number> {
            // Returns the only value that in the current Unit represents `x`, and
            // is between 0 included and one turn excluded.
            fn normalize(x: Number) -> Number {
                let one_turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = one_turn * Number::HALF;
                let x2 = (x + half_turn) % one_turn;
                if x2 >= Number::ZERO { x2 - half_turn } else { x2 + half_turn }
            }

            pub fn new(value: Number) -> Self {
                Self {
                    value: Self::normalize(value),
                    phantom: PhantomData,
                }
            }

            pub fn from_measure_point(m: MeasurePoint<Unit, Number>) -> Self {
                Self {
                    value: Self::normalize(m.value),
                    phantom: PhantomData,
                }
            }

            pub fn to_measure_point(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.value) }

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
            for SignedDirection<Unit, Number> {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Signed direction += angle measure
        impl<Unit, Number: ArithmeticOps> AddAssign<Measure<Unit, Number>>
            for SignedDirection<Unit, Number> {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // Signed direction - angle measure
        impl<Unit: AngleMeasurementUnit<Property = Angle>, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for SignedDirection<Unit, Number> {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Signed direction -= angle measure
        impl<Unit, Number: ArithmeticOps> SubAssign<Measure<Unit, Number>>
            for SignedDirection<Unit, Number> {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // Signed direction - Signed direction
        impl<Unit: AngleMeasurementUnit, Number: ArithmeticOps> Sub<SignedDirection<Unit, Number>>
            for SignedDirection<Unit, Number> {
            type Output = Measure<Unit, Number>;
            fn sub(self, other: SignedDirection<Unit, Number>) -> Self::Output {
                let diff = self.value - other.value;
                let turn = Number::from_f64(Unit::TURN_FRACTION);
                let half_turn = turn * Number::HALF;
                Self::Output::new(if diff > half_turn { diff - turn } else if diff < -half_turn { diff + turn } else { diff })
            }
        }


        impl<Unit, Number: ArithmeticOps> PartialEq<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number> {
            fn eq(&self, other: &SignedDirection<Unit, Number>) -> bool {
                self.value == other.value
            }
        }

        impl<Unit, Number: ArithmeticOps> PartialOrd<SignedDirection<Unit, Number>> for SignedDirection<Unit, Number> {
            fn partial_cmp(&self, other: &SignedDirection<Unit, Number>) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<Unit, Number: ArithmeticOps> Clone for SignedDirection<Unit, Number> {
            fn clone(&self) -> Self { *self }
        }

        impl<Unit, Number: ArithmeticOps> Copy for SignedDirection<Unit, Number> { }

        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display
            for SignedDirection<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at {}{} (in -180°..180°)", self.value, Unit::SUFFIX)
            }
        }
    };
}
