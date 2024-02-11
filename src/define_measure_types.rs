pub enum UncertaintyFeatures {
    None,
    Symmetric,
    TwoSided,
}

pub struct MeasureFeatures {
    pub with_points: bool,
    pub with_directions: bool,
    pub with_2d: bool,
    pub with_3d: bool,
    pub with_transformations: bool,
    pub with_uncertainty: UncertaintyFeatures,
}

#[macro_export]
macro_rules! if_true {
    { true, $fragment:item } => { $fragment };
    { false, $fragment:item } => {};
}

#[macro_export]
macro_rules! if_some_uncertainty {
    { None, $fragment:item } => {};
    { Symmetric, $fragment:item } => { $fragment };
    { TwoSided, $fragment:item } => { $fragment };
}

#[macro_export]
macro_rules! if_symmetric_uncertainty {
    { None, $fragment:item } => {};
    { Symmetric, $fragment:item } => { $fragment };
    { TwoSided, $fragment:item } => {};
}

#[macro_export]
macro_rules! if_two_sided_uncertainty {
    { None, $fragment:item } => {};
    { Symmetric, $fragment:item } => {};
    { TwoSided, $fragment:item } => { $fragment };
}

#[macro_export]
macro_rules! define_measure_types {
    {
        MeasureFeatures {
            with_points: $with_points:tt,
            with_directions: $with_directions:tt,
            with_2d: $with_2d:tt,
            with_3d: $with_3d:tt,
            with_transformations: $with_transformations:tt,
            with_uncertainty: $with_uncertainty:tt,
        }
    } => {
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
        rs_measures::if_true! { $with_points,
            rs_measures::inner_define_measure_point! {}
        }
        rs_measures::if_true! { $with_directions,
            rs_measures::inner_define_unsigned_direction! {}
        }
        rs_measures::if_true! { $with_directions,
            rs_measures::inner_define_signed_direction! {}
        }
        rs_measures::if_true! { $with_2d,
            rs_measures::inner_define_measure_2d! { $with_points $with_directions }
        }
        rs_measures::if_true! { $with_2d,
            rs_measures::if_true! { $with_points,
                rs_measures::inner_define_measure_point_2d! {}
            }
        }
        rs_measures::if_true! { $with_3d,
            rs_measures::inner_define_measure_3d! {}
        }
        rs_measures::if_true! { $with_3d,
            rs_measures::if_true! { $with_points,
                rs_measures::inner_define_measure_point_3d! {}
            }
        }
        rs_measures::if_true! { $with_2d,
            rs_measures::if_true! { $with_transformations,
                rs_measures::inner_define_linear_map_2d! {}
            }
        }
        rs_measures::if_true! { $with_3d,
            rs_measures::if_true! { $with_transformations,
                rs_measures::inner_define_linear_map_3d! {}
            }
        }
        rs_measures::if_true! { $with_2d,
            rs_measures::if_true! { $with_transformations,
                rs_measures::if_true! { $with_points,
                    rs_measures::inner_define_affine_map_2d! {}
                }
            }
        }
        rs_measures::if_true! { $with_3d,
            rs_measures::if_true! { $with_transformations,
                rs_measures::if_true! { $with_points,
                    rs_measures::inner_define_affine_map_3d! {}
                }
            }
        }
    };
}
