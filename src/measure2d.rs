//use rs_measures::angle::Radian;
#[macro_export]
macro_rules! define_measure2d {
    {} => {

        // Measure2d

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

            pub fn unsigned_direction(self) -> UnsignedDirection<Number, rs_measures::angle::Radian> {
                UnsignedDirection::<Number, rs_measures::angle::Radian>::new(self.y.atan2(self.x))
            }

            pub fn signed_direction(self) -> SignedDirection<Number, rs_measures::angle::Radian> {
                SignedDirection::<Number, rs_measures::angle::Radian>::new(self.y.atan2(self.x))
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

        pub struct UnsignedDirection<Number, Unit> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> UnsignedDirection<Number, Unit> {
            pub fn new(value: Number) -> Self {
                Self {
                    //TODO: modulo
                    value,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> UnsignedDirection<Number, DestUnit> {
                UnsignedDirection::<Number, DestUnit> {
                    //TODO: modulo
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<UnsignedDirection<Number, Unit>>
            for UnsignedDirection<Number, Unit> {
            type Output = Measure<Number, Unit>;
            fn sub(self, other: UnsignedDirection<Number, Unit>) -> Self::Output {
                Self::Output::new(self.value - other.value)
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
                write!(f, "at {}{}", self.value, Unit::SUFFIX)
            }
        }

        // Signed direction

        pub struct SignedDirection<Number, Unit> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> SignedDirection<Number, Unit> {
            pub fn new(value: Number) -> Self {
                Self {
                    //TODO: modulo
                    value,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> SignedDirection<Number, DestUnit> {
                SignedDirection::<Number, DestUnit> {
                    //TODO: modulo
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure<Number, Unit>>
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure<Number, Unit>>
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<SignedDirection<Number, Unit>>
            for SignedDirection<Number, Unit> {
            type Output = Measure<Number, Unit>;
            fn sub(self, other: SignedDirection<Number, Unit>) -> Self::Output {
                Self::Output::new(self.value - other.value)
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
                write!(f, "at {}{}", self.value, Unit::SUFFIX)
            }
        }
    };
}
