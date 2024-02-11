rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: false,
        with_directions: false,
        with_2d: false,
        with_3d: false,
        with_transformations: false,
        with_uncertainty: None,
    }
}

pub struct Length;
impl VectorProperty for Length {}

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Velocity;
impl VectorProperty for Velocity {}

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Area;

pub struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Property = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}";
}

/*
use rs_measures::define_units_relationship;

#[test]
fn test_u1_2_mul_u1_2_equals_u3() {
    //expand_1_2 ==
    // Measure2d<U1> * Measure2d<U1> -> Measure<U3>
    // Measure2d<U3>.sqr() -> Measure<U1>
    // Measure2d<U1>.cross_product(Measure2d<U1>) -> Measure<U3>

    define_units_relation! { Metre: 2 * Metre: 2 == SquareMetre }

    let distance1 = Measure2d::<Metre>::new(2., 3.);
    assert_eq!(distance1.to_string(), "(2, 3) m");
    let distance2 = Measure2d::<Metre>::new(4., 5.);
    assert_eq!(distance2.to_string(), "(4, 5) m");
    let area1: Measure<SquareMetre> = distance1 * distance2;
    assert_eq!(area1.to_string(), "23 m\u{b2}");
    let area2: Measure<SquareMetre> = distance1.squared();
    assert_eq!(area2.to_string(), "13 m\u{b2}");
    //let distance4: Measure::<Metre> = distance1.cross_product(distance2);
    //assert_eq!(distance4.to_string(), "-2 m");
}

#[test]
fn test_u1_mul_u2_2_equals_u3_2() {
    //expand_1_2 !=
    // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
    // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
    // Measure2d<U3> / Measure<U1> -> Measure2d<U2>

    define_units_relation! { Second * MetrePerSecond: 2 == Metre: 2 }

    /*
    let velocity1 = Measure::<MetrePerSecond>::new(2.);
    assert_eq!(velocity1.to_string(), "2 m/s");
    let interval1 = Measure::<Second>::new(5.);
    assert_eq!(interval1.to_string(), "5 s");
    let distance1: Measure::<Metre> = velocity1 * interval1;
    assert_eq!(distance1.to_string(), "10 m");
    let distance2: Measure::<Metre> = interval1 * velocity1;
    assert_eq!(distance2.to_string(), "10 m");
    let interval2: Measure::<Second> = distance1 / velocity1;
    assert_eq!(interval2.to_string(), "5 s");
    let interval3: Measure::<Second> = distance1 / velocity1;
    assert_eq!(interval3.to_string(), "5 s");
    */
}
*/

/*
//expand_1_1 ==
    // Measure<U1> * Measure<U1> -> Measure<U2>
    // Measure<U2> / Measure<U1> -> Measure<U1>
    // Measure<U1>.squared() -> Measure<U2>
    // Measure<U2>.sqrt() -> Measure<U1>
//!=
    // Measure<U1> * Measure<U2> -> Measure<U3>
    // Measure<U2> * Measure<U1> -> Measure<U3>
    // Measure<U3> / Measure<U1> -> Measure<U2>
    // Measure<U3> / Measure<U2> -> Measure<U1>

//expand_1_2 ==
    // Measure2d<U1> * Measure2d<U1> -> Measure<U2>
    // Measure2d<U2>.sqr() -> Measure<U1>
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
    // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
    // Measure2d<U3> / Measure<U1> -> Measure2d<U2>

//expand_1_3 ==
    // Measure3d<U1> * Measure3d<U1> -> Measure<U2>
    // Measure3d<U1>.sqr() -> Measure<U2>
    // Measure3d<U1>.cross_product(Measure2d<U1>) -> Measure3d<U3>
//!=
    // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
    // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
    // Measure3d<U3> / Measure<U1> -> Measure3d<U2>

// expand_inverse
    // Measure<U1> * Measure<U2> -> N
    // Measure<U2> * Measure<U1> -> N
    // N64 / Measure<U1> -> Measure<U2>
    // N32 / Measure<U1> -> Measure<U2>
    // N64 / Measure<U2> -> Measure<U1>
    // N32 / Measure<U2> -> Measure<U1>

// expand_2_2 ==
    // Measure2d<U1> * Measure2d<U1> -> Measure<U2>
    // Measure2d<U2>.sqr() -> Measure<U1>
//!=
    // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
    // Measure2d<U2> * Measure2d<U1> -> Measure<U3>

// expand_3_3 ==
    // Measure3d<U1> * Measure3d<U1> -> Measure<U2>
    // Measure3d<U1>.sqr() -> Measure<U2>
//!=
    // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
    // Measure3d<U2> * Measure3d<U1> -> Measure<U3>

// expand_cross_2 ==
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U3>
    // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U3>

// expand_cross_3 ==
    // Measure2<U1>.cross_product(Measure2<U1>) -> Measure<U3>
//!=
    // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
    // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
*/
