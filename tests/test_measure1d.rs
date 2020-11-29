#[test]
fn measure_value() {
    /*
    use rs_measures::{
        angle::{Angle, Radian},
        define_derived_measure_1_1, define_derived_measure_1_2, define_derived_measure_1_3,
        define_derived_measure_2_2, define_derived_measure_3_3, define_derived_measure_inverse_1,
        define_derived_measure_squared_1, define_derived_measure_squared_2,
        define_derived_measure_squared_3, define_measure1d, define_measure2d, define_measure3d,
        traits::{AngleMeasurementUnit, CrossProduct, Sqrt},
    };
    */
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }

    let m = Measure::<f32, Metre>::new(1.23);
    assert_eq!(1.23, m.value);
}

mod angles_decl;

#[test]
fn angle_units_consts() {
    use rs_measures::{angle::Radian, define_measure1d, traits::AngleMeasurementUnit};
    define_measure1d! {}
    assert_eq!(360., angles_decl::Degree::TURN_FRACTION);
    assert_eq!(2. * 3.141592653589793238462643383279, Radian::TURN_FRACTION);
    assert_eq!(1., angles_decl::Turn::TURN_FRACTION);
}

/*
Used Macros
EXPECT_FALSE
EXPECT_TRUE
EXPECT_EQ
EQUAL
EXPECT_FLOAT_EQ
*/
#[test]
fn measures_addition() {
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    #[derive(Debug, PartialEq)]
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }
    assert_eq!(
        19.,
        (Measure::<f32, Metre>::new(12.) + Measure::<f32, Metre>::new(7.)).value
    );
}

#[test]
fn measures_subtraction() {
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    #[derive(Debug, PartialEq)]
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }
    assert_eq!(
        5.,
        (Measure::<f32, Metre>::new(12.) - Measure::<f32, Metre>::new(7.)).value
    );
}

#[test]
fn measure_scalar_multiplication() {
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    #[derive(Debug, PartialEq)]
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }
    assert_eq!(36., (Measure::<f32, Metre>::new(12.) * 3.).value);
}

#[test]
fn measure_scalar_division() {
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    #[derive(Debug, PartialEq)]
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }
    assert_eq!(4., (Measure::<f32, Metre>::new(12.) / 3.).value);
}

#[test]
fn measure_scalar_division_assignment() {
    use rs_measures::define_measure1d;
    define_measure1d! {}

    struct Length;
    #[derive(Debug, PartialEq)]
    struct Metre;
    impl MeasurementUnit for Metre {
        type Quantity = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }
    let mut m = Measure::<f32, Metre>::new(12.);
    m /= 3.;
    assert_eq!(4., m.value);
}
