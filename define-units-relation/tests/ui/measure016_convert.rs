rs_measures::define_measure_1d! {}
struct Mass;
struct KiloGram;
impl MeasurementUnit for KiloGram {
    type Property = Mass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg";
}
struct Time;
struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}
struct Hour;
impl MeasurementUnit for Hour {
    type Property = Time;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h";
}
fn main() {
    let m = Measure::<f32, Second>::new(1.);
    let _ = m.convert::<KiloGram>();
}
