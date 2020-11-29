use std::f64::consts::TAU;

use rs_measures::{
    angle::{Angle, Radian},
    define_derived_measure_1_1, define_derived_measure_1_2, define_derived_measure_1_3,
    define_derived_measure_2_2, define_derived_measure_3_3, define_derived_measure_inverse_1,
    define_derived_measure_squared_1, define_derived_measure_squared_2,
    define_derived_measure_squared_3, define_measure1d, define_measure2d, define_measure3d,
    traits::{AngleMeasurementUnit, CrossProduct, Sqrt},
};
define_measure1d! {}
define_measure2d! {}
define_measure3d! {}

/*
const YOTTA: f64 = 1e24;
const ZETTA: f64 = 1e21;
const EXA: f64 = 1e18;
const PETA: f64 = 1e15;
const TERA: f64 = 1e12;
const GIGA: f64 = 1e9;
const MEGA: f64 = 1e6;
const KILO: f64 = 1e3;
const HECTO: f64 = 1e2;
const DEKA: f64 = 1e1;
const DECI: f64 = 1e-1;
const CENTI: f64 = 1e-2;
const MILLI: f64 = 1e-3;
const MICRO: f64 = 1e-6;
const NANO: f64 = 1e-9;
const PICO: f64 = 1e-12;
const FEMTO: f64 = 1e-15;
const ATTO: f64 = 1e-18;
const ZEPTO: f64 = 1e-21;
const YOCTO: f64 = 1e-24;
*/

// Angle
#[derive(Debug, Clone, Copy)]
struct Turn;
impl MeasurementUnit for Turn {
    type Quantity = Angle;
    const RATIO: f64 = TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rev";
}
impl AngleMeasurementUnit for Turn {
    const TURN_FRACTION: f64 = 1.;
}

#[derive(Debug, Clone, Copy)]
struct Gradian;
impl MeasurementUnit for Gradian {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const TURN_FRACTION: f64 = 400.;
}

#[derive(Debug, Clone, Copy)]
struct Degree;
impl MeasurementUnit for Degree {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const TURN_FRACTION: f64 = 360.;
}

#[derive(Debug, Clone, Copy)]
struct ArcMinute;
impl MeasurementUnit for ArcMinute {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 360. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg'";
}
impl AngleMeasurementUnit for ArcMinute {
    const TURN_FRACTION: f64 = 360. * 60.;
}

#[derive(Debug, Clone, Copy)]
struct ArcSecond;
impl MeasurementUnit for ArcSecond {
    type Quantity = Angle;
    const RATIO: f64 = TAU / 360. / 60. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\"";
}
impl AngleMeasurementUnit for ArcSecond {
    const TURN_FRACTION: f64 = 360. * 60. * 60.;
}

// Acceleration
struct Acceleration;

#[derive(Debug, Clone, Copy)]
struct MetrePerSquareSecond;
impl MeasurementUnit for MetrePerSquareSecond {
    type Quantity = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct GForce;
impl MeasurementUnit for GForce {
    type Quantity = Acceleration;
    const RATIO: f64 = 9.8;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

#[derive(Debug, Clone, Copy)]
struct KiloMetrePerHourPerSecond;
impl MeasurementUnit for KiloMetrePerHourPerSecond {
    type Quantity = Acceleration;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h/s";
}

// Action
struct Action;

#[derive(Debug, Clone, Copy)]
struct JouleSecond;
impl MeasurementUnit for JouleSecond {
    type Quantity = Action;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J\u{b7}s";
}

// Amount of substance, Count
struct Amount;

#[derive(Debug, Clone, Copy)]
struct Unit;
impl MeasurementUnit for Unit {
    type Quantity = Amount;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u.";
}

#[derive(Debug, Clone, Copy)]
struct Dozen;
impl MeasurementUnit for Dozen {
    type Quantity = Amount;
    const RATIO: f64 = 12.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dz.";
}

#[derive(Debug, Clone, Copy)]
struct Mole;
impl MeasurementUnit for Mole {
    type Quantity = Amount;
    const RATIO: f64 = 6.0221413e23;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol";
}

// Angular acceleration
struct AngularAcceleration;

#[derive(Debug, Clone, Copy)]
struct RadianPerSquareSecond;
impl MeasurementUnit for RadianPerSquareSecond {
    type Quantity = AngularAcceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s\u{b2}";
}

// Angular momentum, Spin
struct AngularMomentum;

#[derive(Debug, Clone, Copy)]
struct KilogramSquareMetrePerSecond;
impl MeasurementUnit for KilogramSquareMetrePerSecond {
    type Quantity = AngularMomentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}/s";
}

#[derive(Debug, Clone, Copy)]
struct GramSquareCentiMetrePerSecond;
impl MeasurementUnit for GramSquareCentiMetrePerSecond {
    type Quantity = AngularMomentum;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}/s";
}

// Area
struct Area;

#[derive(Debug, Clone, Copy)]
struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Quantity = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareKiloMetre;
impl MeasurementUnit for SquareKiloMetre {
    type Quantity = Area;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct Hectare;
impl MeasurementUnit for Hectare {
    type Quantity = Area;
    const RATIO: f64 = 1e4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ha";
}

#[derive(Debug, Clone, Copy)]
struct Are;
impl MeasurementUnit for Are {
    type Quantity = Area;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " are";
}

#[derive(Debug, Clone, Copy)]
struct SquareDeciMetre;
impl MeasurementUnit for SquareDeciMetre {
    type Quantity = Area;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareCentiMetre;
impl MeasurementUnit for SquareCentiMetre {
    type Quantity = Area;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareMilliMetre;
impl MeasurementUnit for SquareMilliMetre {
    type Quantity = Area;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareInch;
impl MeasurementUnit for SquareInch {
    type Quantity = Area;
    const RATIO: f64 = 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareFoot;
impl MeasurementUnit for SquareFoot {
    type Quantity = Area;
    const RATIO: f64 = 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareYard;
impl MeasurementUnit for SquareYard {
    type Quantity = Area;
    const RATIO: f64 = 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{b2}";
}

#[derive(Debug, Clone, Copy)]
struct SquareMile;
impl MeasurementUnit for SquareMile {
    type Quantity = Area;
    const RATIO: f64 = 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{b2}";
}

// Area density
struct AreaDensity;

#[derive(Debug, Clone, Copy)]
struct KilogramPerSquareMetre;
impl MeasurementUnit for KilogramPerSquareMetre {
    type Quantity = AreaDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Kg/m\u{b2}";
}

// Capacitance
struct Capacitance;

#[derive(Debug, Clone, Copy)]
struct Farad;
impl MeasurementUnit for Farad {
    type Quantity = Capacitance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F";
}

#[derive(Debug, Clone, Copy)]
struct MilliFarad;
impl MeasurementUnit for MilliFarad {
    type Quantity = Capacitance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mF";
}

#[derive(Debug, Clone, Copy)]
struct MicroFarad;
impl MeasurementUnit for MicroFarad {
    type Quantity = Capacitance;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}F";
}

#[derive(Debug, Clone, Copy)]
struct NanoFarad;
impl MeasurementUnit for NanoFarad {
    type Quantity = Capacitance;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nF";
}

#[derive(Debug, Clone, Copy)]
struct PicoFarad;
impl MeasurementUnit for PicoFarad {
    type Quantity = Capacitance;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pF";
}

// Catalytic activity
struct CatalyticActivity;

#[derive(Debug, Clone, Copy)]
struct Katal;
impl MeasurementUnit for Katal {
    type Quantity = CatalyticActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kat";
}

// Catalytic activity concentration
struct CatalyticActivityConcentration;

#[derive(Debug, Clone, Copy)]
struct KatalPerCubicMetre;
impl MeasurementUnit for KatalPerCubicMetre {
    type Quantity = CatalyticActivityConcentration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kat/m\u{B3}";
}

// Chemical potential, Molar energy
struct ChemicalPotential;

#[derive(Debug, Clone, Copy)]
struct JoulePerMole;
impl MeasurementUnit for JoulePerMole {
    type Quantity = ChemicalPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/mol";
}

// Molar concentration
struct MolarConcentration;

#[derive(Debug, Clone, Copy)]
struct MolePerCubicMetre;
impl MeasurementUnit for MolePerCubicMetre {
    type Quantity = MolarConcentration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{B3}";
}

// current density
struct CurrentDensity;

#[derive(Debug, Clone, Copy)]
struct AmperePerSquareMetre;
impl MeasurementUnit for AmperePerSquareMetre {
    type Quantity = CurrentDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m\u{b2}";
}

// Dose equivalent
struct DoseEquivalent;

#[derive(Debug, Clone, Copy)]
struct Sievert;
impl MeasurementUnit for Sievert {
    type Quantity = DoseEquivalent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Sv";
}

#[derive(Debug, Clone, Copy)]
struct Rem;
impl MeasurementUnit for Rem {
    type Quantity = DoseEquivalent;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rem";
}

// dynamic viscosity, absolute viscosity
struct DynamicViscosity;

#[derive(Debug, Clone, Copy)]
struct PascalSecond;
impl MeasurementUnit for PascalSecond {
    type Quantity = DynamicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa\u{b7}s";
}

// electric charge
struct ElectricCharge;

#[derive(Debug, Clone, Copy)]
struct Coulomb;
impl MeasurementUnit for Coulomb {
    type Quantity = ElectricCharge;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C";
}

#[derive(Debug, Clone, Copy)]
struct MilliCoulomb;
impl MeasurementUnit for MilliCoulomb {
    type Quantity = ElectricCharge;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mC";
}

#[derive(Debug, Clone, Copy)]
struct MicroCoulomb;
impl MeasurementUnit for MicroCoulomb {
    type Quantity = ElectricCharge;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}C";
}

#[derive(Debug, Clone, Copy)]
struct NanoCoulomb;
impl MeasurementUnit for NanoCoulomb {
    type Quantity = ElectricCharge;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nC";
}

#[derive(Debug, Clone, Copy)]
struct PicoCoulomb;
impl MeasurementUnit for PicoCoulomb {
    type Quantity = ElectricCharge;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pC";
}

// Linear electric charge density
struct LinearElectricChargeDensity;

#[derive(Debug, Clone, Copy)]
struct CoulombPerMetre;
impl MeasurementUnit for CoulombPerMetre {
    type Quantity = LinearElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m";
}

// electric displacement, surface electric charge density
struct ElectricDisplacement;

#[derive(Debug, Clone, Copy)]
struct CoulombPerSquareMetre;
impl MeasurementUnit for CoulombPerSquareMetre {
    type Quantity = ElectricDisplacement;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{b2}";
}

// Electric charge density
struct ElectricChargeDensity;

#[derive(Debug, Clone, Copy)]
struct CoulombPerCubicMetre;
impl MeasurementUnit for CoulombPerCubicMetre {
    type Quantity = ElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{B3}";
}

// electric field strength
struct ElectricFieldStrength;

#[derive(Debug, Clone, Copy)]
struct VoltMetre;
impl MeasurementUnit for VoltMetre {
    type Quantity = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V/m";
}

// Electrical conductance, electric susceptance, electric admittance
struct ElectricalConductance;

#[derive(Debug, Clone, Copy)]
struct Siemens;
impl MeasurementUnit for Siemens {
    type Quantity = ElectricalConductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S";
}

// Electrical conductivity
struct ElectricalConductivity;

#[derive(Debug, Clone, Copy)]
struct SiemensPerMetre;
impl MeasurementUnit for SiemensPerMetre {
    type Quantity = ElectricalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S/m";
}

// electric current
struct ElectricCurrent;

#[derive(Debug, Clone, Copy)]
struct Ampere;
impl MeasurementUnit for Ampere {
    type Quantity = ElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A";
}

#[derive(Debug, Clone, Copy)]
struct MilliAmpere;
impl MeasurementUnit for MilliAmpere {
    type Quantity = ElectricCurrent;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mA";
}

#[derive(Debug, Clone, Copy)]
struct MicroAmpere;
impl MeasurementUnit for MicroAmpere {
    type Quantity = ElectricCurrent;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}A";
}

// Electric potential
struct ElectricPotential;

#[derive(Debug, Clone, Copy)]
struct Volt;
impl MeasurementUnit for Volt {
    type Quantity = ElectricPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V";
}

#[derive(Debug, Clone, Copy)]
struct KiloVolt;
impl MeasurementUnit for KiloVolt {
    type Quantity = ElectricPotential;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kV";
}

#[derive(Debug, Clone, Copy)]
struct MilliVolt;
impl MeasurementUnit for MilliVolt {
    type Quantity = ElectricPotential;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mV";
}

struct MicroVolt;
impl MeasurementUnit for MicroVolt {
    type Quantity = ElectricPotential;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}V";
}

// electrical resistance, electrical impedance
struct ElectricalResistance;

struct Ohm;
impl MeasurementUnit for Ohm {
    type Quantity = ElectricalResistance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3A9}";
}

struct MilliOhm;
impl MeasurementUnit for MilliOhm {
    type Quantity = ElectricalResistance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{3A9}";
}

struct KiloOhm;
impl MeasurementUnit for KiloOhm {
    type Quantity = ElectricalResistance;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " k\u{3A9}";
}

// Electrical resistivity
struct ElectricalResistivity;

struct OhmMetre;
impl MeasurementUnit for OhmMetre {
    type Quantity = ElectricalResistivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3A9}\u{b7}m";
}

// Energy, work, heat
struct Energy;

struct Joule;
impl MeasurementUnit for Joule {
    type Quantity = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

struct WattHour;
impl MeasurementUnit for WattHour {
    type Quantity = Energy;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W\u{b7}h";
}

struct KiloWattHour;
impl MeasurementUnit for KiloWattHour {
    type Quantity = Energy;
    const RATIO: f64 = 3.6e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " KW\u{b7}h";
}

struct MegaWattHour;
impl MeasurementUnit for MegaWattHour {
    type Quantity = Energy;
    const RATIO: f64 = 3.6e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW\u{b7}h";
}

struct Calorie;
impl MeasurementUnit for Calorie {
    type Quantity = Energy;
    const RATIO: f64 = 4.187;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cal";
}

struct KiloCalorie;
impl MeasurementUnit for KiloCalorie {
    type Quantity = Energy;
    const RATIO: f64 = 4187.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kcal";
}

// energy density
struct EnergyDensity;

struct JoulePerCubicMetre;
impl MeasurementUnit for JoulePerCubicMetre {
    type Quantity = EnergyDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{B3}";
}

// Entropy, heat capacity
struct Entropy;

struct JoulePerKelvin;
impl MeasurementUnit for JoulePerKelvin {
    type Quantity = Entropy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{B0}K";
}

// Force, weight
struct Force;

struct Newton;
impl MeasurementUnit for Newton {
    type Quantity = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

struct Dyne;
impl MeasurementUnit for Dyne {
    type Quantity = Force;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn";
}

struct KilogramForce;
impl MeasurementUnit for KilogramForce {
    type Quantity = Force;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kgf";
}

struct PoundForce;
impl MeasurementUnit for PoundForce {
    type Quantity = Force;
    const RATIO: f64 = 4.448222;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf";
}

struct Poundal;
impl MeasurementUnit for Poundal {
    type Quantity = Force;
    const RATIO: f64 = 0.138255;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pdl";
}

// Frequency, angular speed, angular velocity
struct Frequency;

struct Hertz;
impl MeasurementUnit for Hertz {
    type Quantity = Frequency;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Hz";
}

struct KiloHertz;
impl MeasurementUnit for KiloHertz {
    type Quantity = Frequency;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kHz";
}

struct MegaHertz;
impl MeasurementUnit for MegaHertz {
    type Quantity = Frequency;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MHz";
}

struct GigaHertz;
impl MeasurementUnit for GigaHertz {
    type Quantity = Frequency;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GHz";
}

struct RadianPerSecond;
impl MeasurementUnit for RadianPerSecond {
    type Quantity = Frequency;
    const RATIO: f64 = 1. / TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s";
}

struct TurnPerMinute;
impl MeasurementUnit for TurnPerMinute {
    type Quantity = Frequency;
    const RATIO: f64 = 1. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rpm";
}

// irradiance, heat flux density
struct Irradiance;

struct WattPerSquareMetre;
impl MeasurementUnit for WattPerSquareMetre {
    type Quantity = Irradiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}";
}

struct Illuminance;

struct Lux;
impl MeasurementUnit for Lux {
    type Quantity = Illuminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lx";
}

struct Phot;
impl MeasurementUnit for Phot {
    type Quantity = Illuminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " phot";
}

struct FootCandle;
impl MeasurementUnit for FootCandle {
    type Quantity = Illuminance;
    const RATIO: f64 = 10.764;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fc";
}
//luxes = candelas * steradians / square_Metres
//luxes = lumens / square_Metres
//foot_candles = lumens / square_feet

// Inductance
struct Inductance;

struct Henry;
impl MeasurementUnit for Henry {
    type Quantity = Inductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H";
}

// Information
struct Information;

struct Bit;
impl MeasurementUnit for Bit {
    type Quantity = Information;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b";
}

struct Byte;
impl MeasurementUnit for Byte {
    type Quantity = Information;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B";
}

struct KiloBit;
impl MeasurementUnit for KiloBit {
    type Quantity = Information;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb";
}

struct KiloByte;
impl MeasurementUnit for KiloByte {
    type Quantity = Information;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB";
}

struct KibiBit;
impl MeasurementUnit for KibiBit {
    type Quantity = Information;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib";
}

struct KibiByte;
impl MeasurementUnit for KibiByte {
    type Quantity = Information;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB";
}

struct MegaBit;
impl MeasurementUnit for MegaBit {
    type Quantity = Information;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb";
}

struct MegaByte;
impl MeasurementUnit for MegaByte {
    type Quantity = Information;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB";
}

struct MebiBit;
impl MeasurementUnit for MebiBit {
    type Quantity = Information;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib";
}

struct MebiByte;
impl MeasurementUnit for MebiByte {
    type Quantity = Information;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB";
}

struct GigaBit;
impl MeasurementUnit for GigaBit {
    type Quantity = Information;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb";
}

struct GigaByte;
impl MeasurementUnit for GigaByte {
    type Quantity = Information;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB";
}

struct GibiBit;
impl MeasurementUnit for GibiBit {
    type Quantity = Information;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib";
}

struct GibiByte;
impl MeasurementUnit for GibiByte {
    type Quantity = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB";
}

struct TeraBit;
impl MeasurementUnit for TeraBit {
    type Quantity = Information;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb";
}

struct TeraByte;
impl MeasurementUnit for TeraByte {
    type Quantity = Information;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB";
}

struct TebiBit;
impl MeasurementUnit for TebiBit {
    type Quantity = Information;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib";
}

struct TebiByte;
impl MeasurementUnit for TebiByte {
    type Quantity = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB";
}

// Information rate
struct InformationRate;

struct BitPerSecond;
impl MeasurementUnit for BitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b/s";
}

struct BytePerSecond;
impl MeasurementUnit for BytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B/s";
}

struct KiloBitPerSecond;
impl MeasurementUnit for KiloBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb/s";
}

struct KiloBytePerSecond;
impl MeasurementUnit for KiloBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB/s";
}

struct KibiBitPerSecond;
impl MeasurementUnit for KibiBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib/s";
}

struct KibiBytePerSecond;
impl MeasurementUnit for KibiBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB/s";
}

struct MegaBitPerSecond;
impl MeasurementUnit for MegaBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb/s";
}

struct MegaBytePerSecond;
impl MeasurementUnit for MegaBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB/s";
}

struct MebiBitPerSecond;
impl MeasurementUnit for MebiBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib/s";
}

struct MebiBytePerSecond;
impl MeasurementUnit for MebiBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB/s";
}

struct GigaBitPerSecond;
impl MeasurementUnit for GigaBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb/s";
}

struct GigaBytePerSecond;
impl MeasurementUnit for GigaBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB/s";
}

struct GibiBitPerSecond;
impl MeasurementUnit for GibiBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib/s";
}

struct GibiBytePerSecond;
impl MeasurementUnit for GibiBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB/s";
}

struct TeraBitPerSecond;
impl MeasurementUnit for TeraBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb/s";
}

struct TeraBytePerSecond;
impl MeasurementUnit for TeraBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB/s";
}

struct TebiBitPerSecond;
impl MeasurementUnit for TebiBitPerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib/s";
}

struct TebiBytePerSecond;
impl MeasurementUnit for TebiBytePerSecond {
    type Quantity = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB/s";
}

// Kinematic viscosity
struct KinematicViscosity;

struct SquareMetrePerSecond;
impl MeasurementUnit for SquareMetrePerSecond {
    type Quantity = KinematicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}/s";
}

struct Stoke;
impl MeasurementUnit for Stoke {
    type Quantity = KinematicViscosity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " St";
}

struct CentiStoke;
impl MeasurementUnit for CentiStoke {
    type Quantity = KinematicViscosity;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cSt";
}

// length, width, height, depth, space, wavelength
struct Length;

#[derive(Debug, Clone, Copy)]
struct Metre;
impl MeasurementUnit for Metre {
    type Quantity = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

struct AstronomicalUnit;
impl MeasurementUnit for AstronomicalUnit {
    type Quantity = Length;
    const RATIO: f64 = 149597870691.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " a.u.";
}

struct Parsec;
impl MeasurementUnit for Parsec {
    type Quantity = Length;
    const RATIO: f64 = 3.0856775813e16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " psc";
}

struct LightYear;
impl MeasurementUnit for LightYear {
    type Quantity = Length;
    const RATIO: f64 = 31557600. * 2.99792458e8;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ly";
}

struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Quantity = Length;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}

struct HectoMetre;
impl MeasurementUnit for HectoMetre {
    type Quantity = Length;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hm";
}

struct DecaMetre;
impl MeasurementUnit for DecaMetre {
    type Quantity = Length;
    const RATIO: f64 = 10.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dam";
}

struct DeciMetre;
impl MeasurementUnit for DeciMetre {
    type Quantity = Length;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm";
}

struct CentiMetre;
impl MeasurementUnit for CentiMetre {
    type Quantity = Length;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm";
}

struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Quantity = Length;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

struct MicroMetre;
impl MeasurementUnit for MicroMetre {
    type Quantity = Length;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}m";
}

struct NanoMetre;
impl MeasurementUnit for NanoMetre {
    type Quantity = Length;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm";
}

struct Angstrom;
impl MeasurementUnit for Angstrom {
    type Quantity = Length;
    const RATIO: f64 = 1e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{212B}";
}

struct Inch;
impl MeasurementUnit for Inch {
    type Quantity = Length;
    const RATIO: f64 = 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in";
}

struct Foot;
impl MeasurementUnit for Foot {
    type Quantity = Length;
    const RATIO: f64 = 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft";
}

struct Yard;
impl MeasurementUnit for Yard {
    type Quantity = Length;
    const RATIO: f64 = 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd";
}

struct Mile;
impl MeasurementUnit for Mile {
    type Quantity = Length;
    const RATIO: f64 = 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi";
}

struct NauticalMile;
impl MeasurementUnit for NauticalMile {
    type Quantity = Length;
    const RATIO: f64 = 1852.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " naut.mi";
}

// Linear density
struct LinearDensity;

struct KilogramPerMetre;
impl MeasurementUnit for KilogramPerMetre {
    type Quantity = LinearDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m";
}

// Luminance
struct Luminance;

struct CandelaPerSquareMetre; // aka "nit"
impl MeasurementUnit for CandelaPerSquareMetre {
    type Quantity = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd/m\u{b2}";
}

struct Stilb;
impl MeasurementUnit for Stilb {
    type Quantity = Luminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " stilb";
}

// Luminous flux, luminous power
struct LuminousFlux;

struct Lumen;
impl MeasurementUnit for Lumen {
    type Quantity = LuminousFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lm";
}

// Luminous intensity
struct LuminousIntensity;

struct Candela;
impl MeasurementUnit for Candela {
    type Quantity = LuminousIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd";
}

// Magnetic field strength, Magnetic field intensity, Magnetization
struct MagneticFieldStrength;

struct AmperePerMetre;
impl MeasurementUnit for AmperePerMetre {
    type Quantity = MagneticFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m";
}

// Magnetic flux
struct MagneticFlux;

struct Weber;
impl MeasurementUnit for Weber {
    type Quantity = MagneticFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Wb";
}

// Magnetic flux density
struct MagneticFluxDensity;

struct Tesla;
impl MeasurementUnit for Tesla {
    type Quantity = MagneticFluxDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " T";
}

struct Gauss;
impl MeasurementUnit for Gauss {
    type Quantity = MagneticFluxDensity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " G";
}

// Magnetic reluctance, Magnetic resistance
struct MagneticReluctance;

struct InverseHenry;
impl MeasurementUnit for InverseHenry {
    type Quantity = MagneticReluctance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/H";
}

// Mass
struct Mass;

struct KiloGram;
impl MeasurementUnit for KiloGram {
    type Quantity = Mass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg";
}

struct MetricTon;
impl MeasurementUnit for MetricTon {
    type Quantity = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

struct HectoGram;
impl MeasurementUnit for HectoGram {
    type Quantity = Mass;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hg";
}

struct DecaGram;
impl MeasurementUnit for DecaGram {
    type Quantity = Mass;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dag";
}

struct Gram;
impl MeasurementUnit for Gram {
    type Quantity = Mass;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

struct MilliGram;
impl MeasurementUnit for MilliGram {
    type Quantity = Mass;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mg";
}

struct MicroGram;
impl MeasurementUnit for MicroGram {
    type Quantity = Mass;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}g";
}

struct NanoGram;
impl MeasurementUnit for NanoGram {
    type Quantity = Mass;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ng";
}

// Mass density
struct MassDensity;

struct KiloGramPerCubicMetre;
impl MeasurementUnit for KiloGramPerCubicMetre {
    type Quantity = MassDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{B3}";
}

// Mass flow rate
struct MassFlowRate;

struct KiloGramPerSecond;
impl MeasurementUnit for KiloGramPerSecond {
    type Quantity = MassFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/s";
}

struct GramPerSecond;
impl MeasurementUnit for GramPerSecond {
    type Quantity = MassFlowRate;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/s";
}

// molar heat capacity, molar entropy
struct MolarHeatCapacity;

struct JoulePerKelvinPerMole;
impl MeasurementUnit for JoulePerKelvinPerMole {
    type Quantity = MolarHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{B0}K/mol";
}

// Moment of inertia, Rotational inertia
struct MomentOfInertia;

struct KiloGramSquareMetre;
impl MeasurementUnit for KiloGramSquareMetre {
    type Quantity = MomentOfInertia;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}";
}

struct GramSquareCentiMetre;
impl MeasurementUnit for GramSquareCentiMetre {
    type Quantity = MomentOfInertia;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}";
}

// Momentum, Impulse
struct Momentum;

struct NewtonSecond;
impl MeasurementUnit for NewtonSecond {
    type Quantity = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}s";
}

struct KiloGramMetrePerSecond;
impl MeasurementUnit for KiloGramMetrePerSecond {
    type Quantity = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m/s";
}

struct DynSecond;
impl MeasurementUnit for DynSecond {
    type Quantity = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn\u{b7}s";
}

struct GramCentiMetrePerSecond;
impl MeasurementUnit for GramCentiMetrePerSecond {
    type Quantity = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm/s";
}

// Magnetic permeability
struct MagneticPermeability;

struct HenryPerMetre;
impl MeasurementUnit for HenryPerMetre {
    type Quantity = MagneticPermeability;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H/m";
}

// Permittivity
struct Permittivity;

struct FaradPerMetre;
impl MeasurementUnit for FaradPerMetre {
    type Quantity = Permittivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F/m";
}

// Power
struct Power;

struct Watt;
impl MeasurementUnit for Watt {
    type Quantity = Power;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

struct MilliWatt;
impl MeasurementUnit for MilliWatt {
    type Quantity = Power;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mW";
}

struct KiloWatt;
impl MeasurementUnit for KiloWatt {
    type Quantity = Power;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kW";
}

struct MegaWatt;
impl MeasurementUnit for MegaWatt {
    type Quantity = Power;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW";
}

struct GigaWatt;
impl MeasurementUnit for GigaWatt {
    type Quantity = Power;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GW";
}

struct HorsePower;
impl MeasurementUnit for HorsePower {
    type Quantity = Power;
    const RATIO: f64 = 745.699872;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " HP";
}

// Pressure, Stress
struct Pressure;

struct Pascal;
impl MeasurementUnit for Pascal {
    type Quantity = Pressure;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa";
}

struct HectoPascal;
impl MeasurementUnit for HectoPascal {
    type Quantity = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hPa";
}

struct PhysicalAtmosphere;
impl MeasurementUnit for PhysicalAtmosphere {
    type Quantity = Pressure;
    const RATIO: f64 = 1.013e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " atm";
}

struct Bar;
impl MeasurementUnit for Bar {
    type Quantity = Pressure;
    const RATIO: f64 = 1e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " bar";
}
struct MilliBar;
impl MeasurementUnit for MilliBar {
    type Quantity = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mbar";
}

struct MmHg;
impl MeasurementUnit for MmHg {
    type Quantity = Pressure;
    const RATIO: f64 = 133.322;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " torr";
}

// Radiance
struct Radiance;

struct WattPerSquareMetrePerSteradian;
impl MeasurementUnit for WattPerSquareMetrePerSteradian {
    type Quantity = Radiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}/sr";
}

// Radiant intensity
struct RadiantIntensity;

struct WattPerSteradian;
impl MeasurementUnit for WattPerSteradian {
    type Quantity = RadiantIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/sr";
}

// Radioactive activity
struct RadioactiveActivity;

struct Becquerel;
impl MeasurementUnit for Becquerel {
    type Quantity = RadioactiveActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Bq";
}
struct KiloBecquerel;
impl MeasurementUnit for KiloBecquerel {
    type Quantity = RadioactiveActivity;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kBq";
}

struct MegaBecquerel;
impl MeasurementUnit for MegaBecquerel {
    type Quantity = RadioactiveActivity;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MBq";
}

struct GigaBecquerel;
impl MeasurementUnit for GigaBecquerel {
    type Quantity = RadioactiveActivity;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GBq";
}

// Radioactive dose
struct RadioactiveDose;

struct Gray;
impl MeasurementUnit for Gray {
    type Quantity = RadioactiveDose;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy";
}

struct Rad;
impl MeasurementUnit for Rad {
    type Quantity = RadioactiveDose;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}

// Radioactive dose rate
struct RadioactiveDoseRate;

struct GrayPerSecond;
impl MeasurementUnit for GrayPerSecond {
    type Quantity = RadioactiveDoseRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy/s";
}

// Reaction rate
struct ReactionRate;

struct MolePerCubicMetrePerSecond;
impl MeasurementUnit for MolePerCubicMetrePerSecond {
    type Quantity = ReactionRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{B3}/s";
}

// Solid angle
struct SolidAngle;

struct Steradian;
impl MeasurementUnit for Steradian {
    type Quantity = SolidAngle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sr";
}

struct AllRound;
impl MeasurementUnit for AllRound {
    type Quantity = SolidAngle;
    const RATIO: f64 = 2. * TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sphere";
}

struct SquareDegree;
impl MeasurementUnit for SquareDegree {
    type Quantity = SolidAngle;
    const RATIO: f64 = TAU * TAU / 360. / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\u{b2}";
}

// Specific energy
struct SpecificEnergy;

struct JoulePerKiloGram;
impl MeasurementUnit for JoulePerKiloGram {
    type Quantity = SpecificEnergy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg";
}

// Specific heat capacity
struct SpecificHeatCapacity;

struct JoulePerKiloGramPerKelvin;
impl MeasurementUnit for JoulePerKiloGramPerKelvin {
    type Quantity = SpecificHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg/\u{B0}K";
}

// Specific volume
struct SpecificVolume;

struct CubicMetrePerKiloGram;
impl MeasurementUnit for CubicMetrePerKiloGram {
    type Quantity = SpecificVolume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{B3}/kg";
}

// Square time
struct SquareTime;

struct SquareSecond;
impl MeasurementUnit for SquareSecond {
    type Quantity = SquareTime;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s\u{b2}";
}

struct HourSecond;
impl MeasurementUnit for HourSecond {
    type Quantity = SquareTime;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}s";
}

struct HourHour;
impl MeasurementUnit for HourHour {
    type Quantity = SquareTime;
    const RATIO: f64 = 3600. * 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}h";
}

// Surface density
struct SurfaceDensity;

struct KiloGramPerSquareMetre;
impl MeasurementUnit for KiloGramPerSquareMetre {
    type Quantity = SurfaceDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{b2}";
}

// Surface tension
struct SurfaceTension;

struct JoulePerSquareMetre;
impl MeasurementUnit for JoulePerSquareMetre {
    type Quantity = SurfaceTension;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{b2}";
}

// Temperature
struct Temperature;

struct Kelvin;
impl MeasurementUnit for Kelvin {
    type Quantity = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B0}K";
}

struct Celsius;
impl MeasurementUnit for Celsius {
    type Quantity = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 273.15;
    const SUFFIX: &'static str = " \u{B0}C";
}

struct Fahrenheit;
impl MeasurementUnit for Fahrenheit {
    type Quantity = Temperature;
    const RATIO: f64 = 5. / 9.;
    const OFFSET: f64 = 273.15 - 32. * 5. / 9.;
    const SUFFIX: &'static str = " \u{B0}F";
}

// Thermal conductivity
struct ThermalConductivity;

struct WattPerMetrePerKelvin;
impl MeasurementUnit for WattPerMetrePerKelvin {
    type Quantity = ThermalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m/\u{B0}K";
}

// Time, mean lifetime
struct Time;

struct Second;
impl MeasurementUnit for Second {
    type Quantity = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

struct Year;
impl MeasurementUnit for Year {
    type Quantity = Time;
    const RATIO: f64 = 365.25 * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Y";
}

struct Month;
impl MeasurementUnit for Month {
    type Quantity = Time;
    const RATIO: f64 = 30. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " M";
}

struct Week;
impl MeasurementUnit for Week {
    type Quantity = Time;
    const RATIO: f64 = 7. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

struct Day;
impl MeasurementUnit for Day {
    type Quantity = Time;
    const RATIO: f64 = 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " D";
}

struct Hour;
impl MeasurementUnit for Hour {
    type Quantity = Time;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h";
}

struct Minute;
impl MeasurementUnit for Minute {
    type Quantity = Time;
    const RATIO: f64 = 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " min";
}

struct MilliSecond;
impl MeasurementUnit for MilliSecond {
    type Quantity = Time;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ms";
}

struct MicroSecond;
impl MeasurementUnit for MicroSecond {
    type Quantity = Time;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}s";
}

struct NanoSecond;
impl MeasurementUnit for NanoSecond {
    type Quantity = Time;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ns";
}

struct PicoSecond;
impl MeasurementUnit for PicoSecond {
    type Quantity = Time;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ps";
}

struct FemtoSecond;
impl MeasurementUnit for FemtoSecond {
    type Quantity = Time;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fs";
}

// Torque
struct Torque;

struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Quantity = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}

// Velocity, speed
struct Velocity;

struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Quantity = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

struct Knot;
impl MeasurementUnit for Knot {
    type Quantity = Velocity;
    const RATIO: f64 = 1852. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kt";
}

struct KiloMetrePerHour;
impl MeasurementUnit for KiloMetrePerHour {
    type Quantity = Velocity;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h";
}

struct MilePerHour;
impl MeasurementUnit for MilePerHour {
    type Quantity = Velocity;
    const RATIO: f64 = 1609. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi/h";
}

struct Mach;
impl MeasurementUnit for Mach {
    type Quantity = Velocity;
    const RATIO: f64 = 340.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mach";
}

struct CentiMetresPerSecond;
impl MeasurementUnit for CentiMetresPerSecond {
    type Quantity = Velocity;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s";
}

// Volume
struct Volume;

struct CubicMetre;
impl MeasurementUnit for CubicMetre {
    type Quantity = Volume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{B3}";
}

struct CubicKiloMetre;
impl MeasurementUnit for CubicKiloMetre {
    type Quantity = Volume;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{B3}";
}

struct CubicInch;
impl MeasurementUnit for CubicInch {
    type Quantity = Volume;
    const RATIO: f64 = 0.0254 * 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{B3}";
}

struct CubicFoot;
impl MeasurementUnit for CubicFoot {
    type Quantity = Volume;
    const RATIO: f64 = 0.3048 * 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{B3}";
}

struct CubicYard;
impl MeasurementUnit for CubicYard {
    type Quantity = Volume;
    const RATIO: f64 = 0.9144 * 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{B3}";
}

struct CubicMile;
impl MeasurementUnit for CubicMile {
    type Quantity = Volume;
    const RATIO: f64 = 1609. * 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{B3}";
}

struct Litre; // a.k.a. cubic decimetre or dm\u{B3}
impl MeasurementUnit for Litre {
    type Quantity = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " l";
}

struct MilliLitre; // a.k.a. cubic centimetre or cm\u{B3}
impl MeasurementUnit for MilliLitre {
    type Quantity = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml";
}

struct MicroLitre; // a.k.a. cubic millimetre or mm\u{B3}
impl MeasurementUnit for MicroLitre {
    type Quantity = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{B5}l";
}

struct NanoLitre;
impl MeasurementUnit for NanoLitre {
    type Quantity = Volume;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nl";
}

struct PicoLitre;
impl MeasurementUnit for PicoLitre {
    type Quantity = Volume;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pl";
}

struct Pint;
impl MeasurementUnit for Pint {
    type Quantity = Volume;
    const RATIO: f64 = 473.2e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pt";
}

struct Gallon;
impl MeasurementUnit for Gallon {
    type Quantity = Volume;
    const RATIO: f64 = 4546e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " gal";
}

// Volumetric flow rate
struct VolumetricFlowRate;

struct CubicMetrePerSecond;
impl MeasurementUnit for CubicMetrePerSecond {
    type Quantity = VolumetricFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{B3}/s";
}

struct CubicCentiMetrePerSecond;
impl MeasurementUnit for CubicCentiMetrePerSecond {
    type Quantity = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{B3}/s";
}

// Wave number
struct WaveNumber;

struct CyclePerMetre;
impl MeasurementUnit for CyclePerMetre {
    type Quantity = WaveNumber;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/m";
}

struct RadianPerMetre;
impl MeasurementUnit for RadianPerMetre {
    type Quantity = WaveNumber;
    const RATIO: f64 = 1. / TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/m";
}

////

fn print_them_all() {
    println!("Turn: {}.", Measure::<f64, Turn>::new(1.2));
    println!("Gradian: {}.", Measure::<f64, Gradian>::new(1.2));
    println!("Degree: {}.", Measure::<f64, Degree>::new(1.2));
    println!("ArcMinute: {}.", Measure::<f64, ArcMinute>::new(1.2));
    println!("ArcSecond: {}.", Measure::<f64, ArcSecond>::new(1.2));
    println!(
        "MetrePerSquareSecond: {}.",
        Measure::<f64, MetrePerSquareSecond>::new(1.2)
    );
    println!("GForce: {}.", Measure::<f64, GForce>::new(1.2));
    println!(
        "KiloMetrePerHourPerSecond: {}.",
        Measure::<f64, KiloMetrePerHourPerSecond>::new(1.2)
    );
    println!("JouleSecond: {}.", Measure::<f64, JouleSecond>::new(1.2));
    println!("Unit: {}.", Measure::<f64, Unit>::new(1.2));
    println!("Dozen: {}.", Measure::<f64, Dozen>::new(1.2));
    println!("Mole: {}.", Measure::<f64, Mole>::new(1.2));
    println!(
        "RadianPerSquareSecond: {}.",
        Measure::<f64, RadianPerSquareSecond>::new(1.2)
    );
    println!(
        "KilogramSquareMetrePerSecond: {}.",
        Measure::<f64, KilogramSquareMetrePerSecond>::new(1.2)
    );
    println!(
        "GramSquareCentiMetrePerSecond: {}.",
        Measure::<f64, GramSquareCentiMetrePerSecond>::new(1.2)
    );
    println!("SquareMetre: {}.", Measure::<f64, SquareMetre>::new(1.2));
    println!(
        "SquareKiloMetre: {}.",
        Measure::<f64, SquareKiloMetre>::new(1.2)
    );
    println!("Hectare: {}.", Measure::<f64, Hectare>::new(1.2));
    println!("Are: {}.", Measure::<f64, Are>::new(1.2));
    println!(
        "SquareDeciMetre: {}.",
        Measure::<f64, SquareDeciMetre>::new(1.2)
    );
    println!(
        "SquareCentiMetre: {}.",
        Measure::<f64, SquareCentiMetre>::new(1.2)
    );
    println!(
        "SquareMilliMetre: {}.",
        Measure::<f64, SquareMilliMetre>::new(1.2)
    );
    println!("SquareInch: {}.", Measure::<f64, SquareInch>::new(1.2));
    println!("SquareFoot: {}.", Measure::<f64, SquareFoot>::new(1.2));
    println!("SquareYard: {}.", Measure::<f64, SquareYard>::new(1.2));
    println!("SquareMile: {}.", Measure::<f64, SquareMile>::new(1.2));
    println!(
        "KilogramPerSquareMetre: {}.",
        Measure::<f64, KilogramPerSquareMetre>::new(1.2)
    );
    println!("Farad: {}.", Measure::<f64, Farad>::new(1.2));
    println!("MilliFarad: {}.", Measure::<f64, MilliFarad>::new(1.2));
    println!("MicroFarad: {}.", Measure::<f64, MicroFarad>::new(1.2));
    println!("NanoFarad: {}.", Measure::<f64, NanoFarad>::new(1.2));
    println!("PicoFarad: {}.", Measure::<f64, PicoFarad>::new(1.2));
    println!("Katal: {}.", Measure::<f64, Katal>::new(1.2));
    println!(
        "KatalPerCubicMetre: {}.",
        Measure::<f64, KatalPerCubicMetre>::new(1.2)
    );
    println!("JoulePerMole: {}.", Measure::<f64, JoulePerMole>::new(1.2));
    println!(
        "MolePerCubicMetre: {}.",
        Measure::<f64, MolePerCubicMetre>::new(1.2)
    );
    println!(
        "AmperePerSquareMetre: {}.",
        Measure::<f64, AmperePerSquareMetre>::new(1.2)
    );
    println!("Sievert: {}.", Measure::<f64, Sievert>::new(1.2));
    println!("Rem: {}.", Measure::<f64, Rem>::new(1.2));
    println!("PascalSecond: {}.", Measure::<f64, PascalSecond>::new(1.2));
    println!("Coulomb: {}.", Measure::<f64, Coulomb>::new(1.2));
    println!("MilliCoulomb: {}.", Measure::<f64, MilliCoulomb>::new(1.2));
    println!("MicroCoulomb: {}.", Measure::<f64, MicroCoulomb>::new(1.2));
    println!("NanoCoulomb: {}.", Measure::<f64, NanoCoulomb>::new(1.2));
    println!("PicoCoulomb: {}.", Measure::<f64, PicoCoulomb>::new(1.2));
    println!(
        "CoulombPerMetre: {}.",
        Measure::<f64, CoulombPerMetre>::new(1.2)
    );
    println!(
        "CoulombPerSquareMetre: {}.",
        Measure::<f64, CoulombPerSquareMetre>::new(1.2)
    );
    println!(
        "CoulombPerCubicMetre: {}.",
        Measure::<f64, CoulombPerCubicMetre>::new(1.2)
    );
    println!("VoltMetre: {}.", Measure::<f64, VoltMetre>::new(1.2));
    println!("Siemens: {}.", Measure::<f64, Siemens>::new(1.2));
    println!(
        "SiemensPerMetre: {}.",
        Measure::<f64, SiemensPerMetre>::new(1.2)
    );
    println!("Ampere: {}.", Measure::<f64, Ampere>::new(1.2));
    println!("MilliAmpere: {}.", Measure::<f64, MilliAmpere>::new(1.2));
    println!("MicroAmpere: {}.", Measure::<f64, MicroAmpere>::new(1.2));
    println!("Volt: {}.", Measure::<f64, Volt>::new(1.2));
    println!("KiloVolt: {}.", Measure::<f64, KiloVolt>::new(1.2));
    println!("MilliVolt: {}.", Measure::<f64, MilliVolt>::new(1.2));
    println!("MicroVolt: {}.", Measure::<f64, MicroVolt>::new(1.2));
    println!("Ohm: {}.", Measure::<f64, Ohm>::new(1.2));
    println!("KiloOhm: {}.", Measure::<f64, KiloOhm>::new(1.2));
    println!("OhmMetre: {}.", Measure::<f64, OhmMetre>::new(1.2));
    println!("Joule: {}.", Measure::<f64, Joule>::new(1.2));
    println!("WattHour: {}.", Measure::<f64, WattHour>::new(1.2));
    println!("KiloWattHour: {}.", Measure::<f64, KiloWattHour>::new(1.2));
    println!("MegaWattHour: {}.", Measure::<f64, MegaWattHour>::new(1.2));
    println!("Calorie: {}.", Measure::<f64, Calorie>::new(1.2));
    println!("KiloCalorie: {}.", Measure::<f64, KiloCalorie>::new(1.2));
    println!(
        "JoulePerCubicMetre: {}.",
        Measure::<f64, JoulePerCubicMetre>::new(1.2)
    );
    println!(
        "JoulePerKelvin: {}.",
        Measure::<f64, JoulePerKelvin>::new(1.2)
    );
    println!("Newton: {}.", Measure::<f64, Newton>::new(1.2));
    println!("Dyne: {}.", Measure::<f64, Dyne>::new(1.2));
    println!(
        "KilogramForce: {}.",
        Measure::<f64, KilogramForce>::new(1.2)
    );
    println!("PoundForce: {}.", Measure::<f64, PoundForce>::new(1.2));
    println!("Poundal: {}.", Measure::<f64, Poundal>::new(1.2));
    println!("Hertz: {}.", Measure::<f64, Hertz>::new(1.2));
    println!("KiloHertz: {}.", Measure::<f64, KiloHertz>::new(1.2));
    println!("MegaHertz: {}.", Measure::<f64, MegaHertz>::new(1.2));
    println!("GigaHertz: {}.", Measure::<f64, GigaHertz>::new(1.2));
    println!(
        "RadianPerSecond: {}.",
        Measure::<f64, RadianPerSecond>::new(1.2)
    );
    println!(
        "TurnPerMinute: {}.",
        Measure::<f64, TurnPerMinute>::new(1.2)
    );
    println!(
        "WattPerSquareMetre: {}.",
        Measure::<f64, WattPerSquareMetre>::new(1.2)
    );
    println!("Lux: {}.", Measure::<f64, Lux>::new(1.2));
    println!("Phot: {}.", Measure::<f64, Phot>::new(1.2));
    println!("FootCandle: {}.", Measure::<f64, FootCandle>::new(1.2));
    println!("Henry: {}.", Measure::<f64, Henry>::new(1.2));
    println!("Bit: {}.", Measure::<f64, Bit>::new(1.2));
    println!("Byte: {}.", Measure::<f64, Byte>::new(1.2));
    println!("KiloBit: {}.", Measure::<f64, KiloBit>::new(1.2));
    println!("KiloByte: {}.", Measure::<f64, KiloByte>::new(1.2));
    println!("KibiBit: {}.", Measure::<f64, KibiBit>::new(1.2));
    println!("KibiByte: {}.", Measure::<f64, KibiByte>::new(1.2));
    println!("MegaBit: {}.", Measure::<f64, MegaBit>::new(1.2));
    println!("MegaByte: {}.", Measure::<f64, MegaByte>::new(1.2));
    println!("MebiBit: {}.", Measure::<f64, MebiBit>::new(1.2));
    println!("MebiByte: {}.", Measure::<f64, MebiByte>::new(1.2));
    println!("GigaBit: {}.", Measure::<f64, GigaBit>::new(1.2));
    println!("GigaByte: {}.", Measure::<f64, GigaByte>::new(1.2));
    println!("GibiBit: {}.", Measure::<f64, GibiBit>::new(1.2));
    println!("GibiByte: {}.", Measure::<f64, GibiByte>::new(1.2));
    println!("TeraBit: {}.", Measure::<f64, TeraBit>::new(1.2));
    println!("TeraByte: {}.", Measure::<f64, TeraByte>::new(1.2));
    println!("TebiBit: {}.", Measure::<f64, TebiBit>::new(1.2));
    println!("TebiByte: {}.", Measure::<f64, TebiByte>::new(1.2));
    println!("BitPerSecond: {}.", Measure::<f64, BitPerSecond>::new(1.2));
    println!(
        "BytePerSecond: {}.",
        Measure::<f64, BytePerSecond>::new(1.2)
    );
    println!(
        "KiloBitPerSecond: {}.",
        Measure::<f64, KiloBitPerSecond>::new(1.2)
    );
    println!(
        "KiloBytePerSecond: {}.",
        Measure::<f64, KiloBytePerSecond>::new(1.2)
    );
    println!(
        "KibiBitPerSecond: {}.",
        Measure::<f64, KibiBitPerSecond>::new(1.2)
    );
    println!(
        "KibiBytePerSecond: {}.",
        Measure::<f64, KibiBytePerSecond>::new(1.2)
    );
    println!(
        "MegaBitPerSecond: {}.",
        Measure::<f64, MegaBitPerSecond>::new(1.2)
    );
    println!(
        "MegaBytePerSecond: {}.",
        Measure::<f64, MegaBytePerSecond>::new(1.2)
    );
    println!(
        "MebiBitPerSecond: {}.",
        Measure::<f64, MebiBitPerSecond>::new(1.2)
    );
    println!(
        "MebiBytePerSecond: {}.",
        Measure::<f64, MebiBytePerSecond>::new(1.2)
    );
    println!(
        "GigaBitPerSecond: {}.",
        Measure::<f64, GigaBitPerSecond>::new(1.2)
    );
    println!(
        "GigaBytePerSecond: {}.",
        Measure::<f64, GigaBytePerSecond>::new(1.2)
    );
    println!(
        "GibiBitPerSecond: {}.",
        Measure::<f64, GibiBitPerSecond>::new(1.2)
    );
    println!(
        "GibiBytePerSecond: {}.",
        Measure::<f64, GibiBytePerSecond>::new(1.2)
    );
    println!(
        "TeraBitPerSecond: {}.",
        Measure::<f64, TeraBitPerSecond>::new(1.2)
    );
    println!(
        "TeraBytePerSecond: {}.",
        Measure::<f64, TeraBytePerSecond>::new(1.2)
    );
    println!(
        "TebiBitPerSecond: {}.",
        Measure::<f64, TebiBitPerSecond>::new(1.2)
    );
    println!(
        "TebiBytePerSecond: {}.",
        Measure::<f64, TebiBytePerSecond>::new(1.2)
    );
    println!(
        "SquareMetrePerSecond: {}.",
        Measure::<f64, SquareMetrePerSecond>::new(1.2)
    );
    println!("Stoke: {}.", Measure::<f64, Stoke>::new(1.2));
    println!("CentiStoke: {}.", Measure::<f64, CentiStoke>::new(1.2));
    println!("Metre: {}.", Measure::<f64, Metre>::new(1.2));
    println!(
        "AstronomicalUnit: {}.",
        Measure::<f64, AstronomicalUnit>::new(1.2)
    );
    println!("Parsec: {}.", Measure::<f64, Parsec>::new(1.2));
    println!("LightYear: {}.", Measure::<f64, LightYear>::new(1.2));
    println!("KiloMetre: {}.", Measure::<f64, KiloMetre>::new(1.2));
    println!("HectoMetre: {}.", Measure::<f64, HectoMetre>::new(1.2));
    println!("DecaMetre: {}.", Measure::<f64, DecaMetre>::new(1.2));
    println!("DeciMetre: {}.", Measure::<f64, DeciMetre>::new(1.2));
    println!("CentiMetre: {}.", Measure::<f64, CentiMetre>::new(1.2));
    println!("MilliMetre: {}.", Measure::<f64, MilliMetre>::new(1.2));
    println!("MicroMetre: {}.", Measure::<f64, MicroMetre>::new(1.2));
    println!("NanoMetre: {}.", Measure::<f64, NanoMetre>::new(1.2));
    println!("Angstrom: {}.", Measure::<f64, Angstrom>::new(1.2));
    println!("Inch: {}.", Measure::<f64, Inch>::new(1.2));
    println!("Foot: {}.", Measure::<f64, Foot>::new(1.2));
    println!("Yard: {}.", Measure::<f64, Yard>::new(1.2));
    println!("Mile: {}.", Measure::<f64, Mile>::new(1.2));
    println!("NauticalMile: {}.", Measure::<f64, NauticalMile>::new(1.2));
    println!(
        "KilogramPerMetre: {}.",
        Measure::<f64, KilogramPerMetre>::new(1.2)
    );
    println!(
        "CandelaPerSquareMetre: {}.",
        Measure::<f64, CandelaPerSquareMetre>::new(1.2)
    );
    println!("Stilb: {}.", Measure::<f64, Stilb>::new(1.2));
    println!("Lumen: {}.", Measure::<f64, Lumen>::new(1.2));
    println!("Candela: {}.", Measure::<f64, Candela>::new(1.2));
    println!(
        "AmperePerMetre: {}.",
        Measure::<f64, AmperePerMetre>::new(1.2)
    );
    println!("Weber: {}.", Measure::<f64, Weber>::new(1.2));
    println!("Tesla: {}.", Measure::<f64, Tesla>::new(1.2));
    println!("Gauss: {}.", Measure::<f64, Gauss>::new(1.2));
    println!("InverseHenry: {}.", Measure::<f64, InverseHenry>::new(1.2));
    println!("KiloGram: {}.", Measure::<f64, KiloGram>::new(1.2));
    /*
    println!(": {}.", Measure::<f64>::new(1.2)) MetricTon;
    println!(": {}.", Measure::<f64>::new(1.2)) HectoGram;
    println!(": {}.", Measure::<f64>::new(1.2)) DecaGram;
    println!(": {}.", Measure::<f64>::new(1.2)) Gram;
    println!(": {}.", Measure::<f64>::new(1.2)) MilliGram;
    println!(": {}.", Measure::<f64>::new(1.2)) MicroGram;
    println!(": {}.", Measure::<f64>::new(1.2)) NanoGram;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloGramPerCubicMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloGramPerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) GramPerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) JoulePerKelvinPerMole;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloGramSquareMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) GramSquareCentiMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) NewtonSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloGramMetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) DynSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) GramCentiMetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) HenryPerMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) FaradPerMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) Watt;
    println!(": {}.", Measure::<f64>::new(1.2)) MilliWatt;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloWatt;
    println!(": {}.", Measure::<f64>::new(1.2)) MegaWatt;
    println!(": {}.", Measure::<f64>::new(1.2)) GigaWatt;
    println!(": {}.", Measure::<f64>::new(1.2)) HorsePower;
    println!(": {}.", Measure::<f64>::new(1.2)) Pascal;
    println!(": {}.", Measure::<f64>::new(1.2)) HectoPascal;
    println!(": {}.", Measure::<f64>::new(1.2)) PhysicalAtmosphere;
    println!(": {}.", Measure::<f64>::new(1.2)) Bar;
    println!(": {}.", Measure::<f64>::new(1.2)) MilliBar;
    println!(": {}.", Measure::<f64>::new(1.2)) MmHg;
    println!(": {}.", Measure::<f64>::new(1.2)) WattPerSquareMetrePerSteradian;
    println!(": {}.", Measure::<f64>::new(1.2)) WattPerSteradian;
    println!(": {}.", Measure::<f64>::new(1.2)) Becquerel;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloBecquerel;
    println!(": {}.", Measure::<f64>::new(1.2)) MegaBecquerel;
    println!(": {}.", Measure::<f64>::new(1.2)) GigaBecquerel;
    println!(": {}.", Measure::<f64>::new(1.2)) Gray;
    println!(": {}.", Measure::<f64>::new(1.2)) Rad;
    println!(": {}.", Measure::<f64>::new(1.2)) GrayPerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) MolePerCubicMetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) Steradian;
    println!(": {}.", Measure::<f64>::new(1.2)) AllRound;
    println!(": {}.", Measure::<f64>::new(1.2)) SquareDegree;
    println!(": {}.", Measure::<f64>::new(1.2)) JoulePerKiloGram;
    println!(": {}.", Measure::<f64>::new(1.2)) JoulePerKiloGramPerKelvin;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicMetrePerKiloGram;
    println!(": {}.", Measure::<f64>::new(1.2)) SquareSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) HourSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) HourHour;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloGramPerSquareMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) JoulePerSquareMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) Kelvin;
    println!(": {}.", Measure::<f64>::new(1.2)) Celsius;
    println!(": {}.", Measure::<f64>::new(1.2)) Fahrenheit;
    println!(": {}.", Measure::<f64>::new(1.2)) WattPerMetrePerKelvin;
    println!(": {}.", Measure::<f64>::new(1.2)) Second;
    println!(": {}.", Measure::<f64>::new(1.2)) Year;
    println!(": {}.", Measure::<f64>::new(1.2)) Month;
    println!(": {}.", Measure::<f64>::new(1.2)) Week;
    println!(": {}.", Measure::<f64>::new(1.2)) Day;
    println!(": {}.", Measure::<f64>::new(1.2)) Hour;
    println!(": {}.", Measure::<f64>::new(1.2)) Minute;
    println!(": {}.", Measure::<f64>::new(1.2)) MilliSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) MicroSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) NanoSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) PicoSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) FemtoSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) NewtonMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) MetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) Knot;
    println!(": {}.", Measure::<f64>::new(1.2)) KiloMetrePerHour;
    println!(": {}.", Measure::<f64>::new(1.2)) MilePerHour;
    println!(": {}.", Measure::<f64>::new(1.2)) Mach;
    println!(": {}.", Measure::<f64>::new(1.2)) CentiMetresPerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicKiloMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicInch;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicFoot;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicYard;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicMile;
    println!(": {}.", Measure::<f64>::new(1.2)) Litre;
    println!(": {}.", Measure::<f64>::new(1.2)) MilliLitre;
    println!(": {}.", Measure::<f64>::new(1.2)) MicroLitre;
    println!(": {}.", Measure::<f64>::new(1.2)) NanoLitre;
    println!(": {}.", Measure::<f64>::new(1.2)) PicoLitre;
    println!(": {}.", Measure::<f64>::new(1.2)) Pint;
    println!(": {}.", Measure::<f64>::new(1.2)) Gallon;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicMetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) CubicCentiMetrePerSecond;
    println!(": {}.", Measure::<f64>::new(1.2)) CyclePerMetre;
    println!(": {}.", Measure::<f64>::new(1.2)) RadianPerMetre;
    ........................................................................................
    */
}

//## Relationships among units

//// computer science ////

// Time * InformationRate = Information

define_derived_measure_1_1! {Second, BitPerSecond, Bit}
define_derived_measure_1_1! {Second, BytePerSecond, Byte}
define_derived_measure_1_1! {Second, KiloBitPerSecond, KiloBit}
define_derived_measure_1_1! {Second, KiloBytePerSecond, KiloByte}
define_derived_measure_1_1! {Second, KibiBitPerSecond, KibiBit}
define_derived_measure_1_1! {Second, KibiBytePerSecond, KibiByte}
define_derived_measure_1_1! {Second, MegaBitPerSecond, MegaBit}
define_derived_measure_1_1! {Second, MegaBytePerSecond, MegaByte}
define_derived_measure_1_1! {Second, MebiBitPerSecond, MebiBit}
define_derived_measure_1_1! {Second, MebiBytePerSecond, MebiByte}
define_derived_measure_1_1! {Second, GigaBitPerSecond, GigaBit}
define_derived_measure_1_1! {Second, GigaBytePerSecond, GigaByte}
define_derived_measure_1_1! {Second, GibiBitPerSecond, GibiBit}
define_derived_measure_1_1! {Second, GibiBytePerSecond, GibiByte}
define_derived_measure_1_1! {Second, TeraBitPerSecond, TeraBit}
define_derived_measure_1_1! {Second, TeraBytePerSecond, TeraByte}
define_derived_measure_1_1! {Second, TebiBitPerSecond, TebiBit}
define_derived_measure_1_1! {Second, TebiBytePerSecond, TebiByte}

//// geometry ////

// sqr(Length) = Area
define_derived_measure_squared_1! {Metre, SquareMetre}
define_derived_measure_squared_1! {KiloMetre, SquareKiloMetre}
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

//// kinematics ////

// Time * Velocity = Length
define_derived_measure_1_1! {Second, MetrePerSecond, Metre}
define_derived_measure_1_2! {Second, MetrePerSecond, Metre}
define_derived_measure_1_3! {Second, MetrePerSecond, Metre}
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

//// mechanics ////

// Length * LinearDensity = Mass
define_derived_measure_1_1! {Metre, KilogramPerMetre, KiloGram}

// Area * AreaDensity = Mass
define_derived_measure_1_1! {SquareMetre, KilogramPerSquareMetre, KiloGram}

// Volume * MassDensity = Mass
define_derived_measure_1_1! {CubicMetre, KiloGramPerCubicMetre, KiloGram}

// Mass * SpecificVolume = Volume
define_derived_measure_1_1! {KiloGram, CubicMetrePerKiloGram, CubicMetre}

// Force * Length = Energy, Torque
define_derived_measure_1_1! {Newton, Metre, Joule}
define_derived_measure_2_2! {Newton, Metre, Joule, NewtonMetre}
define_derived_measure_3_3! {Newton, Metre, Joule, NewtonMetre}

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

//// termodynamics ////

// Temperature * Entropy = Energy
define_derived_measure_1_1! {Kelvin, JoulePerKelvin, Joule}

//// optics ////

// LuminousIntensity * SolidAngle = LuminousFlux
define_derived_measure_1_1! {Candela, Steradian, Lumen}

// Area * Illuminance = LuminousFlux
define_derived_measure_1_1! {SquareMetre, Lux, Lumen}
define_derived_measure_1_1! {SquareCentiMetre, Phot, Lumen}
define_derived_measure_1_1! {SquareFoot, FootCandle, Lumen}

// Area * Irradiance = Power
define_derived_measure_1_1! {SquareMetre, WattPerSquareMetre, Watt}

//// electromagnetism ////

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

// Capacitance * EletricPotential = ElectricCharge
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
define_derived_measure_1_1! {Volt, Metre, VoltMetre}
define_derived_measure_1_2! {Volt, Metre, VoltMetre}
define_derived_measure_1_3! {Volt, Metre, VoltMetre}

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

//// chemistry ////

// Time * CatalyticActivity = Amount
define_derived_measure_1_1! {Second, Katal, Mole}

// Volume * CatalyticActivityConcentration = CatalyticActivity
define_derived_measure_1_1! {CubicMetre, KatalPerCubicMetre, Katal}

// Amount * ChemicalPotential = Energy
define_derived_measure_1_1! {Mole, JoulePerMole, Joule}

// Volume * MolarConcentration = Amount
define_derived_measure_1_1! {CubicMetre, MolePerCubicMetre, Mole}

//// radioactivity ////

// Mass * DoseEquivalent = Energy
define_derived_measure_1_1! {KiloGram, Sievert, Joule}

// SquareTime * DoseEquivalent = Area
define_derived_measure_1_1! {SquareSecond, Sievert, SquareMetre}

// Time * RadioactiveDoseRate = RadioactiveDose
define_derived_measure_1_1! {Second, GrayPerSecond, Gray}

//// Others ////

define_derived_measure_squared_2! {Metre, Metre, Metre}
define_derived_measure_squared_3! {Metre, Metre, Metre}

fn main() {
    print_them_all();
    /*
    let move1 = Measure2d::<f64, Metre>::new(1., 2.);
    move1.clone();
    let move2 = Measure2d::<f64, Metre>::new(3., 4.);
    let move3 = move1.squared();
    let move4 = move1 * move1;
    let move5 = move1 * move2;
    */
    /*
    let length1 = Measure::<f64, Metre>::new(20.);
    let length2 = Measure::<f64, Metre>::new(30.);
    let length3: Measure<f64, Metre> = length1 + length2;
    let time = Measure::<f64, Second>::new(4.);
    let speed: Measure<f64, MetrePerSecond> = length3 / time;
    println!(
        "{} plus {} is {}. They are run in {}, at an average speed of {}.",
        length1, length2, length3, time, speed
    );
    */
}
