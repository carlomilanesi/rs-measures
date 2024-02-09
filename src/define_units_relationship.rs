#[macro_export]
macro_rules! define_units_relationship {
    { $id1:ident == $id2:ident * $id3:ident} => { rs_measures::expand_1_1! {$id2 $id3 $id1} };
    { $id1:ident == $id2:ident * =} => { rs_measures::expand_1_1_same! {$id2 $id1} };
    { $id1:ident:2 == $id2:ident * $id3:ident:2} => { rs_measures::expand_1_2! {$id2 $id3 $id1} };
    { $id1:ident:2 == $id2:ident:2 * $id3:ident} => { rs_measures::expand_1_2! {$id3 $id2 $id1} };
    { $id1:ident:3 == $id2:ident * $id3:ident:3} => { rs_measures::expand_1_3! {$id2 $id3 $id1} };
    { $id1:ident:3 == $id2:ident:3 * $id3:ident} => { rs_measures::expand_1_3! {$id3 $id2 $id1} };
    { $id1:ident == $id2:ident :2 * $id3:ident:2} => { rs_measures::expand_2_2! {$id2 $id3 $id1} };
    { $id1:ident == $id2:ident :2 * =:2} => { rs_measures::expand_2_2_same! {$id2 $id1} };
    { $id1:ident == $id2:ident :3 * $id3:ident:3} => { rs_measures::expand_3_3! {$id2 $id3 $id1} };
    { $id1:ident == $id2:ident :3 * =:3} => { rs_measures::expand_3_3_same! {$id2 $id1} };
    { $id1:ident == 1 / $id3:ident } => { rs_measures::expand_inverse! {$id1 $id3} };
    { $id1:ident == $id2:ident:2 X $id3:ident:2 } => { rs_measures::expand_cross_2! {$id2 $id3 $id1} };
    { $id1:ident == $id2:ident:2 X =:2 } => { rs_measures::expand_cross_2_same! {$id2 $id1} };
    { $id1:ident :3 == $id2:ident:3 X $id3:ident:3 } => { rs_measures::expand_cross_3! {$id2 $id3 $id1} };
    { $id1:ident :3 == $id2:ident:3 X =:3 } => { rs_measures::expand_cross_3_same! {$id2 $id1} };
}

#[macro_export]
macro_rules! expand_1_1 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure<U1> * Measure<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U2> * Measure<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U3> / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure<$unit3, Number> {
            type Output = Measure<$unit2, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }

        // Measure<U3> / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<$unit2, Number>> for Measure<$unit3, Number> {
            type Output = Measure<$unit1, Number>;
            fn div(self, other: Measure<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_1_1_same {
    { $unit1:ident $unit3:ident } => {
        // Measure<U1> * Measure<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.value * other.value)
            }
        }

        // Measure<U3> / Measure<U1> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure<$unit3, Number> {
            type Output = Measure<$unit1, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.value / other.value)
            }
        }

        // Measure<U1>.squared() -> Measure<U3>
        impl<Number: ArithmeticOps> Measure<$unit1, Number> {
            fn squared(self) -> Measure<$unit3, Number> {
                Measure::<$unit3, Number>::new(self.value * self.value)
            }
        }
        // Measure<U3>.sqrt() -> Measure<U1>
        impl<Number: ArithmeticOps> Sqrt for Measure<$unit3, Number> {
            type Output = Measure<$unit1, Number>;
            fn sqrt(self) -> Self::Output {
                Self::Output::new(self.value.sqrt())
            }
        }
    };
}

#[macro_export]
macro_rules! expand_1_2 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Measure2d<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.value * other.x, self.value * other.y)
            }
        }

        // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure2d<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.value, self.y * other.value)
            }
        }

        // Measure2d<U3> / Measure<U1> -> Measure2d<U2>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure2d<$unit3, Number> {
            type Output = Measure2d<$unit2, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x / other.value, self.y / other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_1_3 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new(
                    self.value * other.x,
                    self.value * other.y,
                    self.value * other.z,
                )
            }
        }

        // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.x * other.value,
                    self.y * other.value,
                    self.z * other.value,
                )
            }
        }

        // Measure3d<U3> / Measure<U1> -> Measure3d<U2>
        impl<Number: ArithmeticOps> Div<Measure<$unit1, Number>> for Measure3d<$unit3, Number> {
            type Output = Measure3d<$unit2, Number>;
            fn div(self, other: Measure<$unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.x / other.value,
                    self.y / other.value,
                    self.z / other.value,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! expand_2_2 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit2, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }

        // Measure2d<U2> * Measure2d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_2_2_same {
    {$unit1:ident $unit2:ident} => {
        // Measure2d<U1> * Measure2d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<$unit1, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn mul(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y)
            }
        }

        // Measure2d<U1>.squared() -> Measure<U3>
        impl<Number: ArithmeticOps> Measure2d<$unit1, Number> {
            fn squared(self) -> Measure<$unit2, Number> {
                Measure::<$unit2, Number>::new(self.x * self.x + self.y * self.y)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_3_3_same {
    {$unit1:ident $unit2:ident} => {
        // Measure3d<U1> * Measure3d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit1, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn mul(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }

        // Measure3d<U1>.squared() -> Measure<U3>
        impl<Number: ArithmeticOps> Measure3d<$unit1, Number> {
            fn squared(self) -> Measure<$unit2, Number> {
                Measure::<$unit2, Number>::new(self.x * self.x + self.y * self.y + self.z * self.z)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_3_3 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit2, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }

        // Measure3d<U2> * Measure3d<U1> -> Measure<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn mul(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_inverse {
    {$unit1:ident $unit2:ident} => {
        // Measure<U1> * Measure<U2> -> N
        impl<Number: ArithmeticOps> Mul<Measure<$unit2, Number>> for Measure<$unit1, Number> {
            type Output = Number;
            fn mul(self, other: Measure<$unit2, Number>) -> Self::Output {
                self.value * other.value
            }
        }

        // Measure<U2> * Measure<U1> -> N
        impl<Number: ArithmeticOps> Mul<Measure<$unit1, Number>> for Measure<$unit2, Number> {
            type Output = Number;
            fn mul(self, other: Measure<$unit1, Number>) -> Self::Output {
                self.value * other.value
            }
        }

        // N64 / Measure<U1> -> Measure<U2>
        impl Div<Measure<$unit1, f64>> for f64 {
            type Output = Measure<$unit2, f64>;
            fn div(self, other: Measure<$unit1, f64>) -> Self::Output {
                Self::Output::new(self / other.value)
            }
        }

        // N32 / Measure<U1> -> Measure<U2>
        impl Div<Measure<$unit1, f32>> for f32 {
            type Output = Measure<$unit2, f32>;
            fn div(self, other: Measure<$unit1, f32>) -> Self::Output {
                Self::Output::new(self / other.value)
            }
        }

        // N64 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<$unit2, Number>> for f64 {
            type Output = Measure<$unit1, Number>;
            fn div(self, other: Measure<$unit2, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self) / other.value)
            }
        }

        // N32 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<$unit2, Number>> for f32 {
            type Output = Measure<$unit1, Number>;
            fn div(self, other: Measure<$unit2, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self as f64) / other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_cross_2_same {
    {$unit1:ident $unit2:ident} => {
        // Measure2d<U1>.cross_product(Measure2d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure2d<$unit1, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit2, Number>;
            fn cross_product(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_cross_2 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U3>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure2d<$unit2, Number>> for Measure2d<$unit1, Number> {
            type Output = Measure<$unit3, Number>;
            fn cross_product(self, other: Measure2d<$unit2, Number>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }

        // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure2d<$unit1, Number>> for Measure2d<$unit2, Number> {
            type Output = Measure<$unit3, Number>;
            fn cross_product(self, other: Measure2d<$unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.y - self.y * other.x)
            }
        }
    };
}

#[macro_export]
macro_rules! expand_cross_3_same {
    {$unit1:ident $unit2:ident} => {
        // Measure3d<U1>.cross_product(Measure3d<U1>) -> Measure<U3>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure3d<$unit1, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure3d<$unit2, Number>;
            fn cross_product(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! expand_cross_3 {
    {$unit1:ident $unit2:ident $unit3:ident} => {
        // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure3d<$unit2, Number>> for Measure3d<$unit1, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn cross_product(self, other: Measure3d<$unit2, Number>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }

        // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
        impl<Number: ArithmeticOps> rs_measures::traits::CrossProduct<Measure3d<$unit1, Number>> for Measure3d<$unit2, Number> {
            type Output = Measure3d<$unit3, Number>;
            fn cross_product(self, other: Measure3d<$unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }
        }
    };
}
