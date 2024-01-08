rs_measures::define_measure_3d! {}

// Property: acceleration
pub struct Acceleration;

pub struct MetrePerSquareSecond;
impl MeasurementUnit for MetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}
impl VectorMeasurementUnit for MetrePerSquareSecond {}

pub struct CentiMetrePerSquareSecond;
impl MeasurementUnit for CentiMetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1e-2;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}
impl VectorMeasurementUnit for CentiMetrePerSquareSecond {}

pub struct GForce;
impl MeasurementUnit for GForce {
    type Property = Acceleration;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}
impl VectorMeasurementUnit for GForce {}

pub struct KiloMetrePerHourPerSecond;
impl MeasurementUnit for KiloMetrePerHourPerSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h/s";
}
impl VectorMeasurementUnit for KiloMetrePerHourPerSecond {}

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

pub struct KiloGramSquareMetrePerSecond;
impl MeasurementUnit for KiloGramSquareMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}/s";
}
impl VectorMeasurementUnit for KiloGramSquareMetrePerSecond {}

pub struct GramSquareCentiMetrePerSecond;
impl MeasurementUnit for GramSquareCentiMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}/s";
}
impl VectorMeasurementUnit for GramSquareCentiMetrePerSecond {}

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

pub struct AmperePerSquareMetre;
impl MeasurementUnit for AmperePerSquareMetre {
    type Property = CurrentDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m\u{b2}";
}
impl VectorMeasurementUnit for AmperePerSquareMetre {}

// Property: dimensionless
pub struct Dimensionless;

pub struct Unspecified;
impl MeasurementUnit for Unspecified {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = "";
}
impl VectorMeasurementUnit for Unspecified {}

pub struct Mach;
impl MeasurementUnit for Mach {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mach";
}
impl VectorMeasurementUnit for Mach {}

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

pub struct VoltPerMetre;
impl MeasurementUnit for VoltPerMetre {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V/m";
}
impl VectorMeasurementUnit for VoltPerMetre {}

pub struct NewtonPerCoulomb;
impl MeasurementUnit for NewtonPerCoulomb {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N/C";
}
impl VectorMeasurementUnit for NewtonPerCoulomb {}

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

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}
impl VectorMeasurementUnit for Newton {}

pub struct Dyne;
impl MeasurementUnit for Dyne {
    type Property = Force;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn";
}
impl VectorMeasurementUnit for Dyne {}

pub struct KiloGramForce;
impl MeasurementUnit for KiloGramForce {
    type Property = Force;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kgf";
}
impl VectorMeasurementUnit for KiloGramForce {}

pub struct PoundForce;
impl MeasurementUnit for PoundForce {
    type Property = Force;
    const RATIO: f64 = 4.448222;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf";
}
impl VectorMeasurementUnit for PoundForce {}

pub struct Poundal;
impl MeasurementUnit for Poundal {
    type Property = Force;
    const RATIO: f64 = 0.138255;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pdl";
}
impl VectorMeasurementUnit for Poundal {}

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

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}
impl VectorMeasurementUnit for Metre {}

pub struct AstronomicalUnit;
impl MeasurementUnit for AstronomicalUnit {
    type Property = Length;
    const RATIO: f64 = 149597870691.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " a.u.";
}
impl VectorMeasurementUnit for AstronomicalUnit {}

pub struct Parsec;
impl MeasurementUnit for Parsec {
    type Property = Length;
    const RATIO: f64 = 3.0856775813e16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " psc";
}
impl VectorMeasurementUnit for Parsec {}

pub struct LightYear;
impl MeasurementUnit for LightYear {
    type Property = Length;
    const RATIO: f64 = 31557600. * 2.99792458e8;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ly";
}
impl VectorMeasurementUnit for LightYear {}

pub struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Property = Length;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}
impl VectorMeasurementUnit for KiloMetre {}

pub struct HectoMetre;
impl MeasurementUnit for HectoMetre {
    type Property = Length;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hm";
}
impl VectorMeasurementUnit for HectoMetre {}

pub struct DecaMetre;
impl MeasurementUnit for DecaMetre {
    type Property = Length;
    const RATIO: f64 = 10.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dam";
}
impl VectorMeasurementUnit for DecaMetre {}

pub struct DeciMetre;
impl MeasurementUnit for DeciMetre {
    type Property = Length;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm";
}
impl VectorMeasurementUnit for DeciMetre {}

pub struct CentiMetre;
impl MeasurementUnit for CentiMetre {
    type Property = Length;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm";
}
impl VectorMeasurementUnit for CentiMetre {}

pub struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}
impl VectorMeasurementUnit for MilliMetre {}

pub struct MicroMetre;
impl MeasurementUnit for MicroMetre {
    type Property = Length;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m";
}
impl VectorMeasurementUnit for MicroMetre {}

pub struct NanoMetre;
impl MeasurementUnit for NanoMetre {
    type Property = Length;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm";
}
impl VectorMeasurementUnit for NanoMetre {}

pub struct Angstrom;
impl MeasurementUnit for Angstrom {
    type Property = Length;
    const RATIO: f64 = 1e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{212b}";
}
impl VectorMeasurementUnit for Angstrom {}

pub struct Inch;
impl MeasurementUnit for Inch {
    type Property = Length;
    const RATIO: f64 = 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in";
}
impl VectorMeasurementUnit for Inch {}

pub struct Foot;
impl MeasurementUnit for Foot {
    type Property = Length;
    const RATIO: f64 = 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft";
}
impl VectorMeasurementUnit for Foot {}

pub struct Yard;
impl MeasurementUnit for Yard {
    type Property = Length;
    const RATIO: f64 = 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd";
}
impl VectorMeasurementUnit for Yard {}

pub struct Mile;
impl MeasurementUnit for Mile {
    type Property = Length;
    const RATIO: f64 = 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi";
}
impl VectorMeasurementUnit for Mile {}

pub struct NauticalMile;
impl MeasurementUnit for NauticalMile {
    type Property = Length;
    const RATIO: f64 = 1852.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " naut.mi";
}
impl VectorMeasurementUnit for NauticalMile {}

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

pub struct Lumen;
impl MeasurementUnit for Lumen {
    type Property = LuminousFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lm";
}
impl VectorMeasurementUnit for Lumen {}

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

pub struct AmperePerMetre;
impl MeasurementUnit for AmperePerMetre {
    type Property = MagneticFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m";
}
impl VectorMeasurementUnit for AmperePerMetre {}

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

pub struct Tesla;
impl MeasurementUnit for Tesla {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " T";
}
impl VectorMeasurementUnit for Tesla {}

pub struct Gauss;
impl MeasurementUnit for Gauss {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " G";
}
impl VectorMeasurementUnit for Gauss {}

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

pub type LongTon = ImperialTon;

pub struct USTon;
impl MeasurementUnit for USTon {
    type Property = Mass;
    const RATIO: f64 = 907.18474;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

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

pub struct NewtonSecond;
impl MeasurementUnit for NewtonSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}s";
}
impl VectorMeasurementUnit for NewtonSecond {}

pub struct KiloGramMetrePerSecond;
impl MeasurementUnit for KiloGramMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m/s";
}
impl VectorMeasurementUnit for KiloGramMetrePerSecond {}

pub struct DyneSecond;
impl MeasurementUnit for DyneSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn\u{b7}s";
}
impl VectorMeasurementUnit for DyneSecond {}

pub struct GramCentiMetrePerSecond;
impl MeasurementUnit for GramCentiMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm/s";
}
impl VectorMeasurementUnit for GramCentiMetrePerSecond {}

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

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}
impl VectorMeasurementUnit for NewtonMetre {}

pub struct PoundFoot;
impl MeasurementUnit for PoundFoot {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-ft";
}
impl VectorMeasurementUnit for PoundFoot {}

pub struct PoundInch;
impl MeasurementUnit for PoundInch {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-in";
}
impl VectorMeasurementUnit for PoundInch {}

// Property: velocity, speed
pub struct Velocity;

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}
impl VectorMeasurementUnit for MetrePerSecond {}

pub struct Knot;
impl MeasurementUnit for Knot {
    type Property = Velocity;
    const RATIO: f64 = 1852. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kt";
}
impl VectorMeasurementUnit for Knot {}

pub struct KiloMetrePerHour;
impl MeasurementUnit for KiloMetrePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h";
}
impl VectorMeasurementUnit for KiloMetrePerHour {}

pub struct MilePerHour;
impl MeasurementUnit for MilePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1609. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi/h";
}
impl VectorMeasurementUnit for MilePerHour {}

pub struct CentiMetrePerSecond;
impl MeasurementUnit for CentiMetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s";
}
impl VectorMeasurementUnit for CentiMetrePerSecond {}

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
use units_relation::define_units_relation;

// Computer science

// Properties:
// * Information
// * InformationRate

// InformationRate == Information / Time
define_units_relation! {BitPerSecond == Bit / Second}
define_units_relation! {BytePerSecond == Byte / Second}
define_units_relation! {KiloBitPerSecond == KiloBit / Second}
define_units_relation! {KiloBytePerSecond == KiloByte / Second}
define_units_relation! {KibiBitPerSecond == KibiBit / Second}
define_units_relation! {KibiBytePerSecond == KibiByte / Second}
define_units_relation! {MegaBitPerSecond == MegaBit / Second}
define_units_relation! {MegaBytePerSecond == MegaByte / Second}
define_units_relation! {MebiBitPerSecond == MebiBit / Second}
define_units_relation! {MebiBytePerSecond == MebiByte / Second}
define_units_relation! {GigaBitPerSecond == GigaBit / Second}
define_units_relation! {GigaBytePerSecond == GigaByte / Second}
define_units_relation! {GibiBitPerSecond == GibiBit / Second}
define_units_relation! {GibiBytePerSecond == GibiByte / Second}
define_units_relation! {TeraBitPerSecond == TeraBit / Second}
define_units_relation! {TeraBytePerSecond == TeraByte / Second}
define_units_relation! {TebiBitPerSecond == TebiBit / Second}
define_units_relation! {TebiBytePerSecond == TebiByte / Second}

// Geometry

// Properties:
// * Angle
// * Area
// * Length
// * SolidAngle
// * Volume
// * WaveNumber

// Area == Length * Length
define_units_relation! {SquareMetre == Metre * Metre}
define_units_relation! {SquareKiloMetre == KiloMetre * KiloMetre}
define_units_relation! {Hectare == HectoMetre * HectoMetre}
define_units_relation! {Are == DecaMetre * DecaMetre}
define_units_relation! {SquareDeciMetre == DeciMetre * DeciMetre}
define_units_relation! {SquareCentiMetre == CentiMetre * CentiMetre}
define_units_relation! {SquareMilliMetre == MilliMetre * MilliMetre}
define_units_relation! {SquareMicroMetre == MicroMetre * MicroMetre}
define_units_relation! {SquareNanoMetre == NanoMetre * NanoMetre}
define_units_relation! {SquareInch == Inch * Inch}
define_units_relation! {SquareFoot == Foot * Foot}
define_units_relation! {SquareYard == Yard * Yard}
define_units_relation! {SquareMile == Mile * Mile}

// Volume == Area * Length
define_units_relation! {CubicMetre == SquareMetre * Metre}
define_units_relation! {CubicKiloMetre == SquareKiloMetre * KiloMetre}
define_units_relation! {Litre == SquareDeciMetre * DeciMetre}
define_units_relation! {MilliLitre == SquareCentiMetre * CentiMetre}
define_units_relation! {MicroLitre == SquareMilliMetre * MilliMetre}
define_units_relation! {CubicMicroMetre == SquareMicroMetre * MicroMetre}
define_units_relation! {CubicNanoMetre == SquareNanoMetre * NanoMetre}
define_units_relation! {CubicInch == SquareInch * Inch}
define_units_relation! {CubicFoot == SquareFoot * Foot}
define_units_relation! {CubicYard == SquareYard * Yard}
define_units_relation! {CubicMile == SquareMile * Mile}

// WaveNumber == Angle / Length
define_units_relation! {CyclePerMetre == Cycle / Metre}
define_units_relation! {RadianPerMetre == Radian / Metre}

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
define_units_relation! {MetrePerSquareSecond == MetrePerSecond / Second}
define_units_relation! {MetrePerSquareSecond:2 == MetrePerSecond:2 / Second}
define_units_relation! {MetrePerSquareSecond:3 == MetrePerSecond:3 / Second}
define_units_relation! {CentiMetrePerSquareSecond == CentiMetrePerSecond / Second}
define_units_relation! {CentiMetrePerSquareSecond:2 == CentiMetrePerSecond:2 / Second}
define_units_relation! {CentiMetrePerSquareSecond:3 == CentiMetrePerSecond:3 / Second}
define_units_relation! {KiloMetrePerHourPerSecond == KiloMetrePerHour / Second}
define_units_relation! {KiloMetrePerHourPerSecond:2 == KiloMetrePerHour:2 / Second}
define_units_relation! {KiloMetrePerHourPerSecond:3 == KiloMetrePerHour:3 / Second}

// Velocity == Length / Time
define_units_relation! {MetrePerSecond == Metre / Second}
define_units_relation! {MetrePerSecond:2 == Metre:2 / Second}
define_units_relation! {MetrePerSecond:3 == Metre:3 / Second}
define_units_relation! {Knot == NauticalMile / Hour}
define_units_relation! {Knot:2 == NauticalMile:2 / Hour}
define_units_relation! {Knot:3 == NauticalMile:3 / Hour}
define_units_relation! {KiloMetrePerHour == KiloMetre / Hour}
define_units_relation! {KiloMetrePerHour:2 == KiloMetre:2 / Hour}
define_units_relation! {KiloMetrePerHour:3 == KiloMetre:3 / Hour}
define_units_relation! {MilePerHour == Mile / Hour}
define_units_relation! {MilePerHour:2 == Mile:2 / Hour}
define_units_relation! {MilePerHour:3 == Mile:3 / Hour}
define_units_relation! {CentiMetrePerSecond == CentiMetre / Second}
define_units_relation! {CentiMetrePerSecond:2 == CentiMetre:2 / Second}
define_units_relation! {CentiMetrePerSecond:3 == CentiMetre:3 / Second}

// AngularAcceleration == Frequency / Time
define_units_relation! {RadianPerSquareSecond == RadianPerSecond / Second}

// Frequency == Angle / Time
define_units_relation! {Hertz == Cycle / Second}
define_units_relation! {RadianPerSecond == Radian / Second}
define_units_relation! {CyclePerMinute == Cycle / Minute}

// KinematicViscosity == Area / Time
define_units_relation! {SquareMetrePerSecond == SquareMetre / Second}
define_units_relation! {SquareMetrePerSecond == SquareMilliMetre / MicroSecond}
define_units_relation! {SquareMetrePerSecond == SquareMicroMetre / PicoSecond}
define_units_relation! {Stoke == SquareCentiMetre / Second}
define_units_relation! {CentiStoke == SquareMilliMetre / Second}
define_units_relation! {CentiStoke == SquareMicroMetre / MicroSecond}
define_units_relation! {CentiStoke == SquareNanoMetre / PicoSecond}

// KinematicViscosity == Length * Velocity
define_units_relation! {SquareMetrePerSecond == Metre * MetrePerSecond}
define_units_relation! {SquareMetrePerSecond == HectoMetre * CentiMetrePerSecond}
define_units_relation! {Stoke == CentiMetre * CentiMetrePerSecond}

// SquareTime == Time * Time
define_units_relation! {SquareSecond == Second * Second}
define_units_relation! {HourSecond == Hour * Second}
define_units_relation! {HourHour == Hour * Hour}

// VolumetricFlowRate == Volume / Time
define_units_relation! {CubicMetrePerSecond == CubicMetre / Second}
define_units_relation! {MilliLitrePerSecond == MilliLitre / Second}

// VolumetricFlowRate == Area * Velocity
define_units_relation! {CubicMetrePerSecond == SquareMetre * MetrePerSecond}
define_units_relation! {MilliLitrePerSecond == SquareCentiMetre * CentiMetrePerSecond}

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
define_units_relation! {JouleSecond == Joule * Second}

// Action == Power * SquareTime
define_units_relation! {JouleSecond == Watt * SquareSecond}

// AngularMomentum == Momentum * Length
define_units_relation! {KiloGramSquareMetrePerSecond == KiloGramMetrePerSecond:2 X Metre:2}
define_units_relation! {KiloGramSquareMetrePerSecond:3 == KiloGramMetrePerSecond:3 X Metre:3}

// AngularMomentum == MomentOfInertia / Time
define_units_relation! {KiloGramSquareMetrePerSecond == KiloGramSquareMetre / Second}

// DynamicViscosity == Pressure * Time
define_units_relation! {PascalSecond == Pascal * Second}

// Energy == Force * Length
define_units_relation! {Joule == Newton * Metre}
define_units_relation! {Joule == Newton:2 * Metre:2}
define_units_relation! {Joule == Newton:3 * Metre:3}
define_units_relation! {Erg == Dyne * CentiMetre}
define_units_relation! {Erg == Dyne:2 * CentiMetre:2}
define_units_relation! {Erg == Dyne:3 * CentiMetre:3}

// Energy == Momentum * Velocity
define_units_relation! {Joule == NewtonSecond * MetrePerSecond}
define_units_relation! {Joule == NewtonSecond:2 * MetrePerSecond:2}
define_units_relation! {Joule == NewtonSecond:3 * MetrePerSecond:3}
define_units_relation! {Erg == DyneSecond * CentiMetrePerSecond}
define_units_relation! {Erg == DyneSecond:2 * CentiMetrePerSecond:2}
define_units_relation! {Erg == DyneSecond:3 * CentiMetrePerSecond:3}

// Energy == MomentOfInertia / SquareTime
define_units_relation! {Joule == KiloGramSquareMetre / SquareSecond}

// EnergyDensity == Energy / Volume
define_units_relation! {JoulePerCubicMetre == Joule / CubicMetre}

// Force == Mass * Acceleration
define_units_relation! {Newton == KiloGram * MetrePerSquareSecond}
define_units_relation! {Newton:2 == KiloGram * MetrePerSquareSecond:2}
define_units_relation! {Newton:3 == KiloGram * MetrePerSquareSecond:3}
define_units_relation! {Dyne == Gram * CentiMetrePerSquareSecond}
define_units_relation! {Dyne:2 == Gram * CentiMetrePerSquareSecond:2}
define_units_relation! {Dyne:3 == Gram * CentiMetrePerSquareSecond:3}
define_units_relation! {KiloGramForce == KiloGram * GForce}
define_units_relation! {KiloGramForce:2 == KiloGram * GForce:2}
define_units_relation! {KiloGramForce:3 == KiloGram * GForce:3}

// LinearDensity == Mass / Length
define_units_relation! {KiloGramPerMetre == KiloGram / Metre}
define_units_relation! {GramPerCentiMetre == Gram / CentiMetre}

// MassDensity == Mass / Volume
define_units_relation! {KiloGramPerCubicMetre == KiloGram / CubicMetre}
define_units_relation! {GramPerMilliLitre == Gram / MilliLitre}

// MassFlowRate == Mass / Time
define_units_relation! {KiloGramPerSecond == KiloGram / Second}
define_units_relation! {GramPerSecond == Gram / Second}

// MomentOfInertia == Mass * Area
define_units_relation! {KiloGramSquareMetre == KiloGram * SquareMetre}
define_units_relation! {GramSquareCentiMetre == Gram * SquareCentiMetre}

// Momentum == Force * Time
define_units_relation! {NewtonSecond == Newton * Second}
define_units_relation! {NewtonSecond:2 == Newton:2 * Second}
define_units_relation! {NewtonSecond:3 == Newton:3 * Second}
define_units_relation! {DyneSecond == Dyne * Second}
define_units_relation! {DyneSecond:2 == Dyne:2 * Second}
define_units_relation! {DyneSecond:3 == Dyne:3 * Second}

// Momentum == Mass * Velocity
define_units_relation! {NewtonSecond == KiloGram * MetrePerSecond}
define_units_relation! {NewtonSecond:2 == KiloGram * MetrePerSecond:2}
define_units_relation! {NewtonSecond:3 == KiloGram * MetrePerSecond:3}
define_units_relation! {DyneSecond == Gram * CentiMetrePerSecond}
define_units_relation! {DyneSecond:2 == Gram * CentiMetrePerSecond:2}
define_units_relation! {DyneSecond:3 == Gram * CentiMetrePerSecond:3}

// Power == Energy / Time
define_units_relation! {Watt == Joule / Second}
define_units_relation! {Watt == WattHour / Hour}
define_units_relation! {KiloWatt == KiloWattHour / Hour}
define_units_relation! {ErgPerSecond == Erg / Second}

// Pressure == Force / Area
define_units_relation! {Pascal == Newton / SquareMetre}
define_units_relation! {PoundForcePerSquareInch == PoundForce / SquareInch}
define_units_relation! {HectoPascal == Newton / SquareDeciMetre}

// SpecificEnergy == Energy / Mass
define_units_relation! {JoulePerKiloGram == Joule / KiloGram}

// SpecificVolume == Volume / Mass
define_units_relation! {CubicMetrePerKiloGram == CubicMetre / KiloGram}

// SpecificVolume == 1 / MassDensity
define_units_relation! {CubicMetrePerKiloGram == 1 / KiloGramPerCubicMetre}

// SurfaceDensity == Mass / Area
define_units_relation! {KiloGramPerSquareMetre == KiloGram / SquareMetre}

// SurfaceTension == Energy / Area
define_units_relation! {JoulePerSquareMetre == Joule / SquareMetre}

// Torque == Force * Length
define_units_relation! {NewtonMetre == Newton:2 X Metre:2}
define_units_relation! {NewtonMetre:3 == Newton:3 X Metre:3}
define_units_relation! {PoundFoot == PoundForce:2 X Foot:2}
define_units_relation! {PoundFoot:3 == PoundForce:3 X Foot:3}
define_units_relation! {PoundInch == PoundForce:2 X Inch:2}
define_units_relation! {PoundInch:3 == PoundForce:3 X Inch:3}

// Thermodynamics

// Properties:
// * Entropy
// * SpecificHeatCapacity
// * Temperature
// * ThermalConductivity

// Entropy == Energy / Temperature
define_units_relation! {JoulePerKelvin == Joule / Kelvin}

// SpecificHeatCapacity == Entropy / Mass
define_units_relation! {JoulePerKiloGramPerKelvin == JoulePerKelvin / KiloGram}

// TODO: ThermalConductivity == Power / Length / Temperature
// TODO: define_units_relation! {WattPerMetrePerKelvin == ?}

// Chemistry

// Properties:
// * CatalyticActivity
// * ChemicalPotential
// * MolarConcentration
// * MolarHeatCapacity
// * ReactionRate

// CatalyticActivity == Amount / Time
define_units_relation! {Katal == Mole / Second}

// ChemicalPotential == Energy / Amount
define_units_relation! {JoulePerMole == Joule / Mole}

// MolarConcentration == Amount / Volume
define_units_relation! {MolePerCubicMetre == Mole / CubicMetre}

// MolarHeatCapacity == ChemicalPotential / Temperature
define_units_relation! {JoulePerKelvinPerMole == JoulePerMole / Kelvin}

// ReactionRate == MolarConcentration / Time
define_units_relation! {MolePerCubicMetrePerSecond == MolePerCubicMetre / Second}

// ReactionRate == CatalyticActivity / Volume
define_units_relation! {MolePerCubicMetrePerSecond == Katal / CubicMetre}

// Radioactivity

// Properties:
// * DoseEquivalent
// * RadioactiveActivity
// * RadioactiveDose
// * RadioactiveDoseRate

// RadioactiveDoseRate == RadioactiveDose / Time
define_units_relation! {GrayPerSecond == Gray / Second}

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
define_units_relation! {Lux == Lumen / SquareMetre}
define_units_relation! {Lux == CandelaPerSquareMetre * Steradian}
define_units_relation! {Phot == Lumen / SquareCentiMetre}
define_units_relation! {Phot == Stilb * Steradian}
define_units_relation! {FootCandle == Lumen / SquareFoot}
define_units_relation! {FootCandle == CandelaPerSquareFoot * Steradian}

// Irradiance == Power / Area
define_units_relation! {WattPerSquareMetre == Watt / SquareMetre}

// Luminance == LuminousIntensity / Area
define_units_relation! {CandelaPerSquareMetre == Candela / SquareMetre}
define_units_relation! {Stilb == Candela / SquareCentiMetre}
define_units_relation! {CandelaPerSquareFoot == Candela / SquareFoot}

// LuminousFlux == LuminousIntensity * SolidAngle
define_units_relation! {Lumen == Candela * Steradian}

// Radiance == RadiantIntensity / Area
define_units_relation! {WattPerSquareMetrePerSteradian == WattPerSteradian / SquareMetre}

// Radiance == Irradiance / SolidAngle
define_units_relation! {WattPerSquareMetrePerSteradian == WattPerSquareMetre / Steradian}

// RadiantIntensity == Power / SolidAngle
define_units_relation! {WattPerSteradian == Watt / Steradian}

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
define_units_relation! {Farad == Coulomb / Volt}
define_units_relation! {MilliFarad == MilliCoulomb / Volt}
define_units_relation! {MilliFarad == Coulomb / KiloVolt}
define_units_relation! {MicroFarad == MicroCoulomb / Volt}
define_units_relation! {MicroFarad == MilliCoulomb / KiloVolt}
define_units_relation! {NanoFarad == NanoCoulomb / Volt}
define_units_relation! {NanoFarad == MicroCoulomb / KiloVolt}
define_units_relation! {PicoFarad == PicoCoulomb / Volt}
define_units_relation! {PicoFarad == NanoCoulomb / KiloVolt}

// CurrentDensity == ElectricCurrent * Area
define_units_relation! {AmperePerSquareMetre == Ampere / SquareMetre}

// ElectricalConductance == ElectricCurrent / ElectricPotential
define_units_relation! {Siemens == Ampere / Volt}
define_units_relation! {Siemens == MilliAmpere / KiloVolt}

// ElectricalConductance == 1 / ElectricalResistance
define_units_relation! {Siemens == 1 / Ohm}

// ElectricalConductivity == ElectricalConductance / Length
define_units_relation! {SiemensPerMetre == Siemens / Metre}

// ElectricalResistance == ElectricPotential / ElectricCurrent
define_units_relation! {Ohm == Volt / Ampere}

// ElectricCurrent == ElectricCharge / Time
define_units_relation! {Ampere == Coulomb / Second}
define_units_relation! {Ampere == MilliCoulomb / MilliSecond}
define_units_relation! {MilliAmpere == MilliCoulomb / Second}
define_units_relation! {MilliAmpere == MicroCoulomb / MilliSecond}
define_units_relation! {MicroAmpere == MicroCoulomb / Second}

// ElectricChargeDensity == ElectricCharge / Volume
define_units_relation! {CoulombPerCubicMetre == Coulomb / CubicMetre}
define_units_relation! {CoulombPerCubicMetre == MilliCoulomb / Litre}
define_units_relation! {CoulombPerCubicMetre == MicroCoulomb / MilliLitre}

// ElectricDisplacement == ElectricCharge / Area
define_units_relation! {CoulombPerSquareMetre == Coulomb / SquareMetre}
define_units_relation! {CoulombPerSquareMetre == MicroCoulomb / SquareMilliMetre}

// ElectricFieldStrength == ElectricPotential * Length
define_units_relation! {VoltPerMetre == Volt / Metre}
define_units_relation! {NewtonPerCoulomb == Newton / Coulomb}

// ElectricPotential == Power / ElectricCurrent
define_units_relation! {Volt == Watt / Ampere}
define_units_relation! {Volt == MilliWatt / MilliAmpere}
define_units_relation! {KiloVolt == Watt / MilliAmpere}
define_units_relation! {KiloVolt == MilliWatt / MicroAmpere}
define_units_relation! {KiloVolt == KiloWatt / Ampere}
define_units_relation! {MilliVolt == MilliWatt / Ampere}

// LinearElectricChargeDensity == ElectricCharge / Length
define_units_relation! {CoulombPerMetre == Coulomb / Metre}
define_units_relation! {CoulombPerMetre == MilliCoulomb / MilliMetre}
define_units_relation! {CoulombPerMetre == MicroCoulomb / MicroMetre}

// Permittivity == Capacitance / Length
define_units_relation! {FaradPerMetre == Farad / Metre}
define_units_relation! {FaradPerMetre == MilliFarad / MilliMetre}
define_units_relation! {FaradPerMetre == MicroFarad / MicroMetre}
define_units_relation! {FaradPerMetre == NanoFarad / NanoMetre}

// Magnetism

// Properties:
// * Inductance
// * MagneticFieldStrength
// * MagneticFlux
// * MagneticFluxDensity
// * MagneticPermeability
// * MagneticReluctance

// Inductance == MagneticFlux / ElectricCurrent
define_units_relation! {Henry == Weber / Ampere}

// MagneticFieldStrength == ElectricCurrent / Length
define_units_relation! {AmperePerMetre == Ampere / Metre}
define_units_relation! {AmperePerMetre == MilliAmpere / MilliMetre}
define_units_relation! {AmperePerMetre == MicroAmpere / MicroMetre}

// TODO: MagneticFlux == Mass * Area / SquareTime / Current
// TODO: MagneticFlux == Force * Length / Current

// MagneticFluxDensity == MagneticFlux / Area
// TODO: MagneticFluxDensity == Mass / SquareTime / Current
// TODO: MagneticFluxDensity == Force / Length / Current

// MagneticPermeability == Inductance / Length

// MagneticReluctance == 1 / Inductance
define_units_relation! {InverseHenry == 1 / Henry}

// Others

// Properties:
// * Amount
// * Dimensionless

// Dimensionless == Dimensionless * Dimensionless
define_units_relation! {Unspecified == Unspecified:2 * Unspecified:2}
define_units_relation! {Unspecified == Unspecified:2 X Unspecified:2}
define_units_relation! {Unspecified == Unspecified:3 * Unspecified:3}
define_units_relation! {Unspecified:3 == Unspecified:3 X Unspecified:3}
// N.B.: The following definition is not allowed:
// ```
// define_units_relation! {Unspecified == Unspecified * Unspecified}
// ```
// This is because it would imply that `Measure<Unspecified> / Measure<Unspecified>`
// is a `Measure<Unspecified>`. Though, in general, that division is already defined
// to be a number.
