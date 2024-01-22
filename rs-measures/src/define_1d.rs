#[macro_export]
macro_rules! define_1d {
    {} => {
        use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
        use rs_measures::{
            angle::{Angle, Radian},
            traits::{
                AngleMeasurementUnit, ArithmeticOps, LossyFrom, MeasurementUnit, Sqrt, VectorProperty,
            },
        };
        use std::fmt;
        use std::marker::PhantomData;

        rs_measures::inner_define_measure! {}
        rs_measures::inner_define_measure_point! {}
    };
}
