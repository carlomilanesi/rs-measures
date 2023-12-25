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
        assert_eq_tolerance!($actual as f32, $expected as f32, f32::EPSILON * 64.)
    };
}

#[macro_export]
macro_rules! assert_eq_64 {
    ($actual:expr, $expected:expr) => {
        assert_eq_tolerance!($actual as f64, $expected as f64, f64::EPSILON * 64.)
    };
}
