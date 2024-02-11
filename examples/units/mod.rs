rs_measures::define_measure_types! {
    MeasureFeatures {
        with_points: true,
        with_directions: true,
        with_2d: true,
        with_3d: true,
        with_transformations: false,
        with_uncertainty: None,
    }
}

// Property: acceleration
pub struct Acceleration;
impl VectorProperty for Acceleration {}

pub struct MetrePerSquareSecond;
impl MeasurementUnit for MetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

pub struct CentiMetrePerSquareSecond;
impl MeasurementUnit for CentiMetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1e-2;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

pub struct GForce;
impl MeasurementUnit for GForce {
    type Property = Acceleration;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

pub struct KiloMetrePerHourPerSecond;
impl MeasurementUnit for KiloMetrePerHourPerSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h/s";
}

// Property: action
pub struct Action;

pub struct JouleSecond;
impl MeasurementUnit for JouleSecond {
    type Property = Action;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J\u{b7}s";
}

// Property: amount of substance, count
pub struct Amount;

pub struct Unit;
impl MeasurementUnit for Unit {
    type Property = Amount;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u.";
}

pub struct Dozen;
impl MeasurementUnit for Dozen {
    type Property = Amount;
    const RATIO: f64 = 12.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dz.";
}

pub struct Mole;
impl MeasurementUnit for Mole {
    type Property = Amount;
    const RATIO: f64 = 6.0221413e23;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol";
}

// Property: angle
pub struct Cycle;
impl MeasurementUnit for Cycle {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cycles";
}
impl AngleMeasurementUnit for Cycle {
    const CYCLE_FRACTION: f64 = 1.;
}

pub struct Gradian;
impl MeasurementUnit for Gradian {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const CYCLE_FRACTION: f64 = 400.;
}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

pub struct ArcMinute;
impl MeasurementUnit for ArcMinute {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg'";
}
impl AngleMeasurementUnit for ArcMinute {
    const CYCLE_FRACTION: f64 = 360. * 60.;
}

pub struct ArcSecond;
impl MeasurementUnit for ArcSecond {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360. / 60. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\"";
}
impl AngleMeasurementUnit for ArcSecond {
    const CYCLE_FRACTION: f64 = 360. * 60. * 60.;
}

// Property: angular acceleration
pub struct AngularAcceleration;

pub struct RadianPerSquareSecond;
impl MeasurementUnit for RadianPerSquareSecond {
    type Property = AngularAcceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s\u{b2}";
}

// Property: angular momentum, spin
pub struct AngularMomentum;
impl VectorProperty for AngularMomentum {}

pub struct KiloGramSquareMetrePerSecond;
impl MeasurementUnit for KiloGramSquareMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}/s";
}

pub struct GramSquareCentiMetrePerSecond;
impl MeasurementUnit for GramSquareCentiMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}/s";
}

// Property: area
pub struct Area;

pub struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Property = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}";
}

pub struct SquareKiloMetre;
impl MeasurementUnit for SquareKiloMetre {
    type Property = Area;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{b2}";
}

pub struct Hectare;
impl MeasurementUnit for Hectare {
    type Property = Area;
    const RATIO: f64 = 1e4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ha";
}

pub struct Are;
impl MeasurementUnit for Are {
    type Property = Area;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " are";
}

pub struct SquareDeciMetre;
impl MeasurementUnit for SquareDeciMetre {
    type Property = Area;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm\u{b2}";
}

pub struct SquareCentiMetre;
impl MeasurementUnit for SquareCentiMetre {
    type Property = Area;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b2}";
}

pub struct SquareMilliMetre;
impl MeasurementUnit for SquareMilliMetre {
    type Property = Area;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm\u{b2}";
}

pub struct SquareMicroMetre;
impl MeasurementUnit for SquareMicroMetre {
    type Property = Area;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m\u{b2}";
}

pub struct SquareNanoMetre;
impl MeasurementUnit for SquareNanoMetre {
    type Property = Area;
    const RATIO: f64 = 1e-18;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm\u{b2}";
}

pub struct SquareInch;
impl MeasurementUnit for SquareInch {
    type Property = Area;
    const RATIO: f64 = 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{b2}";
}

pub struct SquareFoot;
impl MeasurementUnit for SquareFoot {
    type Property = Area;
    const RATIO: f64 = 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{b2}";
}

pub struct SquareYard;
impl MeasurementUnit for SquareYard {
    type Property = Area;
    const RATIO: f64 = 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{b2}";
}

pub struct SquareMile;
impl MeasurementUnit for SquareMile {
    type Property = Area;
    const RATIO: f64 = 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{b2}";
}

// Property: capacitance
pub struct Capacitance;

pub struct Farad;
impl MeasurementUnit for Farad {
    type Property = Capacitance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F";
}

pub struct MilliFarad;
impl MeasurementUnit for MilliFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mF";
}

pub struct MicroFarad;
impl MeasurementUnit for MicroFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}F";
}

pub struct NanoFarad;
impl MeasurementUnit for NanoFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nF";
}

pub struct PicoFarad;
impl MeasurementUnit for PicoFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pF";
}

// Property: catalytic activity
pub struct CatalyticActivity;

pub struct Katal;
impl MeasurementUnit for Katal {
    type Property = CatalyticActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kat";
}

// Property: chemical potential, molar energy
pub struct ChemicalPotential;

pub struct JoulePerMole;
impl MeasurementUnit for JoulePerMole {
    type Property = ChemicalPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/mol";
}

// Property: current density
pub struct CurrentDensity;
impl VectorProperty for CurrentDensity {}

pub struct AmperePerSquareMetre;
impl MeasurementUnit for AmperePerSquareMetre {
    type Property = CurrentDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m\u{b2}";
}

// Property: dimensionless
pub struct Dimensionless;
impl VectorProperty for Dimensionless {}

pub struct Unspecified;
impl MeasurementUnit for Unspecified {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}

pub struct Mach;
impl MeasurementUnit for Mach {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mach";
}

// Property: dose equivalent
pub struct DoseEquivalent;

pub struct Sievert;
impl MeasurementUnit for Sievert {
    type Property = DoseEquivalent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Sv";
}

pub struct Rem;
impl MeasurementUnit for Rem {
    type Property = DoseEquivalent;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rem";
}

// Property: dynamic viscosity, absolute viscosity
pub struct DynamicViscosity;

pub struct PascalSecond;
impl MeasurementUnit for PascalSecond {
    type Property = DynamicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa\u{b7}s";
}

// Property: electrical conductance, electric susceptance, electric admittance
pub struct ElectricalConductance;

pub struct Siemens;
impl MeasurementUnit for Siemens {
    type Property = ElectricalConductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S";
}

// Property: electrical conductivity
pub struct ElectricalConductivity;

pub struct SiemensPerMetre;
impl MeasurementUnit for SiemensPerMetre {
    type Property = ElectricalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S/m";
}

// Property: electrical resistance, electrical impedance
pub struct ElectricalResistance;

pub struct Ohm;
impl MeasurementUnit for Ohm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}";
}

pub struct MilliOhm;
impl MeasurementUnit for MilliOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{3a9}";
}

pub struct KiloOhm;
impl MeasurementUnit for KiloOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " k\u{3a9}";
}

// Property: electrical resistivity
pub struct ElectricalResistivity;

pub struct OhmMetre;
impl MeasurementUnit for OhmMetre {
    type Property = ElectricalResistivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}\u{b7}m";
}

// Property: electric charge
pub struct ElectricCharge;

pub struct Coulomb;
impl MeasurementUnit for Coulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C";
}

pub struct MilliCoulomb;
impl MeasurementUnit for MilliCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mC";
}

pub struct MicroCoulomb;
impl MeasurementUnit for MicroCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}C";
}

pub struct NanoCoulomb;
impl MeasurementUnit for NanoCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nC";
}

pub struct PicoCoulomb;
impl MeasurementUnit for PicoCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pC";
}

// Property: electric charge density
pub struct ElectricChargeDensity;

pub struct CoulombPerCubicMetre;
impl MeasurementUnit for CoulombPerCubicMetre {
    type Property = ElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{b3}";
}

// Property: electric current
pub struct ElectricCurrent;

pub struct Ampere;
impl MeasurementUnit for Ampere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A";
}

pub struct MilliAmpere;
impl MeasurementUnit for MilliAmpere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mA";
}

pub struct MicroAmpere;
impl MeasurementUnit for MicroAmpere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}A";
}

// Property: electric displacement, surface electric charge density
pub struct ElectricDisplacement;

pub struct CoulombPerSquareMetre;
impl MeasurementUnit for CoulombPerSquareMetre {
    type Property = ElectricDisplacement;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{b2}";
}

// Property: electric field strength
pub struct ElectricFieldStrength;
impl VectorProperty for ElectricFieldStrength {}

pub struct VoltPerMetre;
impl MeasurementUnit for VoltPerMetre {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V/m";
}

pub struct NewtonPerCoulomb;
impl MeasurementUnit for NewtonPerCoulomb {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N/C";
}

// Property: electric potential
pub struct ElectricPotential;

pub struct Volt;
impl MeasurementUnit for Volt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V";
}

pub struct KiloVolt;
impl MeasurementUnit for KiloVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kV";
}

pub struct MilliVolt;
impl MeasurementUnit for MilliVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mV";
}

pub struct MicroVolt;
impl MeasurementUnit for MicroVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}V";
}

// Property: energy, work, heat
pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

pub struct Erg;
impl MeasurementUnit for Erg {
    type Property = Energy;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " erg";
}

pub struct WattHour;
impl MeasurementUnit for WattHour {
    type Property = Energy;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W\u{b7}h";
}

pub struct KiloWattHour;
impl MeasurementUnit for KiloWattHour {
    type Property = Energy;
    const RATIO: f64 = 3.6e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kW\u{b7}h";
}

pub struct MegaWattHour;
impl MeasurementUnit for MegaWattHour {
    type Property = Energy;
    const RATIO: f64 = 3.6e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW\u{b7}h";
}

pub struct Calorie;
impl MeasurementUnit for Calorie {
    type Property = Energy;
    const RATIO: f64 = 4.187;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cal";
}

pub struct KiloCalorie;
impl MeasurementUnit for KiloCalorie {
    type Property = Energy;
    const RATIO: f64 = 4187.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kcal";
}

pub struct ElectronVolt;
impl MeasurementUnit for ElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-19;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " eV";
}

pub struct KiloElectronVolt;
impl MeasurementUnit for KiloElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " keV";
}

pub struct MegaElectronVolt;
impl MeasurementUnit for MegaElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-13;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MeV";
}

pub struct GigaElectronVolt;
impl MeasurementUnit for GigaElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GeV";
}

pub struct TeraElectronVolt;
impl MeasurementUnit for TeraElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TeV";
}

// Property: energy density
pub struct EnergyDensity;

pub struct JoulePerCubicMetre;
impl MeasurementUnit for JoulePerCubicMetre {
    type Property = EnergyDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{b3}";
}

// Property: entropy, heat capacity
pub struct Entropy;

pub struct JoulePerKelvin;
impl MeasurementUnit for JoulePerKelvin {
    type Property = Entropy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{b0}K";
}

// Property: force, weight
pub struct Force;
impl VectorProperty for Force {}

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

pub struct Dyne;
impl MeasurementUnit for Dyne {
    type Property = Force;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn";
}

pub struct KiloGramForce;
impl MeasurementUnit for KiloGramForce {
    type Property = Force;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kgf";
}

pub struct PoundForce;
impl MeasurementUnit for PoundForce {
    type Property = Force;
    const RATIO: f64 = 4.448222;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf";
}

pub struct Poundal;
impl MeasurementUnit for Poundal {
    type Property = Force;
    const RATIO: f64 = 0.138255;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pdl";
}

// Property: frequency, angular speed, angular velocity
pub struct Frequency;

pub struct Hertz;
impl MeasurementUnit for Hertz {
    type Property = Frequency;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Hz";
}

pub struct CyclePerSecond;
impl MeasurementUnit for CyclePerSecond {
    type Property = Frequency;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " c/s";
}

pub struct KiloHertz;
impl MeasurementUnit for KiloHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kHz";
}

pub struct MegaHertz;
impl MeasurementUnit for MegaHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MHz";
}

pub struct GigaHertz;
impl MeasurementUnit for GigaHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GHz";
}

pub struct RadianPerSecond;
impl MeasurementUnit for RadianPerSecond {
    type Property = Frequency;
    const RATIO: f64 = 1. / core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s";
}

pub struct CyclePerMinute;
impl MeasurementUnit for CyclePerMinute {
    type Property = Frequency;
    const RATIO: f64 = 1. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rpm";
}

pub struct Illuminance;

pub struct Lux;
impl MeasurementUnit for Lux {
    type Property = Illuminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lx";
}

pub struct Phot;
impl MeasurementUnit for Phot {
    type Property = Illuminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " phot";
}

pub struct FootCandle;
impl MeasurementUnit for FootCandle {
    type Property = Illuminance;
    const RATIO: f64 = 10.764;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fc";
}

// Property: inductance
pub struct Inductance;

pub struct Henry;
impl MeasurementUnit for Henry {
    type Property = Inductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H";
}

// Property: information
pub struct Information;

pub struct Bit;
impl MeasurementUnit for Bit {
    type Property = Information;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b";
}

pub struct Byte;
impl MeasurementUnit for Byte {
    type Property = Information;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B";
}

pub struct KiloBit;
impl MeasurementUnit for KiloBit {
    type Property = Information;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb";
}

pub struct KiloByte;
impl MeasurementUnit for KiloByte {
    type Property = Information;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB";
}

pub struct KibiBit;
impl MeasurementUnit for KibiBit {
    type Property = Information;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib";
}

pub struct KibiByte;
impl MeasurementUnit for KibiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB";
}

pub struct MegaBit;
impl MeasurementUnit for MegaBit {
    type Property = Information;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb";
}

pub struct MegaByte;
impl MeasurementUnit for MegaByte {
    type Property = Information;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB";
}

pub struct MebiBit;
impl MeasurementUnit for MebiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib";
}

pub struct MebiByte;
impl MeasurementUnit for MebiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB";
}

pub struct GigaBit;
impl MeasurementUnit for GigaBit {
    type Property = Information;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb";
}

pub struct GigaByte;
impl MeasurementUnit for GigaByte {
    type Property = Information;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB";
}

pub struct GibiBit;
impl MeasurementUnit for GibiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib";
}

pub struct GibiByte;
impl MeasurementUnit for GibiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB";
}

pub struct TeraBit;
impl MeasurementUnit for TeraBit {
    type Property = Information;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb";
}

pub struct TeraByte;
impl MeasurementUnit for TeraByte {
    type Property = Information;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB";
}

pub struct TebiBit;
impl MeasurementUnit for TebiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib";
}

pub struct TebiByte;
impl MeasurementUnit for TebiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB";
}

// Property: information rate
pub struct InformationRate;

pub struct BitPerSecond;
impl MeasurementUnit for BitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b/s";
}

pub struct BytePerSecond;
impl MeasurementUnit for BytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B/s";
}

pub struct KiloBitPerSecond;
impl MeasurementUnit for KiloBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb/s";
}

pub struct KiloBytePerSecond;
impl MeasurementUnit for KiloBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB/s";
}

pub struct KibiBitPerSecond;
impl MeasurementUnit for KibiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib/s";
}

pub struct KibiBytePerSecond;
impl MeasurementUnit for KibiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB/s";
}

pub struct MegaBitPerSecond;
impl MeasurementUnit for MegaBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb/s";
}

pub struct MegaBytePerSecond;
impl MeasurementUnit for MegaBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB/s";
}

pub struct MebiBitPerSecond;
impl MeasurementUnit for MebiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib/s";
}

pub struct MebiBytePerSecond;
impl MeasurementUnit for MebiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB/s";
}

pub struct GigaBitPerSecond;
impl MeasurementUnit for GigaBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb/s";
}

pub struct GigaBytePerSecond;
impl MeasurementUnit for GigaBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB/s";
}

pub struct GibiBitPerSecond;
impl MeasurementUnit for GibiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib/s";
}

pub struct GibiBytePerSecond;
impl MeasurementUnit for GibiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB/s";
}

pub struct TeraBitPerSecond;
impl MeasurementUnit for TeraBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb/s";
}

pub struct TeraBytePerSecond;
impl MeasurementUnit for TeraBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB/s";
}

pub struct TebiBitPerSecond;
impl MeasurementUnit for TebiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib/s";
}

pub struct TebiBytePerSecond;
impl MeasurementUnit for TebiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB/s";
}

// Property: irradiance, heat flux density
pub struct Irradiance;

pub struct WattPerSquareMetre;
impl MeasurementUnit for WattPerSquareMetre {
    type Property = Irradiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}";
}

// Property: kinematic viscosity
pub struct KinematicViscosity;

pub struct SquareMetrePerSecond;
impl MeasurementUnit for SquareMetrePerSecond {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}/s";
}

pub struct Stoke;
impl MeasurementUnit for Stoke {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " St";
}

pub struct CentiStoke;
impl MeasurementUnit for CentiStoke {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cSt";
}

// Property: length, width, height, depth, space, wavelength
pub struct Length;
impl VectorProperty for Length {}

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

pub struct AstronomicalUnit;
impl MeasurementUnit for AstronomicalUnit {
    type Property = Length;
    const RATIO: f64 = 149597870691.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " a.u.";
}

pub struct Parsec;
impl MeasurementUnit for Parsec {
    type Property = Length;
    const RATIO: f64 = 3.0856775813e16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " psc";
}

pub struct LightYear;
impl MeasurementUnit for LightYear {
    type Property = Length;
    const RATIO: f64 = 31557600. * 2.99792458e8;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ly";
}

pub struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Property = Length;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}

pub struct HectoMetre;
impl MeasurementUnit for HectoMetre {
    type Property = Length;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hm";
}

pub struct DecaMetre;
impl MeasurementUnit for DecaMetre {
    type Property = Length;
    const RATIO: f64 = 10.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dam";
}

pub struct DeciMetre;
impl MeasurementUnit for DeciMetre {
    type Property = Length;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm";
}

pub struct CentiMetre;
impl MeasurementUnit for CentiMetre {
    type Property = Length;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm";
}

pub struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

pub struct MicroMetre;
impl MeasurementUnit for MicroMetre {
    type Property = Length;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m";
}

pub struct NanoMetre;
impl MeasurementUnit for NanoMetre {
    type Property = Length;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm";
}

pub struct Angstrom;
impl MeasurementUnit for Angstrom {
    type Property = Length;
    const RATIO: f64 = 1e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{212b}";
}

pub struct Inch;
impl MeasurementUnit for Inch {
    type Property = Length;
    const RATIO: f64 = 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in";
}

pub struct Foot;
impl MeasurementUnit for Foot {
    type Property = Length;
    const RATIO: f64 = 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft";
}

pub struct Yard;
impl MeasurementUnit for Yard {
    type Property = Length;
    const RATIO: f64 = 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd";
}

pub struct Mile;
impl MeasurementUnit for Mile {
    type Property = Length;
    const RATIO: f64 = 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi";
}

pub struct NauticalMile;
impl MeasurementUnit for NauticalMile {
    type Property = Length;
    const RATIO: f64 = 1852.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " naut.mi";
}

// Property: linear density
pub struct LinearDensity;

pub struct KiloGramPerMetre;
impl MeasurementUnit for KiloGramPerMetre {
    type Property = LinearDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m";
}

pub struct GramPerCentiMetre;
impl MeasurementUnit for GramPerCentiMetre {
    type Property = LinearDensity;
    const RATIO: f64 = 1e-1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/cm";
}

// Property: linear electric charge density
pub struct LinearElectricChargeDensity;

pub struct CoulombPerMetre;
impl MeasurementUnit for CoulombPerMetre {
    type Property = LinearElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m";
}

// Property: luminance
pub struct Luminance;

pub struct CandelaPerSquareMetre;
impl MeasurementUnit for CandelaPerSquareMetre {
    type Property = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd/m\u{b2}";
}

pub struct Nit;
impl MeasurementUnit for Nit {
    type Property = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nt";
}

pub struct Stilb;
impl MeasurementUnit for Stilb {
    type Property = Luminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " stilb";
}

pub struct CandelaPerSquareFoot;
impl MeasurementUnit for CandelaPerSquareFoot {
    type Property = Luminance;
    const RATIO: f64 = 10.764;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " stilb";
}

// Property: luminous flux, luminous power
pub struct LuminousFlux;
impl VectorProperty for LuminousFlux {}

pub struct Lumen;
impl MeasurementUnit for Lumen {
    type Property = LuminousFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lm";
}

// Property: luminous intensity
pub struct LuminousIntensity;

pub struct Candela;
impl MeasurementUnit for Candela {
    type Property = LuminousIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd";
}

// Property: magnetic field strength, magnetic field intensity, magnetization
pub struct MagneticFieldStrength;
impl VectorProperty for MagneticFieldStrength {}

pub struct AmperePerMetre;
impl MeasurementUnit for AmperePerMetre {
    type Property = MagneticFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m";
}

// Property: magnetic flux
pub struct MagneticFlux;

pub struct Weber;
impl MeasurementUnit for Weber {
    type Property = MagneticFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Wb";
}

// Property: magnetic flux density
pub struct MagneticFluxDensity;
impl VectorProperty for MagneticFluxDensity {}

pub struct Tesla;
impl MeasurementUnit for Tesla {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " T";
}

pub struct Gauss;
impl MeasurementUnit for Gauss {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " G";
}

// Property: magnetic permeability
pub struct MagneticPermeability;

pub struct HenryPerMetre;
impl MeasurementUnit for HenryPerMetre {
    type Property = MagneticPermeability;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H/m";
}

// Property: magnetic reluctance, magnetic resistance
pub struct MagneticReluctance;

pub struct InverseHenry;
impl MeasurementUnit for InverseHenry {
    type Property = MagneticReluctance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/H";
}

// Property: mass
pub struct Mass;

pub struct KiloGram;
impl MeasurementUnit for KiloGram {
    type Property = Mass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg";
}

pub struct Tonne;
impl MeasurementUnit for Tonne {
    type Property = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type MetricTon = Tonne;

pub struct MegaGram;
impl MeasurementUnit for MegaGram {
    type Property = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mg";
}

pub struct HectoGram;
impl MeasurementUnit for HectoGram {
    type Property = Mass;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hg";
}

pub struct DecaGram;
impl MeasurementUnit for DecaGram {
    type Property = Mass;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dag";
}

pub struct Gram;
impl MeasurementUnit for Gram {
    type Property = Mass;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

pub struct MilliGram;
impl MeasurementUnit for MilliGram {
    type Property = Mass;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mg";
}

pub struct MicroGram;
impl MeasurementUnit for MicroGram {
    type Property = Mass;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}g";
}

pub struct NanoGram;
impl MeasurementUnit for NanoGram {
    type Property = Mass;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ng";
}

pub struct ImperialTon;
impl MeasurementUnit for ImperialTon {
    type Property = Mass;
    const RATIO: f64 = 1016.0469;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type LongTon = ImperialTon;

pub struct USTon;
impl MeasurementUnit for USTon {
    type Property = Mass;
    const RATIO: f64 = 907.18474;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type ShortTon = USTon;

pub struct Stone;
impl MeasurementUnit for Stone {
    type Property = Mass;
    const RATIO: f64 = 6.35029;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " st.";
}

pub struct Pound;
impl MeasurementUnit for Pound {
    type Property = Mass;
    const RATIO: f64 = 0.45359237;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lb";
}

pub struct Ounce;
impl MeasurementUnit for Ounce {
    type Property = Mass;
    const RATIO: f64 = 0.028349523;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " oz";
}

pub struct Carat;
impl MeasurementUnit for Carat {
    type Property = Mass;
    const RATIO: f64 = 0.0002;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ct";
}

// Property: mass density
pub struct MassDensity;

pub struct KiloGramPerCubicMetre;
impl MeasurementUnit for KiloGramPerCubicMetre {
    type Property = MassDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{b3}";
}

pub struct GramPerMilliLitre;
impl MeasurementUnit for GramPerMilliLitre {
    type Property = MassDensity;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/ml";
}

// Property: mass flow rate
pub struct MassFlowRate;

pub struct KiloGramPerSecond;
impl MeasurementUnit for KiloGramPerSecond {
    type Property = MassFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/s";
}

pub struct GramPerSecond;
impl MeasurementUnit for GramPerSecond {
    type Property = MassFlowRate;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/s";
}

// Property: molar concentration
pub struct MolarConcentration;

pub struct MolePerCubicMetre;
impl MeasurementUnit for MolePerCubicMetre {
    type Property = MolarConcentration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{b3}";
}

// Property: molar heat capacity, molar entropy
pub struct MolarHeatCapacity;

pub struct JoulePerKelvinPerMole;
impl MeasurementUnit for JoulePerKelvinPerMole {
    type Property = MolarHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{b0}K/mol";
}

// Property: moment of inertia, rotational inertia
pub struct MomentOfInertia;

pub struct KiloGramSquareMetre;
impl MeasurementUnit for KiloGramSquareMetre {
    type Property = MomentOfInertia;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}";
}

pub struct GramSquareCentiMetre;
impl MeasurementUnit for GramSquareCentiMetre {
    type Property = MomentOfInertia;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}";
}

// Property: momentum, impulse
pub struct Momentum;
impl VectorProperty for Momentum {}

pub struct NewtonSecond;
impl MeasurementUnit for NewtonSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}s";
}

pub struct KiloGramMetrePerSecond;
impl MeasurementUnit for KiloGramMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m/s";
}

pub struct DyneSecond;
impl MeasurementUnit for DyneSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn\u{b7}s";
}

pub struct GramCentiMetrePerSecond;
impl MeasurementUnit for GramCentiMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm/s";
}

// Property: permittivity
pub struct Permittivity;

pub struct FaradPerMetre;
impl MeasurementUnit for FaradPerMetre {
    type Property = Permittivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F/m";
}

// Property: power
pub struct Power;

pub struct Watt;
impl MeasurementUnit for Watt {
    type Property = Power;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

pub struct MilliWatt;
impl MeasurementUnit for MilliWatt {
    type Property = Power;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mW";
}

pub struct KiloWatt;
impl MeasurementUnit for KiloWatt {
    type Property = Power;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kW";
}

pub struct MegaWatt;
impl MeasurementUnit for MegaWatt {
    type Property = Power;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW";
}

pub struct GigaWatt;
impl MeasurementUnit for GigaWatt {
    type Property = Power;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GW";
}

pub struct ErgPerSecond;
impl MeasurementUnit for ErgPerSecond {
    type Property = Power;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " erg/s";
}

pub struct HorsePower;
impl MeasurementUnit for HorsePower {
    type Property = Power;
    const RATIO: f64 = 745.699872;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " HP";
}

// Property: pressure, stress
pub struct Pressure;

pub struct Pascal;
impl MeasurementUnit for Pascal {
    type Property = Pressure;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa";
}

pub struct HectoPascal;
impl MeasurementUnit for HectoPascal {
    type Property = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hPa";
}

pub struct Atmosphere;
impl MeasurementUnit for Atmosphere {
    type Property = Pressure;
    const RATIO: f64 = 1.013e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " atm";
}

pub struct Bar;
impl MeasurementUnit for Bar {
    type Property = Pressure;
    const RATIO: f64 = 1e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " bar";
}
pub struct MilliBar;
impl MeasurementUnit for MilliBar {
    type Property = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mbar";
}

pub struct MmHg;
impl MeasurementUnit for MmHg {
    type Property = Pressure;
    const RATIO: f64 = 133.322;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " torr";
}

pub struct PoundForcePerSquareInch;
impl MeasurementUnit for PoundForcePerSquareInch {
    type Property = Pressure;
    const RATIO: f64 = 6894.757;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lb/in\u{b2}";
}

// Property: radiance
pub struct Radiance;

pub struct WattPerSquareMetrePerSteradian;
impl MeasurementUnit for WattPerSquareMetrePerSteradian {
    type Property = Radiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}/sr";
}

// Property: radiant intensity
pub struct RadiantIntensity;

pub struct WattPerSteradian;
impl MeasurementUnit for WattPerSteradian {
    type Property = RadiantIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/sr";
}

// Property: radioactive activity
pub struct RadioactiveActivity;

pub struct Becquerel;
impl MeasurementUnit for Becquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Bq";
}
pub struct KiloBecquerel;
impl MeasurementUnit for KiloBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kBq";
}

pub struct MegaBecquerel;
impl MeasurementUnit for MegaBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MBq";
}

pub struct GigaBecquerel;
impl MeasurementUnit for GigaBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GBq";
}

// Property: radioactive dose
pub struct RadioactiveDose;

pub struct Gray;
impl MeasurementUnit for Gray {
    type Property = RadioactiveDose;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy";
}

pub struct Rad;
impl MeasurementUnit for Rad {
    type Property = RadioactiveDose;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}

// Property: radioactive dose rate
pub struct RadioactiveDoseRate;

pub struct GrayPerSecond;
impl MeasurementUnit for GrayPerSecond {
    type Property = RadioactiveDoseRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy/s";
}

// Property: reaction rate, catalytic activity concentration
pub struct ReactionRate;

pub struct MolePerCubicMetrePerSecond;
impl MeasurementUnit for MolePerCubicMetrePerSecond {
    type Property = ReactionRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{b3}/s";
}

// Property: solid angle
pub struct SolidAngle;

pub struct Steradian;
impl MeasurementUnit for Steradian {
    type Property = SolidAngle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sr";
}

pub struct Spat;
impl MeasurementUnit for Spat {
    type Property = SolidAngle;
    const RATIO: f64 = 2. * core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sp";
}

pub struct Sphere;
impl MeasurementUnit for Sphere {
    type Property = SolidAngle;
    const RATIO: f64 = 2. * core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sphere";
}

pub struct SquareDegree;
impl MeasurementUnit for SquareDegree {
    type Property = SolidAngle;
    const RATIO: f64 = core::f64::consts::TAU * core::f64::consts::TAU / 360. / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\u{b2}";
}

// Property: specific energy
pub struct SpecificEnergy;

pub struct JoulePerKiloGram;
impl MeasurementUnit for JoulePerKiloGram {
    type Property = SpecificEnergy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg";
}

// Property: specific heat capacity
pub struct SpecificHeatCapacity;

pub struct JoulePerKiloGramPerKelvin;
impl MeasurementUnit for JoulePerKiloGramPerKelvin {
    type Property = SpecificHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg/\u{b0}K";
}

// Property: specific volume
pub struct SpecificVolume;

pub struct CubicMetrePerKiloGram;
impl MeasurementUnit for CubicMetrePerKiloGram {
    type Property = SpecificVolume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}/kg";
}

// Property: square time
pub struct SquareTime;

pub struct SquareSecond;
impl MeasurementUnit for SquareSecond {
    type Property = SquareTime;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s\u{b2}";
}

pub struct HourSecond;
impl MeasurementUnit for HourSecond {
    type Property = SquareTime;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}s";
}

pub struct HourHour;
impl MeasurementUnit for HourHour {
    type Property = SquareTime;
    const RATIO: f64 = 3600. * 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}h";
}

// Property: surface density
pub struct SurfaceDensity;

pub struct KiloGramPerSquareMetre;
impl MeasurementUnit for KiloGramPerSquareMetre {
    type Property = SurfaceDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{b2}";
}

// Property: surface tension
pub struct SurfaceTension;

pub struct JoulePerSquareMetre;
impl MeasurementUnit for JoulePerSquareMetre {
    type Property = SurfaceTension;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{b2}";
}

// Property: temperature
pub struct Temperature;

pub struct Kelvin;
impl MeasurementUnit for Kelvin {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b0}K";
}

pub struct Celsius;
impl MeasurementUnit for Celsius {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 273.15;
    const SUFFIX: &'static str = " \u{b0}C";
}

pub struct Fahrenheit;
impl MeasurementUnit for Fahrenheit {
    type Property = Temperature;
    const RATIO: f64 = 5. / 9.;
    const OFFSET: f64 = 273.15 - 32. * 5. / 9.;
    const SUFFIX: &'static str = " \u{b0}F";
}

// Property: thermal conductivity
pub struct ThermalConductivity;

pub struct WattPerMetrePerKelvin;
impl MeasurementUnit for WattPerMetrePerKelvin {
    type Property = ThermalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m/\u{b0}K";
}

// Property: time, mean lifetime
pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

pub struct Year;
impl MeasurementUnit for Year {
    type Property = Time;
    const RATIO: f64 = 365.25 * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Y";
}

pub struct Month;
impl MeasurementUnit for Month {
    type Property = Time;
    const RATIO: f64 = 30. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " M";
}

pub struct Week;
impl MeasurementUnit for Week {
    type Property = Time;
    const RATIO: f64 = 7. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

pub struct Day;
impl MeasurementUnit for Day {
    type Property = Time;
    const RATIO: f64 = 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " D";
}

pub struct Hour;
impl MeasurementUnit for Hour {
    type Property = Time;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h";
}

pub struct Minute;
impl MeasurementUnit for Minute {
    type Property = Time;
    const RATIO: f64 = 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " min";
}

pub struct MilliSecond;
impl MeasurementUnit for MilliSecond {
    type Property = Time;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ms";
}

pub struct MicroSecond;
impl MeasurementUnit for MicroSecond {
    type Property = Time;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}s";
}

pub struct NanoSecond;
impl MeasurementUnit for NanoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ns";
}

pub struct PicoSecond;
impl MeasurementUnit for PicoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ps";
}

pub struct FemtoSecond;
impl MeasurementUnit for FemtoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fs";
}

// Property: torque
pub struct Torque;
impl VectorProperty for Torque {}

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}

pub struct PoundFoot;
impl MeasurementUnit for PoundFoot {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-ft";
}

pub struct PoundInch;
impl MeasurementUnit for PoundInch {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-in";
}

// Property: velocity, speed
pub struct Velocity;
impl VectorProperty for Velocity {}

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Knot;
impl MeasurementUnit for Knot {
    type Property = Velocity;
    const RATIO: f64 = 1852. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kt";
}

pub struct KiloMetrePerHour;
impl MeasurementUnit for KiloMetrePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h";
}

pub struct MilePerHour;
impl MeasurementUnit for MilePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1609. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi/h";
}

pub struct CentiMetrePerSecond;
impl MeasurementUnit for CentiMetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s";
}

pub struct KiloMetrePerSecond;
impl MeasurementUnit for KiloMetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/s";
}

// Property: volume
pub struct Volume;

pub struct CubicMetre;
impl MeasurementUnit for CubicMetre {
    type Property = Volume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}";
}

pub struct CubicKiloMetre;
impl MeasurementUnit for CubicKiloMetre {
    type Property = Volume;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{b3}";
}

pub struct CubicMicroMetre;
impl MeasurementUnit for CubicMicroMetre {
    type Property = Volume;
    const RATIO: f64 = 1e-18;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m\u{b3}?";
}

pub struct CubicNanoMetre;
impl MeasurementUnit for CubicNanoMetre {
    type Property = Volume;
    const RATIO: f64 = 1e-27;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm\u{b3}?";
}

pub struct CubicInch;
impl MeasurementUnit for CubicInch {
    type Property = Volume;
    const RATIO: f64 = 0.0254 * 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{b3}";
}

pub struct CubicFoot;
impl MeasurementUnit for CubicFoot {
    type Property = Volume;
    const RATIO: f64 = 0.3048 * 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{b3}";
}

pub struct CubicYard;
impl MeasurementUnit for CubicYard {
    type Property = Volume;
    const RATIO: f64 = 0.9144 * 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{b3}";
}

pub struct CubicMile;
impl MeasurementUnit for CubicMile {
    type Property = Volume;
    const RATIO: f64 = 1609. * 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{b3}";
}

pub struct Litre;
impl MeasurementUnit for Litre {
    type Property = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " l";
}

pub struct CubicDecimetre;
impl MeasurementUnit for CubicDecimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm\u{b3}";
}

pub struct MilliLitre;
impl MeasurementUnit for MilliLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml";
}

pub struct CubicCentimetre;
impl MeasurementUnit for CubicCentimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b3}";
}

pub struct MicroLitre;
impl MeasurementUnit for MicroLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}l";
}

pub struct CubicMillimetre;
impl MeasurementUnit for CubicMillimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm\u{b3}";
}

pub struct NanoLitre;
impl MeasurementUnit for NanoLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nl";
}

pub struct PicoLitre;
impl MeasurementUnit for PicoLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pl";
}

pub struct Pint;
impl MeasurementUnit for Pint {
    type Property = Volume;
    const RATIO: f64 = 473.2e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pt";
}

pub struct Gallon;
impl MeasurementUnit for Gallon {
    type Property = Volume;
    const RATIO: f64 = 4546e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " gal";
}

// Property: volumetric flow rate
pub struct VolumetricFlowRate;

pub struct CubicMetrePerSecond;
impl MeasurementUnit for CubicMetrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}/s";
}

pub struct MilliLitrePerSecond;
impl MeasurementUnit for MilliLitrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml/s";
}

pub struct CubicCentimetrePerSecond;
impl MeasurementUnit for CubicCentimetrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b3}/s";
}

// Property: wave number
pub struct WaveNumber;

pub struct CyclePerMetre;
impl MeasurementUnit for CyclePerMetre {
    type Property = WaveNumber;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/m";
}

pub struct RadianPerMetre;
impl MeasurementUnit for RadianPerMetre {
    type Property = WaveNumber;
    const RATIO: f64 = 1. / core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/m";
}

// Relationships among units
use rs_measures::define_units_relationship;

// Computer science

// Properties:
// * Information
// * InformationRate

// InformationRate == Information / Time
define_units_relationship! {Bit == BitPerSecond * Second}
define_units_relationship! {Byte == BytePerSecond * Second}
define_units_relationship! {KiloBit == KiloBitPerSecond * Second}
define_units_relationship! {KiloByte == KiloBytePerSecond * Second}
define_units_relationship! {KibiBit == KibiBitPerSecond * Second}
define_units_relationship! {KibiByte == KibiBytePerSecond * Second}
define_units_relationship! {MegaBit == MegaBitPerSecond * Second}
define_units_relationship! {MegaByte == MegaBytePerSecond * Second}
define_units_relationship! {MebiBit == MebiBitPerSecond * Second}
define_units_relationship! {MebiByte == MebiBytePerSecond * Second}
define_units_relationship! {GigaBit == GigaBitPerSecond * Second}
define_units_relationship! {GigaByte == GigaBytePerSecond * Second}
define_units_relationship! {GibiBit == GibiBitPerSecond * Second}
define_units_relationship! {GibiByte == GibiBytePerSecond * Second}
define_units_relationship! {TeraBit == TeraBitPerSecond * Second}
define_units_relationship! {TeraByte == TeraBytePerSecond * Second}
define_units_relationship! {TebiBit == TebiBitPerSecond * Second}
define_units_relationship! {TebiByte == TebiBytePerSecond * Second}

// Geometry

// Properties:
// * Angle
// * Area
// * Length
// * SolidAngle
// * Volume
// * WaveNumber

// Area == Length * Length
define_units_relationship! {SquareMetre == Metre * =}
define_units_relationship! {SquareKiloMetre == KiloMetre * =}
define_units_relationship! {Hectare == HectoMetre * =}
define_units_relationship! {Are == DecaMetre * =}
define_units_relationship! {SquareDeciMetre == DeciMetre * =}
define_units_relationship! {SquareCentiMetre == CentiMetre * =}
define_units_relationship! {SquareMilliMetre == MilliMetre * =}
define_units_relationship! {SquareMicroMetre == MicroMetre * =}
define_units_relationship! {SquareNanoMetre == NanoMetre * =}
define_units_relationship! {SquareInch == Inch * =}
define_units_relationship! {SquareFoot == Foot * =}
define_units_relationship! {SquareYard == Yard * =}
define_units_relationship! {SquareMile == Mile * =}

// Volume == Area * Length
define_units_relationship! {CubicMetre == SquareMetre * Metre}
define_units_relationship! {CubicKiloMetre == SquareKiloMetre * KiloMetre}
define_units_relationship! {Litre == SquareDeciMetre * DeciMetre}
define_units_relationship! {MilliLitre == SquareCentiMetre * CentiMetre}
define_units_relationship! {MicroLitre == SquareMilliMetre * MilliMetre}
define_units_relationship! {CubicMicroMetre == SquareMicroMetre * MicroMetre}
define_units_relationship! {CubicNanoMetre == SquareNanoMetre * NanoMetre}
define_units_relationship! {CubicInch == SquareInch * Inch}
define_units_relationship! {CubicFoot == SquareFoot * Foot}
define_units_relationship! {CubicYard == SquareYard * Yard}
define_units_relationship! {CubicMile == SquareMile * Mile}

// WaveNumber == Angle / Length
define_units_relationship! {Cycle == CyclePerMetre * Metre}
define_units_relationship! {Radian == RadianPerMetre * Metre}

// Kinematics

// Properties:
// * Acceleration
// * AngularAcceleration
// * Frequency
// * KinematicViscosity
// * SquareTime
// * Time
// * Velocity
// * VolumetricFlowRate

// Acceleration == Velocity / Time
define_units_relationship! {MetrePerSecond == MetrePerSquareSecond * Second}
define_units_relationship! {MetrePerSecond:2 == MetrePerSquareSecond:2 * Second}
define_units_relationship! {MetrePerSecond:3 == MetrePerSquareSecond:3 * Second}
define_units_relationship! {CentiMetrePerSecond == CentiMetrePerSquareSecond * Second}
define_units_relationship! {CentiMetrePerSecond:2 == CentiMetrePerSquareSecond:2 * Second}
define_units_relationship! {CentiMetrePerSecond:3 == CentiMetrePerSquareSecond:3 * Second}
define_units_relationship! {KiloMetrePerHour == KiloMetrePerHourPerSecond * Second}
define_units_relationship! {KiloMetrePerHour:2 == KiloMetrePerHourPerSecond:2 * Second}
define_units_relationship! {KiloMetrePerHour:3 == KiloMetrePerHourPerSecond:3 * Second}

// Velocity == Length / Time
define_units_relationship! {Metre == MetrePerSecond * Second}
define_units_relationship! {Metre:2 == MetrePerSecond:2 * Second}
define_units_relationship! {Metre:3 == MetrePerSecond:3 * Second}
define_units_relationship! {NauticalMile == Knot * Hour}
define_units_relationship! {NauticalMile:2 == Knot:2 * Hour}
define_units_relationship! {NauticalMile:3 == Knot:3 * Hour}
define_units_relationship! {KiloMetre == KiloMetrePerHour * Hour}
define_units_relationship! {KiloMetre:2 == KiloMetrePerHour:2 * Hour}
define_units_relationship! {KiloMetre:3 == KiloMetrePerHour:3 * Hour}
define_units_relationship! {Mile == MilePerHour * Hour}
define_units_relationship! {Mile:2 == MilePerHour:2 * Hour}
define_units_relationship! {Mile:3 == MilePerHour:3 * Hour}
define_units_relationship! {CentiMetre == CentiMetrePerSecond * Second}
define_units_relationship! {CentiMetre:2 == CentiMetrePerSecond:2 * Second}
define_units_relationship! {CentiMetre:3 == CentiMetrePerSecond:3 * Second}

// AngularAcceleration == Frequency / Time
define_units_relationship! {RadianPerSecond == RadianPerSquareSecond * Second}

// Frequency == Angle / Time
define_units_relationship! {Cycle == Hertz * Second}
define_units_relationship! {Radian == RadianPerSecond * Second}
define_units_relationship! {Cycle == CyclePerMinute * Minute}

// KinematicViscosity == Area / Time
define_units_relationship! {SquareMetre == SquareMetrePerSecond * Second}
define_units_relationship! {SquareMilliMetre == SquareMetrePerSecond * MicroSecond}
define_units_relationship! {SquareMicroMetre == SquareMetrePerSecond * PicoSecond}
define_units_relationship! {SquareCentiMetre == Stoke * Second}
define_units_relationship! {SquareMilliMetre == CentiStoke * Second}
define_units_relationship! {SquareMicroMetre == CentiStoke * MicroSecond}
define_units_relationship! {SquareNanoMetre == CentiStoke * PicoSecond}

// KinematicViscosity == Length * Velocity
define_units_relationship! {SquareMetrePerSecond == Metre * MetrePerSecond}
define_units_relationship! {SquareMetrePerSecond == HectoMetre * CentiMetrePerSecond}
define_units_relationship! {Stoke == CentiMetre * CentiMetrePerSecond}

// SquareTime == Time * Time
define_units_relationship! {SquareSecond == Second * =}
define_units_relationship! {HourSecond == Hour * Second}
define_units_relationship! {HourHour == Hour * =}

// VolumetricFlowRate == Volume / Time
define_units_relationship! {CubicMetre == CubicMetrePerSecond * Second}
define_units_relationship! {MilliLitre == MilliLitrePerSecond * Second}

// VolumetricFlowRate == Area * Velocity
define_units_relationship! {CubicMetrePerSecond == SquareMetre * MetrePerSecond}
define_units_relationship! {MilliLitrePerSecond == SquareCentiMetre * CentiMetrePerSecond}

// Dynamics

// Properties:
// * Action
// * AngularMomentum
// * DynamicViscosity
// * Energy
// * EnergyDensity
// * Force
// * KinematicViscosity
// * LinearDensity
// * Mass
// * MassDensity
// * MassFlowRate
// * MomentOfInertia
// * Momentum
// * Power
// * Pressure
// * SpecificEnergy
// * SpecificVolume
// * SurfaceDensity
// * SurfaceTension
// * Torque

// Action == Energy * Time
define_units_relationship! {JouleSecond == Joule * Second}

// Action == Power * SquareTime
define_units_relationship! {JouleSecond == Watt * SquareSecond}

// AngularMomentum == Momentum * Length
define_units_relationship! {KiloGramSquareMetrePerSecond == KiloGramMetrePerSecond:2 X Metre:2}
define_units_relationship! {KiloGramSquareMetrePerSecond:3 == KiloGramMetrePerSecond:3 X Metre:3}

// AngularMomentum == MomentOfInertia / Time
define_units_relationship! {KiloGramSquareMetre == KiloGramSquareMetrePerSecond * Second}

// DynamicViscosity == Pressure * Time
define_units_relationship! {PascalSecond == Pascal * Second}

// Energy == Force * Length
define_units_relationship! {Joule == Newton * Metre}
define_units_relationship! {Joule == Newton:2 * Metre:2}
define_units_relationship! {Joule == Newton:3 * Metre:3}
define_units_relationship! {Erg == Dyne * CentiMetre}
define_units_relationship! {Erg == Dyne:2 * CentiMetre:2}
define_units_relationship! {Erg == Dyne:3 * CentiMetre:3}

// Energy == Momentum * Velocity
define_units_relationship! {Joule == NewtonSecond * MetrePerSecond}
define_units_relationship! {Joule == NewtonSecond:2 * MetrePerSecond:2}
define_units_relationship! {Joule == NewtonSecond:3 * MetrePerSecond:3}
define_units_relationship! {Erg == DyneSecond * CentiMetrePerSecond}
define_units_relationship! {Erg == DyneSecond:2 * CentiMetrePerSecond:2}
define_units_relationship! {Erg == DyneSecond:3 * CentiMetrePerSecond:3}

// Energy == MomentOfInertia / SquareTime
define_units_relationship! {KiloGramSquareMetre == Joule * SquareSecond}

// EnergyDensity == Energy / Volume
define_units_relationship! {Joule == JoulePerCubicMetre * CubicMetre}

// Force == Mass * Acceleration
define_units_relationship! {Newton == KiloGram * MetrePerSquareSecond}
define_units_relationship! {Newton:2 == KiloGram * MetrePerSquareSecond:2}
define_units_relationship! {Newton:3 == KiloGram * MetrePerSquareSecond:3}
define_units_relationship! {Dyne == Gram * CentiMetrePerSquareSecond}
define_units_relationship! {Dyne:2 == Gram * CentiMetrePerSquareSecond:2}
define_units_relationship! {Dyne:3 == Gram * CentiMetrePerSquareSecond:3}
define_units_relationship! {KiloGramForce == KiloGram * GForce}
define_units_relationship! {KiloGramForce:2 == KiloGram * GForce:2}
define_units_relationship! {KiloGramForce:3 == KiloGram * GForce:3}

// LinearDensity == Mass / Length
define_units_relationship! {KiloGram == KiloGramPerMetre * Metre}
define_units_relationship! {Gram == GramPerCentiMetre * CentiMetre}

// MassDensity == Mass / Volume
define_units_relationship! {KiloGram == KiloGramPerCubicMetre * CubicMetre}
define_units_relationship! {Gram == GramPerMilliLitre * MilliLitre}

// MassFlowRate == Mass / Time
define_units_relationship! {KiloGram == KiloGramPerSecond * Second}
define_units_relationship! {Gram == GramPerSecond * Second}

// MomentOfInertia == Mass * Area
define_units_relationship! {KiloGramSquareMetre == KiloGram * SquareMetre}
define_units_relationship! {GramSquareCentiMetre == Gram * SquareCentiMetre}

// Momentum == Force * Time
define_units_relationship! {NewtonSecond == Newton * Second}
define_units_relationship! {NewtonSecond:2 == Newton:2 * Second}
define_units_relationship! {NewtonSecond:3 == Newton:3 * Second}
define_units_relationship! {DyneSecond == Dyne * Second}
define_units_relationship! {DyneSecond:2 == Dyne:2 * Second}
define_units_relationship! {DyneSecond:3 == Dyne:3 * Second}

// Momentum == Mass * Velocity
define_units_relationship! {NewtonSecond == KiloGram * MetrePerSecond}
define_units_relationship! {NewtonSecond:2 == KiloGram * MetrePerSecond:2}
define_units_relationship! {NewtonSecond:3 == KiloGram * MetrePerSecond:3}
define_units_relationship! {DyneSecond == Gram * CentiMetrePerSecond}
define_units_relationship! {DyneSecond:2 == Gram * CentiMetrePerSecond:2}
define_units_relationship! {DyneSecond:3 == Gram * CentiMetrePerSecond:3}

// Power == Energy / Time
define_units_relationship! {Joule == Watt * Second}
define_units_relationship! {WattHour == Watt * Hour}
define_units_relationship! {KiloWattHour == KiloWatt * Hour}
define_units_relationship! {Erg == ErgPerSecond * Second}

// Pressure == Force / Area
define_units_relationship! {Newton == Pascal * SquareMetre}
define_units_relationship! {PoundForce == PoundForcePerSquareInch * SquareInch}
define_units_relationship! {Newton == HectoPascal * SquareDeciMetre}

// SpecificEnergy == Energy / Mass
define_units_relationship! {Joule == JoulePerKiloGram * KiloGram}

// SpecificVolume == Volume / Mass
define_units_relationship! {CubicMetre == CubicMetrePerKiloGram * KiloGram}

// SpecificVolume == 1 / MassDensity
define_units_relationship! {CubicMetrePerKiloGram == 1 / KiloGramPerCubicMetre}

// SurfaceDensity == Mass / Area
define_units_relationship! {KiloGram == KiloGramPerSquareMetre * SquareMetre}

// SurfaceTension == Energy / Area
define_units_relationship! {Joule == JoulePerSquareMetre * SquareMetre}

// Torque == Force * Length
define_units_relationship! {NewtonMetre == Newton:2 X Metre:2}
define_units_relationship! {NewtonMetre:3 == Newton:3 X Metre:3}
define_units_relationship! {PoundFoot == PoundForce:2 X Foot:2}
define_units_relationship! {PoundFoot:3 == PoundForce:3 X Foot:3}
define_units_relationship! {PoundInch == PoundForce:2 X Inch:2}
define_units_relationship! {PoundInch:3 == PoundForce:3 X Inch:3}

// Thermodynamics

// Properties:
// * Entropy
// * SpecificHeatCapacity
// * Temperature
// * ThermalConductivity

// Entropy == Energy / Temperature
define_units_relationship! {Joule == JoulePerKelvin * Kelvin}

// SpecificHeatCapacity == Entropy / Mass
define_units_relationship! {JoulePerKelvin == JoulePerKiloGramPerKelvin * KiloGram}

// TODO: ThermalConductivity == Power / Length / Temperature
// TODO: define_units_relationship! {WattPerMetrePerKelvin == ?}

// Chemistry

// Properties:
// * CatalyticActivity
// * ChemicalPotential
// * MolarConcentration
// * MolarHeatCapacity
// * ReactionRate

// CatalyticActivity == Amount / Time
define_units_relationship! {Mole == Katal * Second}

// ChemicalPotential == Energy / Amount
define_units_relationship! {Joule == JoulePerMole * Mole}

// MolarConcentration == Amount / Volume
define_units_relationship! {Mole == MolePerCubicMetre * CubicMetre}

// MolarHeatCapacity == ChemicalPotential / Temperature
define_units_relationship! {JoulePerMole == JoulePerKelvinPerMole * Kelvin}

// ReactionRate == MolarConcentration / Time
define_units_relationship! {MolePerCubicMetre == MolePerCubicMetrePerSecond * Second}

// ReactionRate == CatalyticActivity / Volume
define_units_relationship! {Katal == MolePerCubicMetrePerSecond * CubicMetre}

// Radioactivity

// Properties:
// * DoseEquivalent
// * RadioactiveActivity
// * RadioactiveDose
// * RadioactiveDoseRate

// RadioactiveDoseRate == RadioactiveDose / Time
define_units_relationship! {Gray == GrayPerSecond * Second}

// Optics

// Properties:
// * Illuminance
// * Irradiance
// * Luminance
// * LuminousFlux
// * LuminousIntensity
// * Radiance
// * RadiantIntensity

// Illuminance == LuminousFlux / Area
define_units_relationship! {Lumen == Lux * SquareMetre}
define_units_relationship! {Lux == CandelaPerSquareMetre * Steradian}
define_units_relationship! {Lumen == Phot * SquareCentiMetre}
define_units_relationship! {Phot == Stilb * Steradian}
define_units_relationship! {Lumen == FootCandle * SquareFoot}
define_units_relationship! {FootCandle == CandelaPerSquareFoot * Steradian}

// Irradiance == Power / Area
define_units_relationship! {Watt == WattPerSquareMetre * SquareMetre}

// Luminance == LuminousIntensity / Area
define_units_relationship! {Candela == CandelaPerSquareMetre * SquareMetre}
define_units_relationship! {Candela == Stilb * SquareCentiMetre}
define_units_relationship! {Candela == CandelaPerSquareFoot * SquareFoot}

// LuminousFlux == LuminousIntensity * SolidAngle
define_units_relationship! {Lumen == Candela * Steradian}

// Radiance == RadiantIntensity / Area
define_units_relationship! {WattPerSteradian == WattPerSquareMetrePerSteradian * SquareMetre}

// Radiance == Irradiance / SolidAngle
define_units_relationship! {WattPerSquareMetre == WattPerSquareMetrePerSteradian * Steradian}

// RadiantIntensity == Power / SolidAngle
define_units_relationship! {Watt == WattPerSteradian * Steradian}

// Electricity

// Properties:
// * Capacitance
// * CurrentDensity
// * ElectricalConductance
// * ElectricalConductivity
// * ElectricalResistance
// * ElectricCharge
// * ElectricChargeDensity
// * ElectricCurrent
// * ElectricDisplacement
// * ElectricFieldStrength
// * ElectricPotential
// * LinearElectricChargeDensity
// * Permittivity

// Capacitance == ElectricCharge / ElectricPotential
define_units_relationship! {Coulomb == Farad * Volt}
define_units_relationship! {MilliCoulomb == MilliFarad * Volt}
define_units_relationship! {Coulomb == MilliFarad * KiloVolt}
define_units_relationship! {MicroCoulomb == MicroFarad * Volt}
define_units_relationship! {MilliCoulomb == MicroFarad * KiloVolt}
define_units_relationship! {NanoCoulomb == NanoFarad * Volt}
define_units_relationship! {MicroCoulomb == NanoFarad * KiloVolt}
define_units_relationship! {PicoCoulomb == PicoFarad * Volt}
define_units_relationship! {NanoCoulomb == PicoFarad * KiloVolt}

// CurrentDensity == ElectricCurrent * Area
define_units_relationship! {Ampere == AmperePerSquareMetre * SquareMetre}

// ElectricalConductance == ElectricCurrent / ElectricPotential
define_units_relationship! {Ampere == Siemens * Volt}
define_units_relationship! {MilliAmpere == Siemens * KiloVolt}

// ElectricalConductance == 1 / ElectricalResistance
define_units_relationship! {Siemens == 1 / Ohm}

// ElectricalConductivity == ElectricalConductance / Length
define_units_relationship! {Siemens == SiemensPerMetre * Metre}

// ElectricalResistance == ElectricPotential / ElectricCurrent
define_units_relationship! {Volt == Ohm * Ampere}

// ElectricCurrent == ElectricCharge / Time
define_units_relationship! {Coulomb == Ampere * Second}
define_units_relationship! {MilliCoulomb == Ampere * MilliSecond}
define_units_relationship! {MilliCoulomb == MilliAmpere * Second}
define_units_relationship! {MicroCoulomb == MilliAmpere * MilliSecond}
define_units_relationship! {MicroCoulomb == MicroAmpere * Second}

// ElectricChargeDensity == ElectricCharge / Volume
define_units_relationship! {Coulomb == CoulombPerCubicMetre * CubicMetre}
define_units_relationship! {MilliCoulomb == CoulombPerCubicMetre * Litre}
define_units_relationship! {MicroCoulomb == CoulombPerCubicMetre * MilliLitre}

// ElectricDisplacement == ElectricCharge / Area
define_units_relationship! {Coulomb == CoulombPerSquareMetre * SquareMetre}
define_units_relationship! {MicroCoulomb == CoulombPerSquareMetre * SquareMilliMetre}

// ElectricFieldStrength == ElectricPotential / Length
define_units_relationship! {Volt == VoltPerMetre * Metre}

// ElectricFieldStrength == Force / ElectricCharge
define_units_relationship! {Newton == NewtonPerCoulomb * Coulomb}

// ElectricPotential == Power / ElectricCurrent
define_units_relationship! {Watt == Volt * Ampere}
define_units_relationship! {MilliWatt == Volt * MilliAmpere}
define_units_relationship! {Watt == KiloVolt * MilliAmpere}
define_units_relationship! {MilliWatt == KiloVolt * MicroAmpere}
define_units_relationship! {KiloWatt == KiloVolt * Ampere}
define_units_relationship! {MilliWatt == MilliVolt * Ampere}

// LinearElectricChargeDensity == ElectricCharge / Length
define_units_relationship! {Coulomb == CoulombPerMetre * Metre}
define_units_relationship! {MilliCoulomb == CoulombPerMetre * MilliMetre}
define_units_relationship! {MicroCoulomb == CoulombPerMetre * MicroMetre}

// Permittivity == Capacitance / Length
define_units_relationship! {Farad == FaradPerMetre * Metre}
define_units_relationship! {MilliFarad == FaradPerMetre * MilliMetre}
define_units_relationship! {MicroFarad == FaradPerMetre * MicroMetre}
define_units_relationship! {NanoFarad == FaradPerMetre * NanoMetre}

// Magnetism

// Properties:
// * Inductance
// * MagneticFieldStrength
// * MagneticFlux
// * MagneticFluxDensity
// * MagneticPermeability
// * MagneticReluctance

// Inductance == MagneticFlux / ElectricCurrent
define_units_relationship! {Weber == Henry * Ampere}

// MagneticFieldStrength == ElectricCurrent / Length
define_units_relationship! {Ampere == AmperePerMetre * Metre}
define_units_relationship! {MilliAmpere == AmperePerMetre * MilliMetre}
define_units_relationship! {MicroAmpere == AmperePerMetre * MicroMetre}

// TODO: MagneticFlux == Mass * Area / SquareTime / Current

// ElectricFieldStrength == Velocity X MagneticFlux
define_units_relationship! {VoltPerMetre == MetrePerSecond * Weber}
define_units_relationship! {VoltPerMetre == MetrePerSecond:2 X Weber:2}
define_units_relationship! {VoltPerMetre:3 == MetrePerSecond:3 X Weber:3}

// MagneticFluxDensity == MagneticFlux / Area
// TODO: MagneticFluxDensity == Mass / SquareTime / Current
// TODO: MagneticFluxDensity == Force / Length / Current

// MagneticPermeability == Inductance / Length

// MagneticReluctance == 1 / Inductance
define_units_relationship! {InverseHenry == 1 / Henry}

// Others

// Properties:
// * Amount
// * Dimensionless

// Dimensionless == Dimensionless * Dimensionless
define_units_relationship! {Unspecified == Unspecified:2 * =:2}
define_units_relationship! {Unspecified == Unspecified:2 X =:2}
define_units_relationship! {Unspecified == Unspecified:3 * =:3}
define_units_relationship! {Unspecified:3 == Unspecified:3 X =:3}
// N.B.: The following definition is not allowed:
// ```
// define_units_relationship! {Unspecified == Unspecified * Unspecified}
// ```
// This is because it would imply that `Measure<Unspecified> / Measure<Unspecified>`
// is a `Measure<Unspecified>`. Though, in general, that division is already defined
// to be a number.
