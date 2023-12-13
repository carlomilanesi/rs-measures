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
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Measure3d<Unit, Number> {
            pub fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }

            pub fn x(self) -> Measure<Unit, Number> { Measure::<Unit, Number>::new(self.x) }

            pub fn y(self) -> Measure<Unit, Number> { Measure::<Unit, Number>::new(self.y) }
            pub fn z(self) -> Measure<Unit, Number> { Measure::<Unit, Number>::new(self.z) }
            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
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

            pub fn squared_norm(self) -> Number {
                self.x * self.x + self.y * self.y + self.z * self.z
            }
        }

        // -Measure3d -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Neg for Measure3d<Unit, Number> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }

        // Measure3d * Number -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Mul<Number> for Measure3d<Unit, Number> {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.x * n, self.y * n, self.z * n)
            }
        }

        // Measure3d *= Number
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> MulAssign<Number> for Measure3d<Unit, Number> {
            fn mul_assign(&mut self, n: Number) {
                self.x *= n;
                self.y *= n;
                self.z *= n;
            }
        }

        // f64 * Measure3d -> Measure3d
        impl<Unit: VectorMeasurementUnit> Mul<Measure3d<Unit, f64>> for f64 {
            type Output = Measure3d<Unit, f64>;
            fn mul(self, other: Measure3d<Unit, f64>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // f32 * Measure3d -> Measure3d
        impl<Unit: VectorMeasurementUnit> Mul<Measure3d<Unit, f32>> for f32 {
            type Output = Measure3d<Unit, f32>;
            fn mul(self, other: Measure3d<Unit, f32>) -> Self::Output {
                Self::Output::new(self * other.x, self * other.y, self * other.z)
            }
        }

        // Measure3d / Number -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Div<Number> for Measure3d<Unit, Number> {
            type Output = Self;
            fn div(self, n: Number) -> Self::Output {
                Self::new(self.x / n, self.y / n, self.z / n)
            }
        }

        // Measure3d /= Number
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> DivAssign<Number> for Measure3d<Unit, Number> {
            fn div_assign(&mut self, n: Number) {
                self.x /= n;
                self.y /= n;
                self.z /= n;
            }
        }

        // Measure3d + Measure3d -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Add<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // Measure3d += Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> AddAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number> {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // Measure3d - Measure3d -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<Measure3d<Unit, Number>>
            for Measure3d<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // Measure3d -= Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> SubAssign<Measure3d<Unit, Number>> for Measure3d<Unit, Number> {
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
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> MeasurePoint3d<Unit, Number> {
            pub fn new(x: Number, y: Number, z: Number) -> Self {
                Self {
                    x,
                    y,
                    z,
                    phantom: PhantomData,
                }
            }

            pub fn x(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.x) }

            pub fn y(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.y) }

            pub fn z(self) -> MeasurePoint<Unit, Number> { MeasurePoint::<Unit, Number>::new(self.z) }

            pub fn convert<DestUnit: VectorMeasurementUnit<Property = Unit::Property>>(
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
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Add<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        {
            type Output = Self;
            fn add(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        // MeasurePoint3d += Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> AddAssign<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        {
            fn add_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // MeasurePoint3d - Measure3d -> MeasurePoint3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure3d<Unit, Number>) -> Self::Output {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // MeasurePoint3d -= Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> SubAssign<Measure3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number>
        {
            fn sub_assign(&mut self, other: Measure3d<Unit, Number>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        // MeasurePoint3d - MeasurePoint3d -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Sub<MeasurePoint3d<Unit, Number>>
            for MeasurePoint3d<Unit, Number> {
            type Output = Measure3d<Unit, Number>;
            fn sub(self, other: MeasurePoint3d<Unit, Number>) -> Self::Output {
                Self::Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        // Measure3d == Measure3d -> bool
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> PartialEq<Measure3d<Unit, Number>> for Measure3d<Unit, Number> {
            fn eq(&self, other: &Measure3d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        // Measure3d.clone() -> Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Clone for Measure3d<Unit, Number> {
            fn clone(&self) -> Self {
                *self
                /*
                Measure3d::<Unit, Number> {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    phantom: std::marker::PhantomData::<Unit>,
                }
                */
            }
        }

        // Measure3d = Measure3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Copy for Measure3d<Unit, Number> { }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> fmt::Display for Measure3d<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
        }

        // MeasurePoint3d == MeasurePoint3d -> bool
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> PartialEq<MeasurePoint3d<Unit, Number>> for MeasurePoint3d<Unit, Number> {
            fn eq(&self, other: &MeasurePoint3d<Unit, Number>) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        // MeasurePoint3d.clone() -> MeasurePoint3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Clone for MeasurePoint3d<Unit, Number> {
            fn clone(&self) -> Self {
                *self
                /*
                MeasurePoint3d::<Unit, Number> {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    phantom: std::marker::PhantomData::<Unit>,
                }
                */
            }
        }

        // MeasurePoint3d = MeasurePoint3d
        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> Copy for MeasurePoint3d<Unit, Number> { }

        impl<Unit: VectorMeasurementUnit, Number: ArithmeticOps> fmt::Display for MeasurePoint3d<Unit, Number> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "at ({}, {}, {}){}", self.x, self.y, self.z, Unit::SUFFIX)
            }
        }
    };
}

// affine_map3
/*
    // convert(affine_map3)
    template <class ToUnit, class FromUnit, typename Num>
    affine_map3<ToUnit,Num> convert(affine_map3<FromUnit,Num> map)
    {
        affine_map3<ToUnit,Num> result;
        result.coeff(0, 0) = map.coeff(0, 0);
        result.coeff(0, 1) = map.coeff(0, 1);
        result.coeff(0, 2) = map.coeff(0, 2);
        result.coeff(0, 3) = map.coeff(0, 3) *
            Unit::RATIO / DestUnit::RATIO;
        result.coeff(1, 0) = map.coeff(1, 0);
        result.coeff(1, 1) = map.coeff(1, 1);
        result.coeff(1, 2) = map.coeff(1, 2);
        result.coeff(1, 3) = map.coeff(1, 3) *
            Unit::RATIO / DestUnit::RATIO;
        result.coeff(2, 0) = map.coeff(2, 0);
        result.coeff(2, 1) = map.coeff(2, 1);
        result.coeff(2, 2) = map.coeff(2, 2);
        result.coeff(2, 3) = map.coeff(2, 3) *
            Unit::RATIO / DestUnit::RATIO;
        return result;
    }

*/
