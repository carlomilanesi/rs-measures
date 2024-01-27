#[macro_export]
macro_rules! inner_define_measure {
    {} => {
        pub struct Measure<Unit, Number: ArithmeticOps = f64> {
            pub value: Number,
            phantom: PhantomData<Unit>,
        }

        impl<Unit, Number> Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            /// Measure::new(Number) -> Measure
            pub const fn new(value: Number) -> Self {
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

        impl<Unit, Number> Default for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            // It returns the zero vector.
            fn default() -> Self {
                Self::new(Number::ZERO)
            }
        }

        // -Measure -> Measure
        impl<Unit, Number> Neg for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        // Measure + Measure -> Measure
        impl<Unit, Number> Add<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn add(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value + other.value)
            }
        }

        // Measure += Measure
        impl<Unit, Number> AddAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn add_assign(&mut self, other: Measure<Unit, Number>) {
                self.value += other.value;
            }
        }

        // Measure - Measure -> Measure
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Sub<Measure<Unit, Number>>
            for Measure<Unit, Number>
        {
            type Output = Self;
            fn sub(self, other: Measure<Unit, Number>) -> Self::Output {
                Self::new(self.value - other.value)
            }
        }

        // Measure -= Measure
        impl<Unit, Number> SubAssign<Measure<Unit, Number>> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn sub_assign(&mut self, other: Measure<Unit, Number>) {
                self.value -= other.value;
            }
        }

        // Measure * Number -> Measure
        impl<Unit, Number> Mul<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            type Output = Self;
            fn mul(self, n: Number) -> Self::Output {
                Self::new(self.value * n)
            }
        }

        // Measure *= Number
        impl<Unit, Number> MulAssign<Number> for Measure<Unit, Number>
        where
            Unit: MeasurementUnit,
            Number: ArithmeticOps,
        {
            fn mul_assign(&mut self, n: Number) {
                self.value *= n;
            }
        }

        // f64 * Measure -> Measure
        impl<Unit> Mul<Measure<Unit, f64>> for f64
        where
            Unit: MeasurementUnit,
        {
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
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> Div<Measure<Unit, Number>>
            for Measure<Unit, Number>
        {
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
            fn clone(&self) -> Self {
                *self
            }
        }

        // Measure = Measure
        impl<Unit, Number: ArithmeticOps> Copy for Measure<Unit, Number> {}

        // format!("{}", Measure)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Display for Measure<Unit, Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }

        // format!("{:?}", Measure)
        impl<Unit: MeasurementUnit, Number: ArithmeticOps> fmt::Debug for Measure<Unit, Number> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, formatter)?;
                formatter.write_str(Unit::SUFFIX)
            }
        }
    };
}
