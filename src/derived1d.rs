// U1 (Scalar) * U2 (Scalar) == U3 (Scalar)
// with U1 != U2
// For example: define_derived_measure_1_1!{Second, MetrePerSecond, Metre}
#[macro_export]
macro_rules! define_derived_measure_1_1 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty} => {
        // Measure<U1> * Measure<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit2>> for Measure<Number, $Unit1> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U2> * Measure<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit1>> for Measure<Number, $Unit2> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U3> / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for Measure<Number, $Unit3> {
            type Output = Measure<Number, $Unit2>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }

        // Measure<U3> / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit2>> for Measure<Number, $Unit3> {
            type Output = Measure<Number, $Unit1>;
            fn div(self, other: Measure<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }
    };
}

// U1 (Scalar) * U1 (Scalar) == U2 (Scalar)
// For example: define_derived_measure_squared_1!{Metre, SquareMetre}
#[macro_export]
macro_rules! define_derived_measure_squared_1 {
    {$Unit1:ty, $Unit2:ty} => {
        // Measure<U1> * Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit1>> for Measure<Number, $Unit1> {
            type Output = Measure<Number, $Unit2>;
            fn mul(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U2> / Measure<U1> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for Measure<Number, $Unit2> {
            type Output = Measure<Number, $Unit1>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }

        // Measure<U1>.squared() -> Measure<U2>
        impl<Number: ArithmeticOps> Measure<Number, $Unit1> {
            fn squared(self) -> Measure<Number, $Unit2> {
                Measure::<Number, $Unit2>::new(self.value * self.value)
            }
        }
        // Measure<U2>.sqrt() -> Measure<U1>
        impl<Number: ArithmeticOps> Sqrt for Measure<Number, $Unit2> {
            type Output = Measure<Number, $Unit1>;
            fn sqrt(self) -> Self::Output {
                Self::Output::new(self.value.sqrt())
            }
        }
    };
}

// U1 (Scalar) * U2 (Scalar) == N
// with U1 != U2
// For example: define_derived_measure_inverse_1!{Hertz, Second}
#[macro_export]
macro_rules! define_derived_measure_inverse_1 {
    {$Unit1:ty, $Unit2:ty} => {
        // Measure<U1> * Measure<U2> -> N
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit2>> for Measure<Number, $Unit1> {
            type Output = Number;
            fn mul(self, other: Measure<Number, $Unit2>) -> Self::Output {
                self.value * other.value
            }
        }

        // Measure<U2> * Measure<U1> -> N
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit1>> for Measure<Number, $Unit2> {
            type Output = Number;
            fn mul(self, other: Measure<Number, $Unit1>) -> Self::Output {
                self.value * other.value
            }
        }

        // N64 / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for f64 {
            type Output = Measure<Number, $Unit2>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(Number::from_f64(self) / other.value)
            }
        }

        // N32 / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for f32 {
            type Output = Measure<Number, $Unit2>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(Number::from_f64(self as f64) / other.value)
            }
        }

        // N64 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit2>> for f64 {
            type Output = Measure<Number, $Unit1>;
            fn div(self, other: Measure<Number, $Unit2>) -> Self::Output {
                Self::Output::new(Number::from_f64(self) / other.value)
            }
        }

        // N32 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit2>> for f32 {
            type Output = Measure<Number, $Unit1>;
            fn div(self, other: Measure<Number, $Unit2>) -> Self::Output {
                Self::Output::new(Number::from_f64(self as f64) / other.value)
            }
        }
        //TODO: try to avoid to duplicate the code for every supported numeric type
    };
}
