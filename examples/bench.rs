// Build and run with:
//     cargo run --release --example bench

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

pub struct Force;
impl VectorProperty for Force {}

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

rs_measures::define_units_relationship! {Joule == Newton * Metre}

const BENCH_MATRIX_SIZE: usize = 120;
const TINY_BENCH_MATRIX_SIZE: usize = 4;

#[allow(clippy::needless_range_loop)]
fn array_raw_matrix_multiplication() -> bool {
    let mat1 = [[0.; BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    let mat2 = [[0.; BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    let mut mat3 = [[0.; BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    for row in 0..BENCH_MATRIX_SIZE {
        for column in 0..BENCH_MATRIX_SIZE {
            for step in 0..BENCH_MATRIX_SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    mat3[BENCH_MATRIX_SIZE / 2][BENCH_MATRIX_SIZE / 2] >= 0.
}

#[allow(clippy::needless_range_loop)]
fn tiny_array_raw_matrix_multiplication() -> bool {
    let mat1 = [[0.; TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    let mat2 = [[0.; TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    let mut mat3 = [[0.; TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    for row in 0..TINY_BENCH_MATRIX_SIZE {
        for column in 0..TINY_BENCH_MATRIX_SIZE {
            for step in 0..TINY_BENCH_MATRIX_SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    mat3[TINY_BENCH_MATRIX_SIZE / 2][TINY_BENCH_MATRIX_SIZE / 2] >= 0.
}

extern crate nalgebra as na;
use na::{ArrayStorage, Matrix, Matrix4, U120};

fn nalgebra_raw_matrix_builtin_multiplication() -> bool {
    type M120 = Matrix<f64, U120, U120, ArrayStorage<f64, 120, 120>>;
    let mat1 = M120::zeros();
    let mat2 = M120::zeros();
    let mat3 = mat1 * mat2;
    mat3[(BENCH_MATRIX_SIZE / 2, BENCH_MATRIX_SIZE / 2)] >= 0.
}

fn tiny_nalgebra_raw_matrix_builtin_multiplication() -> bool {
    type M4 = Matrix4<f64>;
    let mat1 = M4::zeros();
    let mat2 = M4::zeros();
    let mat3 = mat1 * mat2;
    mat3[(TINY_BENCH_MATRIX_SIZE / 2, TINY_BENCH_MATRIX_SIZE / 2)] >= 0.
}

fn nalgebra_raw_matrix_explicit_multiplication() -> bool {
    type M120 = Matrix<f64, U120, U120, ArrayStorage<f64, 120, 120>>;
    let mat1 = M120::zeros();
    let mat2 = M120::zeros();
    let mut mat3 = M120::zeros();
    for row in 0..BENCH_MATRIX_SIZE {
        for column in 0..BENCH_MATRIX_SIZE {
            for step in 0..BENCH_MATRIX_SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    mat3[(BENCH_MATRIX_SIZE / 2, BENCH_MATRIX_SIZE / 2)] >= 0.
}

fn tiny_nalgebra_raw_matrix_explicit_multiplication() -> bool {
    type M4 = Matrix4<f64>;
    let mat1 = M4::zeros();
    let mat2 = M4::zeros();
    let mut mat3 = M4::zeros();
    for row in 0..TINY_BENCH_MATRIX_SIZE {
        for column in 0..TINY_BENCH_MATRIX_SIZE {
            for step in 0..TINY_BENCH_MATRIX_SIZE {
                mat3[(row, column)] += mat1[(row, step)] * mat2[(step, column)];
            }
        }
    }
    mat3[(TINY_BENCH_MATRIX_SIZE / 2, TINY_BENCH_MATRIX_SIZE / 2)] >= 0.
}

#[allow(clippy::needless_range_loop)]
fn array_measure_matrix_multiplication() -> bool {
    let mat1 = [[Measure::<Newton>::new(0.); BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    let mat2 = [[Measure::<Metre>::new(0.); BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    let mut mat3 = [[Measure::<Joule>::new(0.); BENCH_MATRIX_SIZE]; BENCH_MATRIX_SIZE];
    for row in 0..BENCH_MATRIX_SIZE {
        for column in 0..BENCH_MATRIX_SIZE {
            for step in 0..BENCH_MATRIX_SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    mat3[BENCH_MATRIX_SIZE / 2][BENCH_MATRIX_SIZE / 2].value >= 0.
}

#[allow(clippy::needless_range_loop)]
fn tiny_array_measure_matrix_multiplication() -> bool {
    let mat1 = [[Measure::<Newton>::new(0.); TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    let mat2 = [[Measure::<Metre>::new(0.); TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    let mut mat3 = [[Measure::<Joule>::new(0.); TINY_BENCH_MATRIX_SIZE]; TINY_BENCH_MATRIX_SIZE];
    for row in 0..TINY_BENCH_MATRIX_SIZE {
        for column in 0..TINY_BENCH_MATRIX_SIZE {
            for step in 0..TINY_BENCH_MATRIX_SIZE {
                mat3[row][column] += mat1[row][step] * mat2[step][column];
            }
        }
    }
    mat3[TINY_BENCH_MATRIX_SIZE / 2][TINY_BENCH_MATRIX_SIZE / 2].value >= 0.
}

fn main() {
    let start = std::time::Instant::now();
    if !array_raw_matrix_multiplication() {
        return;
    }
    let array_raw_duration = start.elapsed().as_micros();

    let start = std::time::Instant::now();
    if !nalgebra_raw_matrix_builtin_multiplication() {
        return;
    }
    let nalgebra_raw_builtin_duration = start.elapsed().as_micros();

    let start = std::time::Instant::now();
    if !nalgebra_raw_matrix_explicit_multiplication() {
        return;
    }
    let nalgebra_raw_explicit_duration = start.elapsed().as_micros();

    let start = std::time::Instant::now();
    if !array_measure_matrix_multiplication() {
        return;
    }
    let array_measure_duration = start.elapsed().as_micros();

    let start = std::time::Instant::now();
    if !tiny_array_raw_matrix_multiplication() {
        return;
    }
    let tiny_array_raw_duration = start.elapsed().as_nanos();

    let start = std::time::Instant::now();
    if !tiny_nalgebra_raw_matrix_builtin_multiplication() {
        return;
    }
    let tiny_nalgebra_raw_builtin_duration = start.elapsed().as_nanos();

    let start = std::time::Instant::now();
    if !tiny_nalgebra_raw_matrix_explicit_multiplication() {
        return;
    }
    let tiny_nalgebra_raw_explicit_duration = start.elapsed().as_nanos();

    let start = std::time::Instant::now();
    if !tiny_array_measure_matrix_multiplication() {
        return;
    }
    let tiny_array_measure_duration = start.elapsed().as_nanos();

    println!("array raw: {array_raw_duration}.");
    println!("nalgebra raw builtin: {nalgebra_raw_builtin_duration}.");
    println!("nalgebra raw explicit: {nalgebra_raw_explicit_duration}.");
    println!("array measure: {array_measure_duration}.");

    println!("tiny array raw: {tiny_array_raw_duration}.");
    println!("tiny nalgebra raw builtin: {tiny_nalgebra_raw_builtin_duration}.");
    println!("tiny nalgebra raw explicit: {tiny_nalgebra_raw_explicit_duration}.");
    println!("tiny array measure: {tiny_array_measure_duration}.");
}
