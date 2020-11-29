#[macro_export]
macro_rules! define_measure3d {
    {} => {
        pub struct Measure3d<Number: ArithmeticOps, Unit> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            phantom: std::marker::PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Measure3d<Number, Unit> {
            pub fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> Measure3d<Number, DestUnit> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                Measure3d::<Number, DestUnit> {
                    x: self.x * factor,
                    y: self.y * factor,
                    z: self.z * factor,
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> Measure3d<DestNumber, Unit> {
                Measure3d::<DestNumber, Unit> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    z: DestNumber::from(self.z),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> Measure3d<DestNumber, Unit> {
                Measure3d::<DestNumber, Unit> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    z: DestNumber::lossy_from(self.z),
                    phantom: PhantomData,
                }
            }

            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y + self.z * self.z
            }
        }

        // -measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Neg for Measure3d<Number, Unit> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }

        // measure * number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Mul<Number> for Measure3d<Number, Unit> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n, self.z * n)
            }
        }

        // measure *= number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> MulAssign<Number> for Measure3d<Number, Unit> {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
                self.z *= n;
            }
        }

        // measure / number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Div<Number> for Measure3d<Number, Unit> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n, self.z / n)
            }
        }

        // measure /= number
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> DivAssign<Number> for Measure3d<Number, Unit> {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
                self.z /= n;
            }
        }

        // measure + measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Add<Measure3d<Number, Unit>>
            for Measure3d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure3d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // measure += measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> AddAssign<Measure3d<Number, Unit>> for Measure3d<Number, Unit> {
            fn add_assign(&mut self, other: Measure3d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // measure - measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<Measure3d<Number, Unit>>
            for Measure3d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // measure -= measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> SubAssign<Measure3d<Number, Unit>> for Measure3d<Number, Unit> {
            fn sub_assign(&mut self, other: Measure3d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        pub struct MeasurePoint3d<Number, Unit> {
            pub x: Number,
            pub y: Number,
            pub z: Number,
            phantom: PhantomData<Unit>,
        }
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> MeasurePoint3d<Number, Unit> {
            pub fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }
            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
                &self,
            ) -> MeasurePoint3d<Number, DestUnit> {
                let factor = Number::from_f64(Unit::RATIO / DestUnit::RATIO);
                let offset = Number::from_f64((Unit::OFFSET - DestUnit::OFFSET) / DestUnit::RATIO);
                MeasurePoint3d::<Number, DestUnit> {
                    x: self.x * factor + offset,
                    y: self.y * factor + offset,
                    z: self.z * factor + offset,
                    phantom: PhantomData,
                }
            }
            pub fn lossless_into<DestNumber: ArithmeticOps + From<Number>>(
                &self,
            ) -> MeasurePoint3d<DestNumber, Unit> {
                MeasurePoint3d::<DestNumber, Unit> {
                    x: DestNumber::from(self.x),
                    y: DestNumber::from(self.y),
                    z: DestNumber::from(self.z),
                    phantom: PhantomData,
                }
            }
            pub fn lossy_into<DestNumber: ArithmeticOps + LossyFrom<Number>>(
                &self,
            ) -> MeasurePoint3d<DestNumber, Unit> {
                MeasurePoint3d::<DestNumber, Unit> {
                    x: DestNumber::lossy_from(self.x),
                    y: DestNumber::lossy_from(self.y),
                    z: DestNumber::lossy_from(self.z),
                    phantom: PhantomData,
                }
            }
        }

        // measure point + measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Add<Measure3d<Number, Unit>>
            for MeasurePoint3d<Number, Unit>
        {
            type Output = Self;
            fn add(self, other: Measure3d<Number, Unit>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // measure point += measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> AddAssign<Measure3d<Number, Unit>>
            for MeasurePoint3d<Number, Unit>
        {
            fn add_assign(&mut self, other: Measure3d<Number, Unit>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // measure point - measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<Measure3d<Number, Unit>>
            for MeasurePoint3d<Number, Unit>
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Number, Unit>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // measure point -= measure
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> SubAssign<Measure3d<Number, Unit>>
            for MeasurePoint3d<Number, Unit>
        {
            fn sub_assign(&mut self, other: Measure3d<Number, Unit>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        // measure point - measure point
        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Sub<MeasurePoint3d<Number, Unit>>
            for MeasurePoint3d<Number, Unit> {
            type Output = Measure3d<Number, Unit>;
            fn sub(self, other: MeasurePoint3d<Number, Unit>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> PartialEq<Measure3d<Number, Unit>> for Measure3d<Number, Unit> {
            fn eq(&self, other: &Measure3d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Clone for Measure3d<Number, Unit> {
            fn clone(&self) -> Self {
                Measure3d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Copy for Measure3d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> fmt::Display for Measure3d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> PartialEq<MeasurePoint3d<Number, Unit>> for MeasurePoint3d<Number, Unit> {
            fn eq(&self, other: &MeasurePoint3d<Number, Unit>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Clone for MeasurePoint3d<Number, Unit> {
            fn clone(&self) -> Self {
                MeasurePoint3d::<Number, Unit> {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    phantom: std::marker::PhantomData::<Unit>,
                }
            }
        }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> Copy for MeasurePoint3d<Number, Unit> { }

        impl<Number: ArithmeticOps, Unit: VectorMeasurementUnit> fmt::Display for MeasurePoint3d<Number, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
        }
    };
}
