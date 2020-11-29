// U1 (Scalar) * U2 (Vector) == U3 (Vector)
// with U1 != U2
// For example: define_derived_measure_1_3!{Second, MetrePerSecond, Metre}
#[macro_export]
macro_rules! define_derived_measure_1_3 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty} => {
        // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<Number, $Unit2>> for Measure<Number, $Unit1> {
            type Output = Measure3d<Number, $Unit3>;
            fn mul(self, other: Measure3d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(
                    self.value * other.x,
                    self.value * other.y,
                    self.value * other.z,
                )
            }
        }

        // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<Number, $Unit1>> for Measure3d<Number, $Unit2> {
            type Output = Measure3d<Number, $Unit3>;
            fn mul(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(
                    self.x * other.value,
                    self.y * other.value,
                    self.z * other.value,
                )
            }
        }

        // Measure3d<U3> / Measure<U1> -> Measure3d<U2>
        impl<Number: ArithmeticOps> Div<Measure<Number, $Unit1>> for Measure3d<Number, $Unit3> {
            type Output = Measure3d<Number, $Unit2>;
            fn div(self, other: Measure<Number, $Unit1>) -> Self::Output {
                Self::Output::new(
                    self.x / other.value,
                    self.y / other.value,
                    self.z / other.value,
                )
            }
        }
    };
}

// U1 (Vector) * U2 (Vector) == U3 (Scalar)
// with U1 != U2
// U1 (Vector) X U2 (Vector) == U4 (Vector)
// with U1 != U2
// For example: define_derived_measure_3_3!{Newton, Metre, Joule, NewtonMetre}
#[macro_export]
macro_rules! define_derived_measure_3_3 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty, $Unit4:ty} => {
        // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<Number, $Unit2>> for Measure3d<Number, $Unit1> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure3d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }

        // Measure3d<U2> * Measure3d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<Number, $Unit1>> for Measure3d<Number, $Unit2> {
            type Output = Measure<Number, $Unit3>;
            fn mul(self, other: Measure3d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }

        // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
        impl<Number: ArithmeticOps> CrossProduct<Measure3d<Number, $Unit2>> for Measure3d<Number, $Unit1> {
            type Output = Measure3d<Number, $Unit4>;
            fn cross_product(self, other: Measure3d<Number, $Unit2>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }

        // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
        impl<Number: ArithmeticOps> CrossProduct<Measure3d<Number, $Unit1>> for Measure3d<Number, $Unit2> {
            type Output = Measure3d<Number, $Unit4>;
            fn cross_product(self, other: Measure3d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }
    };
}

// U1 (Vector) * U1 (Vector) == U2 (Scalar)
// U1 (Vector) X U1 (Vector) == U3 (Vector)
// For example: define_derived_measure_squared_3!{Metre, Metre, Metre}
#[macro_export]
macro_rules! define_derived_measure_squared_3 {
    {$Unit1:ty, $Unit2:ty, $Unit3:ty} => {
        // Measure3d<U1> * Measure3d<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Mul<Measure3d<Number, $Unit1>> for Measure3d<Number, $Unit1> {
            type Output = Measure<Number, $Unit2>;
            fn mul(self, other: Measure3d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }

        // Measure3d<U1>.sqr() -> Measure<U2>
        impl<Number: ArithmeticOps> Measure3d<Number, $Unit1> {
            fn squared(self) -> Measure<Number, $Unit2> {
                Measure::<Number, $Unit2>::new(self.x * self.x + self.y * self.y + self.z * self.z)
            }
        }

        // Measure3d<U1>.cross_product(Measure2d<U1>) -> Measure3d<U3>
        impl<Number: ArithmeticOps> CrossProduct<Measure3d<Number, $Unit1>> for Measure3d<Number, $Unit1> {
            type Output = Measure3d<Number, $Unit3>;
            fn cross_product(self, other: Measure3d<Number, $Unit1>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }
    };
}
