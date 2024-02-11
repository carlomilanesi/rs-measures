// Conversion to use the library Rs-measures of benchmark The Computer Language Benchmarks Game
// https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-rust-6.html
//
// Build with:
//     cargo build --release --example nbody-measures
// And then run with:
//     /bin/time target/release/examples/nbody-measures 50000000
// It should print:
// -0.169075164 J
// -0.169059907 J

rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: true,
        with_directions: false,
        with_2d: false,
        with_3d: true,
        with_transformations: false,
        with_uncertainty: None,
    }
}

pub struct Length;
impl VectorProperty for Length {}

#[derive(Copy, Clone)]
pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

pub struct Area;

#[derive(Copy, Clone)]
pub struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Property = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m2";
}

pub struct Volume;

pub struct CubicMetre;
impl MeasurementUnit for CubicMetre {
    type Property = Volume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m3";
}

pub struct TimePerVolume;

pub struct SecondPerCubicMetre;
impl MeasurementUnit for SecondPerCubicMetre {
    type Property = TimePerVolume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s/m3";
}

pub struct Time;

#[derive(Copy, Clone)]
pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Velocity;
impl VectorProperty for Velocity {}

#[derive(Copy, Clone)]
pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

pub struct Mass;

pub struct KiloGram;
impl MeasurementUnit for KiloGram {
    type Property = Mass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg";
}

pub struct GravitationalConstant;

pub struct NewtonSquareMetrePerSquareKilogram;
impl MeasurementUnit for NewtonSquareMetrePerSquareKilogram {
    type Property = GravitationalConstant;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " G";
}

pub struct SquareVelocity;

pub struct SquareMetrePerSquareSecond;
impl MeasurementUnit for SquareMetrePerSquareSecond {
    type Property = SquareVelocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m2/s2";
}

pub struct SquareMass;

pub struct SquareKiloGram;
impl MeasurementUnit for SquareKiloGram {
    type Property = SquareMass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg2";
}

pub struct SquareMassPerLength;

pub struct SquareKiloGramPerMetre;
impl MeasurementUnit for SquareKiloGramPerMetre {
    type Property = SquareMassPerLength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km2/m";
}

pub struct TimePerArea;
impl VectorProperty for TimePerArea {}

#[derive(Copy, Clone)]
pub struct SecondPerSquareMetre;
impl MeasurementUnit for SecondPerSquareMetre {
    type Property = TimePerArea;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s/m2";
}

pub struct MassTimePerArea;
impl VectorProperty for MassTimePerArea {}

pub struct KiloGramSecondPerSquareMetre;
impl MeasurementUnit for KiloGramSecondPerSquareMetre {
    type Property = MassTimePerArea;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg s/m2";
}

rs_measures::define_units_relationship! {Metre:3 == MetrePerSecond:3 * Second}
rs_measures::define_units_relationship! {SquareMetre == Metre * =}
rs_measures::define_units_relationship! {SquareMetre == Metre:3 * =:3}
rs_measures::define_units_relationship! {CubicMetre == SquareMetre * Metre}
rs_measures::define_units_relationship! {Second == SecondPerCubicMetre * CubicMetre}
rs_measures::define_units_relationship! {SquareMetrePerSquareSecond == MetrePerSecond:3 * =:3}
rs_measures::define_units_relationship! {SquareKiloGram == KiloGram * =}
rs_measures::define_units_relationship! {Joule == KiloGram * SquareMetrePerSquareSecond}
rs_measures::define_units_relationship! {SquareKiloGram == SquareKiloGramPerMetre * Metre}
rs_measures::define_units_relationship! {Joule == NewtonSquareMetrePerSquareKilogram * SquareKiloGramPerMetre}
rs_measures::define_units_relationship! {SecondPerSquareMetre:3 == Metre:3 * SecondPerCubicMetre}
rs_measures::define_units_relationship! {KiloGramSecondPerSquareMetre:3 == SecondPerSquareMetre:3 * KiloGram}
rs_measures::define_units_relationship! {MetrePerSecond:3 == KiloGramSecondPerSquareMetre:3 * NewtonSquareMetrePerSquareKilogram}

use std::f64::consts::PI;

const SOLAR_MASS: Measure<KiloGram> = Measure::<KiloGram>::new(4.0 * PI * PI);
const DPY: f64 = 365.24;

pub struct Body {
    pub x: MeasurePoint3d<Metre>,
    pub mass: Measure<KiloGram>, // Putting `mass` here, improves the alignment.
    pub v: Measure3d<MetrePerSecond>,
}

const N_BODIES: usize = 5;
#[allow(clippy::excessive_precision)]
fn bodies() -> [Body; N_BODIES] {
    [
        // sun:
        Body {
            x: MeasurePoint3d::<Metre>::default(),
            v: Measure3d::<MetrePerSecond>::default(),
            mass: SOLAR_MASS,
        },
        // jupiter:
        Body {
            x: MeasurePoint3d::new(
                4.84143144246472090e+00,
                -1.16032004402742839e+00,
                -1.03622044471123109e-01,
            ),
            v: Measure3d::new(
                1.66007664274403694e-03 * DPY,
                7.69901118419740425e-03 * DPY,
                -6.90460016972063023e-05 * DPY,
            ),
            mass: 9.54791938424326609e-04 * SOLAR_MASS,
        },
        // saturn:
        Body {
            x: MeasurePoint3d::new(
                8.34336671824457987e+00,
                4.12479856412430479e+00,
                -4.03523417114321381e-01,
            ),
            v: Measure3d::new(
                -2.76742510726862411e-03 * DPY,
                4.99852801234917238e-03 * DPY,
                2.30417297573763929e-05 * DPY,
            ),
            mass: 2.85885980666130812e-04 * SOLAR_MASS,
        },
        // uranus:
        Body {
            x: MeasurePoint3d::new(
                1.28943695621391310e+01,
                -1.51111514016986312e+01,
                -2.23307578892655734e-01,
            ),
            v: Measure3d::new(
                2.96460137564761618e-03 * DPY,
                2.37847173959480950e-03 * DPY,
                -2.96589568540237556e-05 * DPY,
            ),
            mass: 4.36624404335156298e-05 * SOLAR_MASS,
        },
        // neptune:
        Body {
            x: MeasurePoint3d::new(
                1.53796971148509165e+01,
                -2.59193146099879641e+01,
                1.79258772950371181e-01,
            ),
            v: Measure3d::new(
                2.68067772490389322e-03 * DPY,
                1.62824170038242295e-03 * DPY,
                -9.51592254519715870e-05 * DPY,
            ),
            mass: 5.15138902046611451e-05 * SOLAR_MASS,
        },
    ]
}

pub fn offset_momentum(bodies: &mut [Body; N_BODIES]) {
    let (sun, rest) = bodies.split_at_mut(1);
    let sun = &mut sun[0];
    for body in rest {
        let m_ratio = body.mass / SOLAR_MASS;
        sun.v -= body.v * m_ratio;
    }
}

pub fn energy(bodies: &[Body; N_BODIES]) -> Measure<Joule> {
    let g = Measure::<NewtonSquareMetrePerSquareKilogram>::new(1.);
    let mut e = Measure::<Joule>::default();
    for i in 0..N_BODIES {
        let bi = &bodies[i];
        e += bi.mass * (bi.v * bi.v) * 0.5;
        for bj in &bodies[i + 1..] {
            let dx = bi.x - bj.x;
            e -= g * (bi.mass * bj.mass / (dx * dx).sqrt());
        }
    }
    e
}

pub fn advance(bodies: &mut [Body; N_BODIES], dt: Measure<Second>) {
    const N: usize = N_BODIES * (N_BODIES - 1) / 2;

    // compute distance between bodies:
    let mut r = [Measure3d::<Metre>::default(); N];
    {
        let mut i = 0;
        for j in 0..N_BODIES {
            for k in j + 1..N_BODIES {
                r[i] = bodies[j].x - bodies[k].x;
                i += 1;
            }
        }
    }

    let mut mag = [Measure::<SecondPerCubicMetre>::default(); N];
    for i in 0..N {
        let d1 = r[i] * r[i];
        mag[i] = dt / (d1 * d1.sqrt());
    }

    let g = Measure::<NewtonSquareMetrePerSquareKilogram>::new(1.);
    let mut i = 0;
    for j in 0..N_BODIES {
        for k in j + 1..N_BODIES {
            let f = r[i] * mag[i];
            bodies[j].v -= f * bodies[k].mass * g;
            bodies[k].v += f * bodies[j].mass * g;
            i += 1
        }
    }

    for body in bodies {
        body.x += body.v * dt;
    }
}

fn run(n: usize) -> (Measure<Joule>, Measure<Joule>) {
    let mut bodies = bodies();
    offset_momentum(&mut bodies);
    let energy_before = energy(&bodies);
    for _ in 0..n {
        advance(&mut bodies, Measure::<Second>::new(0.01));
    }
    let energy_after = energy(&bodies);
    (energy_before, energy_after)
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let (energy_before, energy_after) = run(n);
    println!("{:.9}\n{:.9}", energy_before, energy_after,);
}
