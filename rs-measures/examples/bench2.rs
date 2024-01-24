// To run this, type (where 10000 is the size of the data set; the default is 100):
// cargo run --release --example bench2 10000

use rand::{rngs::StdRng, Rng, SeedableRng};

rs_measures::define_1d_3d! {}

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

pub struct NanoSecond;
impl MeasurementUnit for NanoSecond {
    type Property = Time;
    const RATIO: f64 = 1.0e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ns";
}

fn main() {
    const MAX_DATA_SIZE: usize = 100_000_000; // items, using 2.4 GB
    let mut args = std::env::args();
    args.next();
    let mut n_data_size = 100;
    if let Some(arg) = args.next() {
        if let Ok(n) = arg.parse::<usize>() {
            if n > MAX_DATA_SIZE {
                println!("Warning: {arg} is too big as data size.");
                n_data_size = MAX_DATA_SIZE;
            } else {
                n_data_size = n;
            }
        } else {
            println!("Warning: {arg} is not allowed as data size.");
        }
    } else {
        println!("No data size specified.");
    }
    println!("Using {n_data_size} items.");

    use_naked_numbers(n_data_size);
    use_rs_measures(n_data_size);
}

fn use_rs_measures(n_data_size: usize) {
    let mut mp1 = Vec::<MeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut mp2 = Vec::<MeasurePoint<Metre>>::with_capacity(n_data_size);
    let mut m1 = Vec::<Measure<Metre>>::with_capacity(n_data_size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    for _ in 0..n_data_size {
        mp1.push(MeasurePoint::<Metre>::new(rng.gen::<f64>()));
        mp2.push(MeasurePoint::<Metre>::new(rng.gen::<f64>()));
        m1.push(Measure::<Metre>::new(rng.gen::<f64>()));
    }

    let start = std::time::Instant::now();
    for i in 0..n_data_size {
        let r1 = mp1[i] + m1[i] * 3.1 - mp2[i];
        let r2 = mp1[i] - (mp2[i] - 3.1 * m1[i]);
        let r = r1 - r2;
        if r.value.abs() > 1.0e-15 {
            println!("{r1} {r2} {r}");
        }
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using Measure<Metre, f64>: {}.",
        duration / n_data_size as f64
    );

    let start = std::time::Instant::now();
    for i in 0..n_data_size {
        let r1 = mp1[i].value + m1[i].value * 3.1 - mp2[i].value;
        let r2 = mp1[i].value - (mp2[i].value - 3.1 * m1[i].value);
        let r = r1 - r2;
        if r.abs() > 1.0e-15 {
            println!("{r1} {r2} {r}");
        }
    }
    let duration = Measure::<NanoSecond>::new(start.elapsed().as_nanos() as f64);
    println!(
        "Time per item using Measure<Metre, f64>.value: {}.",
        duration / n_data_size as f64
    );
}

fn use_naked_numbers(n_data_size: usize) {
    let mut mp1 = Vec::<f64>::with_capacity(n_data_size);
    let mut mp2 = Vec::<f64>::with_capacity(n_data_size);
    let mut m1 = Vec::<f64>::with_capacity(n_data_size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);

    for _ in 0..n_data_size {
        mp1.push(rng.gen::<f64>());
        mp2.push(rng.gen::<f64>());
        m1.push(rng.gen::<f64>());
    }

    let start = std::time::Instant::now();
    for i in 0..n_data_size {
        let r1 = mp1[i] + m1[i] * 3.1 - mp2[i];
        let r2 = mp1[i] - (mp2[i] - 3.1 * m1[i]);
        let r = r1 - r2;
        if r.abs() > 1.0e-15 {
            println!("{r1} {r2} {r}");
        }
    }
    let duration = Measure::<NanoSecond, f32>::new(start.elapsed().as_nanos() as f32);
    println!(
        "Time per item using directly f64: {}.",
        duration / n_data_size as f32
    );
}
