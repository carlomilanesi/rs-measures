use std::f64::consts::TAU;

use rs_measures::define_measure_3d;
define_measure_3d! {}

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

pub struct GForce;
impl MeasurementUnit for GForce {
    type Property = Acceleration;
    const RATIO: f64 = 9.8;
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
pub struct Turn;
impl MeasurementUnit for Turn {
    type Property = Angle;
    const RATIO: f64 = TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Turn {
    const TURN_FRACTION: f64 = 1.;
}

pub struct Gradian;
impl MeasurementUnit for Gradian {
    type Property = Angle;
    const RATIO: f64 = TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const TURN_FRACTION: f64 = 400.;
}

pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}

pub struct ArcMinute;
impl MeasurementUnit for ArcMinute {
    type Property = Angle;
    const RATIO: f64 = TAU / 360. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg'";
}
impl AngleMeasurementUnit for ArcMinute {
    const TURN_FRACTION: f64 = 360. * 60.;
}

pub struct ArcSecond;
impl MeasurementUnit for ArcSecond {
    type Property = Angle;
    const RATIO: f64 = TAU / 360. / 60. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\"";
}
impl AngleMeasurementUnit for ArcSecond {
    const TURN_FRACTION: f64 = 360. * 60. * 60.;
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

pub struct KilogramSquareMetrePerSecond;
impl MeasurementUnit for KilogramSquareMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}/s";
}
impl VectorMeasurementUnit for KilogramSquareMetrePerSecond {}

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
    const SUFFIX: &'static str = " \u{B5}F";
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

// Property: catalytic activity concentration
pub struct CatalyticActivityConcentration;

pub struct KatalPerCubicMetre;
impl MeasurementUnit for KatalPerCubicMetre {
    type Property = CatalyticActivityConcentration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kat/m\u{B3}";
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
    const SUFFIX: &'static str = " \u{3A9}";
}

pub struct MilliOhm;
impl MeasurementUnit for MilliOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{3A9}";
}

pub struct KiloOhm;
impl MeasurementUnit for KiloOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " k\u{3A9}";
}

// Property: electrical resistivity
pub struct ElectricalResistivity;

pub struct OhmMetre;
impl MeasurementUnit for OhmMetre {
    type Property = ElectricalResistivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3A9}\u{b7}m";
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
    const SUFFIX: &'static str = " \u{B5}C";
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
    const SUFFIX: &'static str = " C/m\u{B3}";
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
    const SUFFIX: &'static str = " \u{B5}A";
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

pub struct VoltPerMetre; // a.k.a. newton per coulomb
impl MeasurementUnit for VoltPerMetre {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V/m";
}
impl VectorMeasurementUnit for VoltPerMetre {}

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
    const SUFFIX: &'static str = " \u{B5}V";
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
    const SUFFIX: &'static str = " J/m\u{B3}";
}

// Property: entropy, heat capacity
pub struct Entropy;

pub struct JoulePerKelvin;
impl MeasurementUnit for JoulePerKelvin {
    type Property = Entropy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{B0}K";
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

pub struct KilogramForce;
impl MeasurementUnit for KilogramForce {
    type Property = Force;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kgf";
}
impl VectorMeasurementUnit for KilogramForce {}

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
    const RATIO: f64 = 1. / TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s";
}

pub struct TurnPerMinute;
impl MeasurementUnit for TurnPerMinute {
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
    const SUFFIX: &'static str = " \u{B5}m";
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
    const SUFFIX: &'static str = " \u{212B}";
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

pub struct KilogramPerMetre;
impl MeasurementUnit for KilogramPerMetre {
    type Property = LinearDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m";
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

pub struct CandelaPerSquareMetre; // a.k.a. nit
impl MeasurementUnit for CandelaPerSquareMetre {
    type Property = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd/m\u{b2}";
}

pub struct Stilb;
impl MeasurementUnit for Stilb {
    type Property = Luminance;
    const RATIO: f64 = 10000.;
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

pub struct Tonne; // a.k.a. metric ton or megagram
impl MeasurementUnit for Tonne {
    type Property = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
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
    const SUFFIX: &'static str = " \u{B5}g";
}

pub struct NanoGram;
impl MeasurementUnit for NanoGram {
    type Property = Mass;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ng";
}

pub struct ImperialTon; // a.k.a. long ton or weight ton
impl MeasurementUnit for ImperialTon {
    type Property = Mass;
    const RATIO: f64 = 1016.0469;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

pub struct USTon; // a.k.a. short ton
impl MeasurementUnit for USTon {
    type Property = Mass;
    const RATIO: f64 = 907.18474;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

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
    const SUFFIX: &'static str = " kg/m\u{B3}";
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
    const SUFFIX: &'static str = " mol/m\u{B3}";
}

// Property: molar heat capacity, molar entropy
pub struct MolarHeatCapacity;

pub struct JoulePerKelvinPerMole;
impl MeasurementUnit for JoulePerKelvinPerMole {
    type Property = MolarHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{B0}K/mol";
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

pub struct DynSecond;
impl MeasurementUnit for DynSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn\u{b7}s";
}
impl VectorMeasurementUnit for DynSecond {}

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

// Property: reaction rate
pub struct ReactionRate;

pub struct MolePerCubicMetrePerSecond;
impl MeasurementUnit for MolePerCubicMetrePerSecond {
    type Property = ReactionRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{B3}/s";
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

pub struct AllRound;
impl MeasurementUnit for AllRound {
    type Property = SolidAngle;
    const RATIO: f64 = 2. * TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sphere";
}

pub struct SquareDegree;
impl MeasurementUnit for SquareDegree {
    type Property = SolidAngle;
    const RATIO: f64 = TAU * TAU / 360. / 360.;
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
    const SUFFIX: &'static str = " J/kg/\u{B0}K";
}

// Property: specific volume
pub struct SpecificVolume;

pub struct CubicMetrePerKiloGram;
impl MeasurementUnit for CubicMetrePerKiloGram {
    type Property = SpecificVolume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{B3}/kg";
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
    const SUFFIX: &'static str = " \u{B0}K";
}

pub struct Celsius;
impl MeasurementUnit for Celsius {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 273.15;
    const SUFFIX: &'static str = " \u{B0}C";
}

pub struct Fahrenheit;
impl MeasurementUnit for Fahrenheit {
    type Property = Temperature;
    const RATIO: f64 = 5. / 9.;
    const OFFSET: f64 = 273.15 - 32. * 5. / 9.;
    const SUFFIX: &'static str = " \u{B0}F";
}

// Property: thermal conductivity
pub struct ThermalConductivity;

pub struct WattPerMetrePerKelvin;
impl MeasurementUnit for WattPerMetrePerKelvin {
    type Property = ThermalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m/\u{B0}K";
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
    const SUFFIX: &'static str = " \u{B5}s";
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

pub struct CentiMetresPerSecond;
impl MeasurementUnit for CentiMetresPerSecond {
    type Property = Velocity;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s";
}
impl VectorMeasurementUnit for CentiMetresPerSecond {}

// Property: volume
pub struct Volume;

pub struct CubicMetre;
impl MeasurementUnit for CubicMetre {
    type Property = Volume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{B3}";
}

pub struct CubicKiloMetre;
impl MeasurementUnit for CubicKiloMetre {
    type Property = Volume;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{B3}";
}

pub struct CubicInch;
impl MeasurementUnit for CubicInch {
    type Property = Volume;
    const RATIO: f64 = 0.0254 * 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{B3}";
}

pub struct CubicFoot;
impl MeasurementUnit for CubicFoot {
    type Property = Volume;
    const RATIO: f64 = 0.3048 * 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{B3}";
}

pub struct CubicYard;
impl MeasurementUnit for CubicYard {
    type Property = Volume;
    const RATIO: f64 = 0.9144 * 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{B3}";
}

pub struct CubicMile;
impl MeasurementUnit for CubicMile {
    type Property = Volume;
    const RATIO: f64 = 1609. * 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{B3}";
}

pub struct Litre; // a.k.a. cubic decimetre or dm\u{B3}
impl MeasurementUnit for Litre {
    type Property = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " l";
}

pub struct MilliLitre; // a.k.a. cubic centimetre or cm\u{B3}
impl MeasurementUnit for MilliLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml";
}

pub struct MicroLitre; // a.k.a. cubic millimetre or mm\u{B3}
impl MeasurementUnit for MicroLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}l";
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
    const SUFFIX: &'static str = " m\u{B3}/s";
}

pub struct CubicCentiMetrePerSecond;
impl MeasurementUnit for CubicCentiMetrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{B3}/s";
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
    const RATIO: f64 = 1. / TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/m";
}

//## Relationships among units
use define_units_relation::define_units_relation;

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
define_units_relation! {Hectare == HectoMetre * HectoMetre}
define_units_relation! {Are == DecaMetre * DecaMetre}
define_units_relation! {SquareDeciMetre == DeciMetre * DeciMetre}
define_units_relation! {SquareCentiMetre == CentiMetre * CentiMetre}
define_units_relation! {SquareMilliMetre == MilliMetre * MilliMetre}
define_units_relation! {SquareInch == Inch * Inch}
define_units_relation! {SquareFoot == Foot * Foot}
define_units_relation! {SquareYard == Yard * Yard}
define_units_relation! {SquareMile == Mile * Mile}

// Volume == Area * Length
// SolidAngle == Angle * Angle
// WaveNumber == Angle / Length

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
// Acceleration == Length / SquareTime
// Velocity == Length / Time
// AngularAcceleration == AngularVelocity / Time
// Frequency == Angle / Time
// Frequency == 1 / Time
// KinematicViscosity == Area / Time
// SquareTime = Time * Time
// VolumetricFlowRate == Volume / Time

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
// AngularMomentum == Momentum * Length
// DynamicViscosity == Pressure * Time
// Energy == Force * Length
// EnergyDensity == Energy / Volume
// Force == Mass * Acceleration
// LinearDensity == Mass / Length
// MassDensity == Mass / Volume
// MassFlowRate == Mass / Time
// MomentOfInertia == Mass * Area
// Momentum == Force * Time
// Momentum == Mass * Velocity
// Power == Energy / Time
// Pressure == Force / Area
// SpecificEnergy == Joule / Volume
// SpecificVolume == Volume / Mass
// SpecificVolume == 1 / MassDensity
// SurfaceDensity == Mass / Area
// SurfaceTension == Energy / Area
// Torque == Force * Length

// Thermodynamics

// Properties:
// * Entropy
// * SpecificHeatCapacity
// * Temperature
// * ThermalConductivity

// Entropy == Energy / Temperature
// SpecificHeatCapacity == Entropy / Mass
// TODO: ThermalConductivity == Power / Length / Temperature

// Chemistry

// Properties:
// * CatalyticActivity
// * CatalyticActivityConcentration
// * ChemicalPotential
// * MolarConcentration
// * MolarHeatCapacity
// * ReactionRate

// CatalyticActivity == Amount / Time
// CatalyticActivityConcentration == CatalyticActivity / Volume
// ChemicalPotential == Energy / Amount
// MolarConcentration == Amount / Volume
// MolarHeatCapacity == ChemicalPotential / Temperature
// ReactionRate == MolarConcentration / Time
// ReactionRate == CatalyticActivity / Volume

// Radioactivity

// Properties:
// * DoseEquivalent
// * RadioactiveActivity
// * RadioactiveDose
// * RadioactiveDoseRate

// DoseEquivalent == Energy / Mass
// RadioactiveDoseRate == RadioactiveDose / Time

// Lighting

// Properties:
// * Illuminance
// * Irradiance
// * Luminance
// * LuminousFlux
// * LuminousIntensity
// * Radiance
// * RadiantIntensity

// Illuminance == LuminousFlux / Area
// Irradiance == Power / Area
// Luminance == LuminousIntensity / Area
// LuminousFlux == LuminousIntensity * SolidAngle
// Radiance == RadiantIntensity / Area
// Radiance == Irradiance / SolidAngle
// RadiantIntensity == Power / SolidAngle

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
// CurrentDensity == ElectricCurrent / Area
// ElectricalConductance == ElectricCurrent / ElectricPotential
// ElectricalConductance == 1 / ElectricalResistance
// ElectricalConductivity == ElectricalConductance / Length
// ElectricalResistance == ElectricPotential / ElectricCurrent
// ElectricCurrent == ElectricCharge / Time
// ElectricChargeDensity == ElectricCharge / Volume
// ElectricDisplacement == ElectricCharge / Area
// ElectricFieldStrength == ElectricPotential * Length
// ElectricPotential == Power / ElectricCurrent
// LinearElectricChargeDensity == ElectricCharge / Length
// Permittivity == Capacitance / Length

// Magnetism

// Properties:
// * Inductance
// * MagneticFieldStrength
// * MagneticFlux
// * MagneticFluxDensity
// * MagneticPermeability
// * MagneticReluctance

// Inductance == MagneticFlux / ElectricCurrent
// MagneticFieldStrength == ElectricCurrent / Length
// MagneticFlux == ElectricCurrent * Inductance
// TODO: MagneticFlux == Mass * Area / SquareTime / Current
// MagneticFluxDensity == MagneticFlux / Area
// TODO: MagneticFluxDensity == Mass / SquareTime / Current
// MagneticPermeability == Inductance / Length
// MagneticReluctance == 1 / Inductance

// Others

// Properties:
// * Amount
// * Dimensionless

// geometry //

// Area == Length * Length
define_units_relation! {SquareMetre == Metre * Metre}
define_units_relation! {SquareKiloMetre == KiloMetre * KiloMetre}

/*TODO
define_derived_measure_squared_1! {}
define_derived_measure_squared_1! {HectoMetre, Hectare}
define_derived_measure_squared_1! {DecaMetre, Are}
define_derived_measure_squared_1! {DeciMetre, SquareDeciMetre}
define_derived_measure_squared_1! {CentiMetre, SquareCentiMetre}
define_derived_measure_squared_1! {MilliMetre, SquareMilliMetre}
define_derived_measure_squared_1! {Inch, SquareInch}
define_derived_measure_squared_1! {Foot, SquareFoot}
define_derived_measure_squared_1! {Yard, SquareYard}
define_derived_measure_squared_1! {Mile, SquareMile}

// Length * Area = Volume
define_derived_measure_1_1! {Metre, SquareMetre, CubicMetre}
define_derived_measure_1_1! {KiloMetre, SquareKiloMetre, CubicKiloMetre}
define_derived_measure_1_1! {DeciMetre, SquareDeciMetre, Litre}
define_derived_measure_1_1! {CentiMetre, SquareCentiMetre, MilliLitre}
define_derived_measure_1_1! {MilliMetre, SquareMilliMetre, MicroLitre}
define_derived_measure_1_1! {Inch, SquareInch, CubicInch}
define_derived_measure_1_1! {Foot, SquareFoot, CubicFoot}
define_derived_measure_1_1! {Yard, SquareYard, CubicYard}
define_derived_measure_1_1! {Mile, SquareMile, CubicMile}
*/
// kinematics //

// Velocity == Length / Time
define_units_relation! {Metre / Second == MetrePerSecond}
define_units_relation! {Metre:2 / Second == MetrePerSecond:2}
define_units_relation! {Metre:3 / Second == MetrePerSecond:3}
/*
define_derived_measure_1_1! {Hour, Knot, NauticalMile}
define_derived_measure_1_2! {Hour, Knot, NauticalMile}
define_derived_measure_1_3! {Hour, Knot, NauticalMile}
define_derived_measure_1_1! {Hour, KiloMetrePerHour, KiloMetre}
define_derived_measure_1_2! {Hour, KiloMetrePerHour, KiloMetre}
define_derived_measure_1_3! {Hour, KiloMetrePerHour, KiloMetre}
define_derived_measure_1_1! {Hour, MilePerHour, Mile}
define_derived_measure_1_2! {Hour, MilePerHour, Mile}
define_derived_measure_1_3! {Hour, MilePerHour, Mile}

// SquareTime * Acceleration = Length
define_derived_measure_1_1! {SquareSecond, MetrePerSquareSecond, Metre}
define_derived_measure_1_2! {SquareSecond, MetrePerSquareSecond, Metre}
define_derived_measure_1_3! {SquareSecond, MetrePerSquareSecond, Metre}
define_derived_measure_1_1! {HourSecond, KiloMetrePerHourPerSecond, KiloMetre}
define_derived_measure_1_2! {HourSecond, KiloMetrePerHourPerSecond, KiloMetre}
define_derived_measure_1_3! {HourSecond, KiloMetrePerHourPerSecond, KiloMetre}

// Time * Acceleration = Velocity
define_derived_measure_1_1! {Second, MetrePerSquareSecond, MetrePerSecond}
define_derived_measure_1_2! {Second, MetrePerSquareSecond, MetrePerSecond}
define_derived_measure_1_3! {Second, MetrePerSquareSecond, MetrePerSecond}
define_derived_measure_1_1! {Second, KiloMetrePerHourPerSecond, KiloMetrePerHour}
define_derived_measure_1_2! {Second, KiloMetrePerHourPerSecond, KiloMetrePerHour}
define_derived_measure_1_3! {Second, KiloMetrePerHourPerSecond, KiloMetrePerHour}

// Time * AngularVelocity = Angle
define_derived_measure_1_1! {Second, RadianPerSecond, Radian}
define_derived_measure_1_1! {Second, Hertz, Turn}
define_derived_measure_1_1! {Minute, TurnPerMinute, Turn}
define_derived_measure_1_1! {MilliSecond, KiloHertz, Turn}
define_derived_measure_1_1! {MicroSecond, MegaHertz, Turn}
define_derived_measure_1_1! {NanoSecond, GigaHertz, Turn}

// SquareTime * AngularAcceleration = Angle
define_derived_measure_1_1! {SquareSecond, RadianPerSquareSecond, Radian}
define_derived_measure_1_1! {Second, RadianPerSquareSecond, RadianPerSecond}

// Time * KinematicViscosity = Area
define_derived_measure_1_1! {Second, SquareMetrePerSecond, SquareMetre}

// mechanics //

// Length * LinearDensity = Mass
define_derived_measure_1_1! {Metre, KilogramPerMetre, KiloGram}

// Area * SurfaceDensity = Mass
define_derived_measure_1_1! {SquareMetre, KiloGramPerSquareMetre, KiloGram}

// Volume * MassDensity = Mass
define_derived_measure_1_1! {CubicMetre, KiloGramPerCubicMetre, KiloGram}

// Mass * SpecificVolume = Volume
define_derived_measure_1_1! {KiloGram, CubicMetrePerKiloGram, CubicMetre}
*/

// Force * Length = Energy, Torque
define_units_relation! {Newton * Metre == Joule}
define_units_relation! {Newton:2 * Metre:2 == Joule}
define_units_relation! {Newton:3 * Metre:3 == Joule}
define_units_relation! {Newton:2 X Metre:2 == NewtonMetre}
define_units_relation! {Newton:3 X Metre:3 == NewtonMetre:3}

define_units_relation! {Metre:2 X Metre:2 == Metre}
define_units_relation! {Metre:3 X Metre:3 == Metre:3}

/*
// Mass * Acceleration = Force
define_derived_measure_1_1! {KiloGram, MetrePerSquareSecond, Newton}
define_derived_measure_1_2! {KiloGram, MetrePerSquareSecond, Newton}
define_derived_measure_1_3! {KiloGram, MetrePerSquareSecond, Newton}

// Time * Power = Energy
define_derived_measure_1_1! {Second, Watt, Joule}
define_derived_measure_1_1! {SquareMetre, Pascal, Newton}

// Mass * Velocity = Momentum
define_derived_measure_1_1! {KiloGram, MetrePerSecond, KiloGramMetrePerSecond}
define_derived_measure_1_2! {KiloGram, MetrePerSecond, KiloGramMetrePerSecond}
define_derived_measure_1_3! {KiloGram, MetrePerSecond, KiloGramMetrePerSecond}

// Momentum * Length = AngularMomentum
define_derived_measure_1_1! {KiloGramMetrePerSecond, Metre, KilogramSquareMetrePerSecond}
define_derived_measure_1_2! {KiloGramMetrePerSecond, Metre, KilogramSquareMetrePerSecond}
define_derived_measure_1_3! {KiloGramMetrePerSecond, Metre, KilogramSquareMetrePerSecond}

// Volume * EnergyDensity = Energy
define_derived_measure_1_1! {CubicMetre, JoulePerCubicMetre, Joule}

// Time * Pressure = DynamicViscosity
define_derived_measure_1_1! {Second, Pascal, PascalSecond}

// thermodynamics //

// Temperature * Entropy = Energy
define_derived_measure_1_1! {Kelvin, JoulePerKelvin, Joule}

// optics //

// LuminousIntensity * SolidAngle = LuminousFlux
define_derived_measure_1_1! {Candela, Steradian, Lumen}

// Area * Illuminance = LuminousFlux
define_derived_measure_1_1! {SquareMetre, Lux, Lumen}
define_derived_measure_1_1! {SquareCentiMetre, Phot, Lumen}
define_derived_measure_1_1! {SquareFoot, FootCandle, Lumen}

// Area * Irradiance = Power
define_derived_measure_1_1! {SquareMetre, WattPerSquareMetre, Watt}

// electromagnetism //

// ElectricCurrent * Time = ElectricCharge
define_derived_measure_1_1! {Ampere, Second, Coulomb}
define_derived_measure_1_1! {MilliAmpere, Second, MilliCoulomb}
define_derived_measure_1_1! {MicroAmpere, Second, MicroCoulomb}
define_derived_measure_1_1! {Ampere, MilliSecond, MilliCoulomb}

// Length * LinearElectricChargeDensity = ElectricCharge
define_derived_measure_1_1! {Metre, CoulombPerMetre, Coulomb}

// Area * ElectricDisplacement = ElectricCharge
define_derived_measure_1_1! {SquareMetre, CoulombPerSquareMetre, Coulomb}

// Volume * ElectricChargeDensity = ElectricCharge
define_derived_measure_1_1! {CubicMetre, CoulombPerCubicMetre, Coulomb}

// ElectricalResistance * ElectricCurrent = ElectricPotential
define_derived_measure_1_1! {Ohm, Ampere, Volt}
define_derived_measure_1_1! {Ohm, MilliAmpere, MilliVolt}
define_derived_measure_1_1! {MilliOhm, Ampere, MilliVolt}
define_derived_measure_1_1! {KiloOhm, MilliAmpere, Volt}

// ElectricalConductance * ElectricPotential = ElectricCurrent
define_derived_measure_1_1! {Siemens, Volt, Ampere}

// ElectricalConductance * ElectricalResistance = Number
define_derived_measure_inverse_1! {Siemens, Ohm}

// Length * ElectricalConductivity = ElectricalConductance
define_derived_measure_1_1! {Metre, SiemensPerMetre, Siemens}

// Capacitance * ElectricPotential = ElectricCharge
define_derived_measure_1_1! {Farad, Volt, Coulomb}
define_derived_measure_1_1! {MilliFarad, Volt, MilliCoulomb}
define_derived_measure_1_1! {MicroFarad, Volt, MicroCoulomb}
define_derived_measure_1_1! {NanoFarad, Volt, NanoCoulomb}
define_derived_measure_1_1! {NanoFarad, MilliVolt, MicroCoulomb}
define_derived_measure_1_1! {PicoFarad, Volt, PicoCoulomb}
define_derived_measure_1_1! {PicoFarad, MilliVolt, NanoCoulomb}

// Area * CurrentDensity = ElectricCurrent
define_derived_measure_1_1! {SquareMetre, AmperePerSquareMetre, Ampere}
define_derived_measure_1_2! {SquareMetre, AmperePerSquareMetre, Ampere}
define_derived_measure_1_3! {SquareMetre, AmperePerSquareMetre, Ampere}

// ElectricPotential * Length = ElectricFieldStrength
define_derived_measure_1_1! {Volt, Metre, VoltPerMetre}
define_derived_measure_1_2! {Volt, Metre, VoltPerMetre}
define_derived_measure_1_3! {Volt, Metre, VoltPerMetre}

// MagneticFieldStrength * Length = ElectricCurrent
define_derived_measure_1_1! {AmperePerMetre, Metre, Ampere}
define_derived_measure_2_2! {AmperePerMetre, Metre, Ampere, Ampere}
define_derived_measure_3_3! {AmperePerMetre, Metre, Ampere, Ampere}

// Inductance * MagneticReluctance = number
define_derived_measure_inverse_1! {Henry, InverseHenry}

// Length * MagneticPermeability = Inductance
define_derived_measure_1_1! {Metre, HenryPerMetre, Henry}

// MagneticReluctance * MagneticFlux = ElectricCurrent
define_derived_measure_1_1! {InverseHenry, Weber, Ampere}

// ElectricCurrent * Inductance = MagneticFlux
define_derived_measure_1_1! {Ampere, Henry, Weber}

// chemistry //

// Time * CatalyticActivity = Amount
define_derived_measure_1_1! {Second, Katal, Mole}

// Volume * CatalyticActivityConcentration = CatalyticActivity
define_derived_measure_1_1! {CubicMetre, KatalPerCubicMetre, Katal}

// Amount * ChemicalPotential = Energy
define_derived_measure_1_1! {Mole, JoulePerMole, Joule}

// Volume * MolarConcentration = Amount
define_derived_measure_1_1! {CubicMetre, MolePerCubicMetre, Mole}

// radioactivity //

// Mass * DoseEquivalent = Energy
define_derived_measure_1_1! {KiloGram, Sievert, Joule}

// SquareTime * DoseEquivalent = Area
define_derived_measure_1_1! {SquareSecond, Sievert, SquareMetre, Sievert}

// Time * RadioactiveDoseRate = RadioactiveDose
define_derived_measure_1_1! {Second, GrayPerSecond, Gray}

// Others //

//define_derived_measure_squared_2! {Metre, Metre, Metre}
//define_derived_measure_squared_3! {Metre, Metre, Metre}
*/

//TODO
//luxes = candelas * steradians / square_Metres
//luxes = lumens / square_Metres
//foot_candles = lumens / square_feet
