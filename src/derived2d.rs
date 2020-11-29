// U1 (Scalar) * U2 (Vector) == U3 (Vector)
// with U1 != U2
// For example: define_derived_measure_1_2!{Second, MetrePerSecond, Metre}
#[macro_export]
macro_rules! define_derived_measure_1_2 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty} => {
        // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<Number, $Unit2>> for Measure<Number, $Unit1> {
            type Output = Measure2d<Number, $Unit3>;
            fn mul(self, other: Measure2d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.value * other.x, self.value * other.y)
            }
        }

        // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit1>> for Measure2d<Number, $Unit2> {
            type Output = Measure2d<Number, $Unit3>;
            fn mul(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.value, self.y * other.value)
            }
        }

        // Measure2d<U3> / Measure<U1> -> Measure2d<U2>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for Measure2d<Number, $Unit3> {
            type Output = Measure2d<Number, $Unit2>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x / other.value, self.y / other.value)
            }
        }
    };
}

// U1 (Vector) * U2 (Vector) == U3 (Scalar)
// with U1 != U2
// U1 (Vector) X U2 (Vector) == U4 (Vector)
// with U1 != U2
// For example: define_derived_measure_2_2!{Newton, Metre, Joule, NewtonMetre}
#[macro_export]
macro_rules! define_derived_measure_2_2 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty, $Unit4:ty} => {
        // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<Number, $Unit2>> for Measure2d<Number, $Unit1> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure2d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }

        // Measure2d<U2> * Measure2d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<Number, $Unit1>> for Measure2d<Number, $Unit2> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure2d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }

        // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U4>
        impl<Number: ArithmeticOps> CrossProduct<Measure2d<Number, $Unit2>> for Measure2d<Number, $Unit1> {
            type Output = Measure<Number, $Unit4>;
            fn cross_product(self, other: Measure2d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }

        // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U4>
        impl<Number: ArithmeticOps> CrossProduct<Measure2d<Number, $Unit1>> for Measure2d<Number, $Unit2> {
            type Output = Measure<Number, $Unit4>;
            fn cross_product(self, other: Measure2d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }
    };
}

// U1 (Vector) * U1 (Vector) == U2 (Scalar)
// U1 (Vector) X U1 (Vector) == U3 (Scalar)
// For example: define_derived_measure_squared_2!{Metre, Metre, Metre}
#[macro_export]
macro_rules! define_derived_measure_squared_2 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty} => {
        // Measure2d<U1> * Measure2d<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Mul<Measure2d<Number, $Unit1>> for Measure2d<Number, $Unit1> {
            type Output = Measure<Number, $Unit2>;
            fn mul(self, other: Measure2d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }

        // Measure2d<U2>.sqr() -> Measure<U1>
        impl<Number: ArithmeticOps> Measure2d<Number, $Unit2> {
            fn squared(self) -> Measure<Number, $Unit1> {
                Measure::<Number, $Unit1>::new(self.x * self.x + self.y * self.y)
            }
        }

        // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> CrossProduct<Measure2d<Number, $Unit1>> for Measure2d<Number, $Unit1> {
            type Output = Measure<Number, $Unit3>;
            fn cross_product(self, other: Measure2d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }
    };
}
