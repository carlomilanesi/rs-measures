#[macro_export]
macro_rules! assert_eq_tolerance {
    ($actual:expr, $expected:expr, $tolerance:expr) => {
        let a = $actual;
        let e = $expected;
        let abs_e = if e < 0. { -e } else { e };
        let t = $tolerance * if abs_e < 1. { 1. } else { abs_e };
        if (e - a).abs() > t {
            panic!(
                "Actual: {}, Expected: {}, Difference: {}, Tolerance: {}.",
                a,
                e,
                a - e,
                t,
            );
        }
    };
}

#[macro_export]
macro_rules! assert_eq_32 {
    ($actual:expr, $expected:expr) => {
        assert_eq_tolerance!($actual, $expected, f32::EPSILON * 8.)
    };
}

#[macro_export]
macro_rules! assert_eq_64 {
    ($actual:expr, $expected:expr) => {
        assert_eq_tolerance!($actual, $expected, f64::EPSILON * 8.)
    };
}
