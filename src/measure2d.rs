#[macro_export]
macro_rules! define_measure2d {
    {} => {
        pub struct Measure2d<Number: ArithmeticOps, Unit> {
            pub x: Number,
            pub y: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Measure2d<Number, Unit> {
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Quantity = Unit::Quantity>>(
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
        }

        // measure * number
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Mul<Number> for Measure2d<Number, Unit> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n)
            }
        }

        // measure *= number
        impl<Number: ArithmeticOps, Unit> MulAssign<Number> for Measure2d<Number, Unit> {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
            }
        }

        // measure / number
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Div<Number> for Measure2d<Number, Unit> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n)
            }
        }

        // measure /= number
        impl<Number: ArithmeticOps, Unit> DivAssign<Number> for Measure2d<Number, Unit> {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
            }
        }

        // measure + measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure2d<Number, Unit>>
            for Measure2d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure += measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn add_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure - measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure2d<Number, Unit>>
            for Measure2d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure -= measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn sub_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        pub struct MeasurePoint2d<Number, Unit> {
            pub x: Number,
            pub y: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> MeasurePoint2d<Number, Unit> {
            pub fn new(x: Number, y: Number) -> Self {
                Self {
                    x,
                    y,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: MeasurementUnit<Quantity = Unit::Quantity>>(
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
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Add<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }

        // measure point += measure
        impl<Number: ArithmeticOps, Unit> AddAssign<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            fn add_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        // measure point - measure
        impl<Number: ArithmeticOps, Unit: MeasurementUnit> Sub<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure2d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y)
            }
        }

        // measure point -= measure
        impl<Number: ArithmeticOps, Unit> SubAssign<Measure2d<Number, Unit>>
            for MeasurePoint2d<Number, Unit>
        {
            fn sub_assign(&mut self, other: Measure2d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialEq<Measure2d<Number, Unit>> for Measure2d<Number, Unit> {
            fn eq(&self, other: &Measure2d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for Measure2d<Number, Unit> {
            fn clone(&self) -> Self {
                Measure2d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for Measure2d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display for Measure2d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }

        impl<Number: ArithmeticOps, Unit> PartialEq<MeasurePoint2d<Number, Unit>> for MeasurePoint2d<Number, Unit> {
            fn eq(&self, other: &MeasurePoint2d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl<Number: ArithmeticOps, Unit> Clone for MeasurePoint2d<Number, Unit> {
            fn clone(&self) -> Self {
                MeasurePoint2d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit> Copy for MeasurePoint2d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: MeasurementUnit> fmt::Display for MeasurePoint2d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}){}", self.x, self.y, Unit::SUFFIX)
            }
        }
    };
}
