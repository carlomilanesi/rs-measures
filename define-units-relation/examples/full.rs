// To run this, type:
// cargo run --example full

mod units;
use rs_measures::{angle::Radian, traits::CrossProduct};
use units::*;

fn print_all_acceleration_units() {
    println!("* Acceleration units");
    println!(
        "  MetrePerSquareSecond: {}, {}, {}, {}, {}, {};",
        Measure::<MetrePerSquareSecond>::new(1.),
        MeasurePoint::<MetrePerSquareSecond>::new(1.),
        Measure2d::<MetrePerSquareSecond>::new(1., 2.),
        MeasurePoint2d::<MetrePerSquareSecond>::new(1., 2.),
        Measure3d::<MetrePerSquareSecond>::new(1., 2., 3.),
        MeasurePoint3d::<MetrePerSquareSecond>::new(1., 2., 3.),
    );
    println!(
        "  GForce: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {};",
        Measure::<GForce>::new(1.),
        Measure::<GForce>::new(1.).convert::<MetrePerSquareSecond>(),
        MeasurePoint::<GForce>::new(1.),
        MeasurePoint::<GForce>::new(1.).convert::<MetrePerSquareSecond>(),
        Measure2d::<GForce>::new(1., 2.),
        Measure2d::<GForce>::new(1., 2.).convert::<MetrePerSquareSecond>(),
        MeasurePoint2d::<GForce>::new(1., 2.),
        MeasurePoint2d::<GForce>::new(1., 2.).convert::<MetrePerSquareSecond>(),
        Measure3d::<GForce>::new(1., 2., 3.),
        Measure3d::<GForce>::new(1., 2., 3.).convert::<MetrePerSquareSecond>(),
        MeasurePoint3d::<GForce>::new(1., 2., 3.),
        MeasurePoint3d::<GForce>::new(1., 2., 3.).convert::<MetrePerSquareSecond>(),
    );
    println!(
        "  KiloMetrePerHourPerSecond: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {};",
        Measure::<KiloMetrePerHourPerSecond>::new(1.),
        Measure::<KiloMetrePerHourPerSecond>::new(1.).convert::<MetrePerSquareSecond>(),
        MeasurePoint::<KiloMetrePerHourPerSecond>::new(1.),
        MeasurePoint::<KiloMetrePerHourPerSecond>::new(1.).convert::<MetrePerSquareSecond>(),
        Measure2d::<KiloMetrePerHourPerSecond>::new(1., 2.),
        Measure2d::<KiloMetrePerHourPerSecond>::new(1., 2.).convert::<MetrePerSquareSecond>(),
        MeasurePoint2d::<KiloMetrePerHourPerSecond>::new(1., 2.),
        MeasurePoint2d::<KiloMetrePerHourPerSecond>::new(1., 2.).convert::<MetrePerSquareSecond>(),
        Measure3d::<KiloMetrePerHourPerSecond>::new(1., 2., 3.),
        Measure3d::<KiloMetrePerHourPerSecond>::new(1., 2., 3.).convert::<MetrePerSquareSecond>(),
        MeasurePoint3d::<KiloMetrePerHourPerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<KiloMetrePerHourPerSecond>::new(1., 2., 3.)
            .convert::<MetrePerSquareSecond>(),
    );
    println!();
}

fn print_all_action_units() {
    println!("* Action units");
    println!(
        "  JouleSecond: {}, {};",
        Measure::<JouleSecond>::new(1.),
        MeasurePoint::<JouleSecond>::new(1.),
    );
    println!();
}

fn print_all_amount_units() {
    println!("* Amount units");
    println!(
        "  Unit: {}, {};",
        Measure::<Unit>::new(1.),
        MeasurePoint::<Unit>::new(1.),
    );
    println!(
        "  Dozen: {}, {}, {}, {};",
        Measure::<Dozen>::new(1.),
        Measure::<Dozen>::new(1.).convert::<Unit>(),
        MeasurePoint::<Dozen>::new(1.),
        MeasurePoint::<Dozen>::new(1.).convert::<Unit>(),
    );
    println!(
        "  Mole: {}, {}, {}, {};",
        Measure::<Mole>::new(1.),
        Measure::<Mole>::new(1.).convert::<Unit>(),
        MeasurePoint::<Mole>::new(1.),
        MeasurePoint::<Mole>::new(1.).convert::<Unit>(),
    );
    println!();
}

fn print_all_angle_units() {
    println!("** Angle units **");
    println!(
        "  Radian: {}, {}, {}, {};",
        Measure::<Radian>::new(1.),
        MeasurePoint::<Radian>::new(1.),
        SignedDirection::<Radian>::new(1.),
        UnsignedDirection::<Radian>::new(1.),
    );
    println!(
        "  Cycle: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Cycle>::new(1.),
        Measure::<Cycle>::new(1.).convert::<Radian>(),
        MeasurePoint::<Cycle>::new(1.),
        MeasurePoint::<Cycle>::new(1.).convert::<Radian>(),
        SignedDirection::<Cycle>::new(1.),
        SignedDirection::<Cycle>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Cycle>::new(1.),
        UnsignedDirection::<Cycle>::new(1.).convert::<Radian>(),
    );
    println!(
        "  Gradian: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Gradian>::new(1.),
        Measure::<Gradian>::new(1.).convert::<Radian>(),
        MeasurePoint::<Gradian>::new(1.),
        MeasurePoint::<Gradian>::new(1.).convert::<Radian>(),
        SignedDirection::<Gradian>::new(1.),
        SignedDirection::<Gradian>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Gradian>::new(1.),
        UnsignedDirection::<Gradian>::new(1.).convert::<Radian>(),
    );
    println!(
        "  Degree: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Degree>::new(1.),
        Measure::<Degree>::new(1.).convert::<Radian>(),
        MeasurePoint::<Degree>::new(1.),
        MeasurePoint::<Degree>::new(1.).convert::<Radian>(),
        SignedDirection::<Degree>::new(1.),
        SignedDirection::<Degree>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Degree>::new(1.),
        UnsignedDirection::<Degree>::new(1.).convert::<Radian>(),
    );
    println!(
        "  ArcMinute: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<ArcMinute>::new(1.),
        Measure::<ArcMinute>::new(1.).convert::<Radian>(),
        MeasurePoint::<ArcMinute>::new(1.),
        MeasurePoint::<ArcMinute>::new(1.).convert::<Radian>(),
        SignedDirection::<ArcMinute>::new(1.),
        SignedDirection::<ArcMinute>::new(1.).convert::<Radian>(),
        UnsignedDirection::<ArcMinute>::new(1.),
        UnsignedDirection::<ArcMinute>::new(1.).convert::<Radian>(),
    );
    println!(
        "  ArcSecond: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<ArcSecond>::new(1.),
        Measure::<ArcSecond>::new(1.).convert::<Radian>(),
        MeasurePoint::<ArcSecond>::new(1.),
        MeasurePoint::<ArcSecond>::new(1.).convert::<Radian>(),
        SignedDirection::<ArcSecond>::new(1.),
        SignedDirection::<ArcSecond>::new(1.).convert::<Radian>(),
        UnsignedDirection::<ArcSecond>::new(1.),
        UnsignedDirection::<ArcSecond>::new(1.).convert::<Radian>(),
    );
    println!();
}

fn print_all_angular_acceleration_units() {
    println!("* AngularAcceleration units");
    println!(
        "  RadianPerSquareSecond: {}, {};",
        Measure::<RadianPerSquareSecond>::new(1.),
        MeasurePoint::<RadianPerSquareSecond>::new(1.),
    );
    println!();
}

fn print_all_angular_momentum_units() {
    println!("* AngularMomentum units");
    println!(
        "  KiloGramSquareMetrePerSecond: {}, {}, {}, {}, {}, {};",
        Measure::<KiloGramSquareMetrePerSecond>::new(1.),
        MeasurePoint::<KiloGramSquareMetrePerSecond>::new(1.),
        Measure2d::<KiloGramSquareMetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<KiloGramSquareMetrePerSecond>::new(1., 2.),
        Measure3d::<KiloGramSquareMetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<KiloGramSquareMetrePerSecond>::new(1., 2., 3.),
    );
    println!(
        "  GramSquareCentiMetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<GramSquareCentiMetrePerSecond>::new(1.),
        Measure::<GramSquareCentiMetrePerSecond>::new(1.).convert::<KiloGramSquareMetrePerSecond>(),
        MeasurePoint::<GramSquareCentiMetrePerSecond>::new(1.),
        MeasurePoint::<GramSquareCentiMetrePerSecond>::new(1.).convert::<KiloGramSquareMetrePerSecond>(),
        Measure2d::<GramSquareCentiMetrePerSecond>::new(1., 2.),
        Measure2d::<GramSquareCentiMetrePerSecond>::new(1., 2.).convert::<KiloGramSquareMetrePerSecond>(),
        MeasurePoint2d::<GramSquareCentiMetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<GramSquareCentiMetrePerSecond>::new(1., 2.).convert::<KiloGramSquareMetrePerSecond>(),
        Measure3d::<GramSquareCentiMetrePerSecond>::new(1., 2., 3.),
        Measure3d::<GramSquareCentiMetrePerSecond>::new(1., 2., 3.).convert::<KiloGramSquareMetrePerSecond>(),
        MeasurePoint3d::<GramSquareCentiMetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<GramSquareCentiMetrePerSecond>::new(1., 2., 3.).convert::<KiloGramSquareMetrePerSecond>(),
    );
    println!();
}

fn print_all_area_units() {
    println!("* Area units");
    println!(
        "  SquareMetre: {}, {};",
        Measure::<SquareMetre>::new(1.),
        MeasurePoint::<SquareMetre>::new(1.)
    );
    println!(
        "  SquareKiloMetre: {} == {}, {} == {};",
        Measure::<SquareKiloMetre>::new(1.),
        Measure::<SquareKiloMetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareKiloMetre>::new(1.),
        MeasurePoint::<SquareKiloMetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  Hectare: {} == {}, {} == {};",
        Measure::<Hectare>::new(1.),
        Measure::<Hectare>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<Hectare>::new(1.),
        MeasurePoint::<Hectare>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  Are: {} == {}, {} == {};",
        Measure::<Are>::new(1.),
        Measure::<Are>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<Are>::new(1.),
        MeasurePoint::<Are>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareDeciMetre: {} == {}, {} == {};",
        Measure::<SquareDeciMetre>::new(1.),
        Measure::<SquareDeciMetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareDeciMetre>::new(1.),
        MeasurePoint::<SquareDeciMetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareCentiMetre: {} == {}, {} == {};",
        Measure::<SquareCentiMetre>::new(1.),
        Measure::<SquareCentiMetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareCentiMetre>::new(1.),
        MeasurePoint::<SquareCentiMetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareMilliMetre: {} == {}, {} == {};",
        Measure::<SquareMilliMetre>::new(1.),
        Measure::<SquareMilliMetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareMilliMetre>::new(1.),
        MeasurePoint::<SquareMilliMetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareInch: {} == {}, {} == {};",
        Measure::<SquareInch>::new(1.),
        Measure::<SquareInch>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareInch>::new(1.),
        MeasurePoint::<SquareInch>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareFoot: {} == {}, {} == {};",
        Measure::<SquareFoot>::new(1.),
        Measure::<SquareFoot>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareFoot>::new(1.),
        MeasurePoint::<SquareFoot>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareYard: {} == {}, {} == {};",
        Measure::<SquareYard>::new(1.),
        Measure::<SquareYard>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareYard>::new(1.),
        MeasurePoint::<SquareYard>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareMile: {} == {}, {} == {};",
        Measure::<SquareMile>::new(1.),
        Measure::<SquareMile>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareMile>::new(1.),
        MeasurePoint::<SquareMile>::new(1.).convert::<SquareMetre>(),
    );
    println!();
}

fn print_all_area_density_units() {
    println!("* AreaDensity units");
    println!(
        "  KiloGramPerSquareMetre: {}, {};",
        Measure::<KiloGramPerSquareMetre>::new(1.),
        MeasurePoint::<KiloGramPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_capacitance_units() {
    println!("* Capacitance units");
    println!(
        "  Farad: {}, {};",
        Measure::<Farad>::new(1.),
        MeasurePoint::<Farad>::new(1.),
    );
    println!(
        "  MilliFarad: {} == {}, {} == {};",
        Measure::<MilliFarad>::new(1.),
        Measure::<MilliFarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<MilliFarad>::new(1.),
        MeasurePoint::<MilliFarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  MicroFarad: {} == {}, {} == {};",
        Measure::<MicroFarad>::new(1.),
        Measure::<MicroFarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<MicroFarad>::new(1.),
        MeasurePoint::<MicroFarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  NanoFarad: {} == {}, {} == {};",
        Measure::<NanoFarad>::new(1.),
        Measure::<NanoFarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<NanoFarad>::new(1.),
        MeasurePoint::<NanoFarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  PicoFarad: {} == {}, {} == {};",
        Measure::<PicoFarad>::new(1.),
        Measure::<PicoFarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<PicoFarad>::new(1.),
        MeasurePoint::<PicoFarad>::new(1.).convert::<Farad>(),
    );
    println!();
}

fn print_all_catalytic_activity_units() {
    println!("* CatalyticActivity units");
    println!(
        "  Katal: {}, {};",
        Measure::<Katal>::new(1.),
        MeasurePoint::<Katal>::new(1.),
    );
    println!();
}

fn print_all_chemical_potential_units() {
    println!("* ChemicalPotential units");
    println!(
        "  JoulePerMole: {}, {};",
        Measure::<JoulePerMole>::new(1.),
        MeasurePoint::<JoulePerMole>::new(1.),
    );
    println!();
}

fn print_all_current_density_units() {
    println!("* CurrentDensity units");
    println!(
        "  AmperePerSquareMetre: {}, {}, {}, {}, {}, {};",
        Measure::<AmperePerSquareMetre>::new(1.),
        MeasurePoint::<AmperePerSquareMetre>::new(1.),
        Measure2d::<AmperePerSquareMetre>::new(1., 2.),
        MeasurePoint2d::<AmperePerSquareMetre>::new(1., 2.),
        Measure3d::<AmperePerSquareMetre>::new(1., 2., 3.),
        MeasurePoint3d::<AmperePerSquareMetre>::new(1., 2., 3.),
    );
    println!();
}

fn print_all_dimensionless_units() {
    println!("* Dimensionless units");
    println!(
        "  Unspecified: {}, {}, {}, {}, {}, {};",
        Measure::<Unspecified>::new(1.),
        MeasurePoint::<Unspecified>::new(1.),
        Measure2d::<Unspecified>::new(1., 2.),
        MeasurePoint2d::<Unspecified>::new(1., 2.),
        Measure3d::<Unspecified>::new(1., 2., 3.),
        MeasurePoint3d::<Unspecified>::new(1., 2., 3.)
    );
    println!(
        "  Mach: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Mach>::new(1.),
        Measure::<Mach>::new(1.).convert::<Unspecified>(),
        MeasurePoint::<Mach>::new(1.),
        MeasurePoint::<Mach>::new(1.).convert::<Unspecified>(),
        Measure2d::<Mach>::new(1., 2.),
        Measure2d::<Mach>::new(1., 2.).convert::<Unspecified>(),
        MeasurePoint2d::<Mach>::new(1., 2.),
        MeasurePoint2d::<Mach>::new(1., 2.).convert::<Unspecified>(),
        Measure3d::<Mach>::new(1., 2., 3.),
        Measure3d::<Mach>::new(1., 2., 3.).convert::<Unspecified>(),
        MeasurePoint3d::<Mach>::new(1., 2., 3.),
        MeasurePoint3d::<Mach>::new(1., 2., 3.).convert::<Unspecified>(),
    );
    println!();
}

fn print_all_dose_equivalent_units() {
    println!("* DoseEquivalent units");
    println!(
        "  Sievert: {}, {};",
        Measure::<Sievert>::new(1.),
        MeasurePoint::<Sievert>::new(1.),
    );
    println!(
        "  Rem: {} == {}, {} == {};",
        Measure::<Rem>::new(1.),
        Measure::<Rem>::new(1.).convert::<Sievert>(),
        MeasurePoint::<Rem>::new(1.),
        MeasurePoint::<Rem>::new(1.).convert::<Sievert>(),
    );
    println!();
}

fn print_all_dynamic_viscosity_units() {
    println!("* DynamicViscosity units");
    println!(
        "  PascalSecond: {}, {};",
        Measure::<PascalSecond>::new(1.),
        MeasurePoint::<PascalSecond>::new(1.),
    );
    println!();
}

fn print_all_electrical_conductance_units() {
    println!("* ElectricalConductance units");
    println!(
        "  Siemens: {}, {};",
        Measure::<Siemens>::new(1.),
        MeasurePoint::<Siemens>::new(1.),
    );
    println!();
}

fn print_all_electrical_conductivity_units() {
    println!("* ElectricalConductivity units");
    println!(
        "  SiemensPerMetre: {}, {};",
        Measure::<SiemensPerMetre>::new(1.),
        MeasurePoint::<SiemensPerMetre>::new(1.),
    );
    println!();
}

fn print_all_electrical_resistance_units() {
    println!("* ElectricalResistance units");
    println!(
        "  Ohm: {}, {};",
        Measure::<Ohm>::new(1.),
        MeasurePoint::<Ohm>::new(1.),
    );
    println!(
        "  MilliOhm: {} == {}, {} == {};",
        Measure::<MilliOhm>::new(1.),
        Measure::<MilliOhm>::new(1.).convert::<Ohm>(),
        MeasurePoint::<MilliOhm>::new(1.),
        MeasurePoint::<MilliOhm>::new(1.).convert::<Ohm>(),
    );
    println!(
        "  KiloOhm: {} == {}, {} == {};",
        Measure::<KiloOhm>::new(1.),
        Measure::<KiloOhm>::new(1.).convert::<Ohm>(),
        MeasurePoint::<KiloOhm>::new(1.),
        MeasurePoint::<KiloOhm>::new(1.).convert::<Ohm>(),
    );
    println!();
}

fn print_all_electrical_resistivity_units() {
    println!("* ElectricalResistivity units");
    println!(
        "  OhmMetre: {}, {};",
        Measure::<OhmMetre>::new(1.),
        MeasurePoint::<OhmMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_charge_units() {
    println!("* ElectricCharge units");
    println!(
        "  Coulomb: {}, {};",
        Measure::<Coulomb>::new(1.),
        MeasurePoint::<Coulomb>::new(1.),
    );
    println!(
        "  MilliCoulomb: {} == {}, {} == {};",
        Measure::<MilliCoulomb>::new(1.),
        Measure::<MilliCoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<MilliCoulomb>::new(1.),
        MeasurePoint::<MilliCoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  MicroCoulomb: {} == {}, {} == {};",
        Measure::<MicroCoulomb>::new(1.),
        Measure::<MicroCoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<MicroCoulomb>::new(1.),
        MeasurePoint::<MicroCoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  NanoCoulomb: {} == {}, {} == {};",
        Measure::<NanoCoulomb>::new(1.),
        Measure::<NanoCoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<NanoCoulomb>::new(1.),
        MeasurePoint::<NanoCoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  PicoCoulomb: {} == {}, {} == {};",
        Measure::<PicoCoulomb>::new(1.),
        Measure::<PicoCoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<PicoCoulomb>::new(1.),
        MeasurePoint::<PicoCoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!();
}

fn print_all_electric_charge_density_units() {
    println!("* ElectricChargeDensity units");
    println!(
        "  CoulombPerCubicMetre: {}, {};",
        Measure::<CoulombPerCubicMetre>::new(1.),
        MeasurePoint::<CoulombPerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_current_units() {
    println!("* ElectricCurrent units");
    println!(
        "  Ampere: {}, {};",
        Measure::<Ampere>::new(1.),
        MeasurePoint::<Ampere>::new(1.),
    );
    println!(
        "  MilliAmpere: {} == {}, {} == {};",
        Measure::<MilliAmpere>::new(1.),
        Measure::<MilliAmpere>::new(1.).convert::<Ampere>(),
        MeasurePoint::<MilliAmpere>::new(1.),
        MeasurePoint::<MilliAmpere>::new(1.).convert::<Ampere>(),
    );
    println!(
        "  MicroAmpere: {} == {}, {} == {};",
        Measure::<MicroAmpere>::new(1.),
        Measure::<MicroAmpere>::new(1.).convert::<Ampere>(),
        MeasurePoint::<MicroAmpere>::new(1.),
        MeasurePoint::<MicroAmpere>::new(1.).convert::<Ampere>(),
    );
    println!();
}

fn print_all_electric_displacement_units() {
    println!("* ElectricDisplacement units");
    println!(
        "  CoulombPerSquareMetre: {}, {};",
        Measure::<CoulombPerSquareMetre>::new(1.),
        MeasurePoint::<CoulombPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_field_strength_units() {
    println!("* ElectricFieldStrength units");
    println!(
        "  VoltPerMetre: {}, {}, {}, {}, {}, {};",
        Measure::<VoltPerMetre>::new(1.),
        MeasurePoint::<VoltPerMetre>::new(1.),
        Measure2d::<VoltPerMetre>::new(1., 2.),
        MeasurePoint2d::<VoltPerMetre>::new(1., 2.),
        Measure3d::<VoltPerMetre>::new(1., 2., 3.),
        MeasurePoint3d::<VoltPerMetre>::new(1., 2., 3.),
    );
    println!();
}

fn print_all_electric_potential_units() {
    println!("* ElectricPotential units");
    println!(
        "  Volt: {}, {};",
        Measure::<Volt>::new(1.),
        MeasurePoint::<Volt>::new(1.),
    );
    println!(
        "  KiloVolt: {} == {}, {} == {};",
        Measure::<KiloVolt>::new(1.),
        Measure::<KiloVolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<KiloVolt>::new(1.),
        MeasurePoint::<KiloVolt>::new(1.).convert::<Volt>(),
    );
    println!(
        "  MilliVolt: {} == {}, {} == {};",
        Measure::<MilliVolt>::new(1.),
        Measure::<MilliVolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<MilliVolt>::new(1.),
        MeasurePoint::<MilliVolt>::new(1.).convert::<Volt>(),
    );
    println!(
        "  MicroVolt: {} == {}, {} == {};",
        Measure::<MicroVolt>::new(1.),
        Measure::<MicroVolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<MicroVolt>::new(1.),
        MeasurePoint::<MicroVolt>::new(1.).convert::<Volt>(),
    );
    println!();
}

fn print_all_energy_units() {
    println!("* Energy units");
    println!(
        "  Joule: {}, {};",
        Measure::<Joule>::new(1.),
        MeasurePoint::<Joule>::new(1.),
    );
    println!(
        "  WattHour: {} == {}, {} == {};",
        Measure::<WattHour>::new(1.),
        Measure::<WattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<WattHour>::new(1.),
        MeasurePoint::<WattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  KiloWattHour: {} == {}, {} == {};",
        Measure::<KiloWattHour>::new(1.),
        Measure::<KiloWattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<KiloWattHour>::new(1.),
        MeasurePoint::<KiloWattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  MegaWattHour: {} == {}, {} == {};",
        Measure::<MegaWattHour>::new(1.),
        Measure::<MegaWattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<MegaWattHour>::new(1.),
        MeasurePoint::<MegaWattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Calorie: {} == {}, {} == {};",
        Measure::<Calorie>::new(1.),
        Measure::<Calorie>::new(1.).convert::<Joule>(),
        MeasurePoint::<Calorie>::new(1.),
        MeasurePoint::<Calorie>::new(1.).convert::<Joule>(),
    );
    println!(
        "  KiloCalorie: {} == {}, {} == {};",
        Measure::<KiloCalorie>::new(1.),
        Measure::<KiloCalorie>::new(1.).convert::<Joule>(),
        MeasurePoint::<KiloCalorie>::new(1.),
        MeasurePoint::<KiloCalorie>::new(1.).convert::<Joule>(),
    );
    println!(
        "  ElectronVolt: {} == {}, {} == {};",
        Measure::<ElectronVolt>::new(1.),
        Measure::<ElectronVolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<ElectronVolt>::new(1.),
        MeasurePoint::<ElectronVolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  KiloElectronVolt: {} == {}, {} == {};",
        Measure::<KiloElectronVolt>::new(1.),
        Measure::<KiloElectronVolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<KiloElectronVolt>::new(1.),
        MeasurePoint::<KiloElectronVolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  MegaElectronVolt: {} == {}, {} == {};",
        Measure::<MegaElectronVolt>::new(1.),
        Measure::<MegaElectronVolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<MegaElectronVolt>::new(1.),
        MeasurePoint::<MegaElectronVolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  GigaElectronVolt: {} == {}, {} == {};",
        Measure::<GigaElectronVolt>::new(1.),
        Measure::<GigaElectronVolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<GigaElectronVolt>::new(1.),
        MeasurePoint::<GigaElectronVolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  TeraElectronVolt: {} == {}, {} == {};",
        Measure::<TeraElectronVolt>::new(1.),
        Measure::<TeraElectronVolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<TeraElectronVolt>::new(1.),
        MeasurePoint::<TeraElectronVolt>::new(1.).convert::<Joule>(),
    );
    println!();
}

fn print_all_energy_density_units() {
    println!("* EnergyDensity units");
    println!(
        "  JoulePerCubicMetre: {}, {};",
        Measure::<JoulePerCubicMetre>::new(1.),
        MeasurePoint::<JoulePerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_entropy_units() {
    println!("* Entropy units");
    println!(
        "  JoulePerKelvin: {}, {};",
        Measure::<JoulePerKelvin>::new(1.),
        MeasurePoint::<JoulePerKelvin>::new(1.),
    );
    println!();
}

fn print_all_force_units() {
    println!("* Force units");
    println!(
        "  Newton: {}, {}, {}, {}, {}, {};",
        Measure::<Newton>::new(1.),
        MeasurePoint::<Newton>::new(1.),
        Measure2d::<Newton>::new(1., 2.),
        MeasurePoint2d::<Newton>::new(1., 2.),
        Measure3d::<Newton>::new(1., 2., 3.),
        MeasurePoint3d::<Newton>::new(1., 2., 3.),
    );
    println!(
        "  Dyne: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Dyne>::new(1.),
        Measure::<Dyne>::new(1.).convert::<Newton>(),
        MeasurePoint::<Dyne>::new(1.),
        MeasurePoint::<Dyne>::new(1.).convert::<Newton>(),
        Measure2d::<Dyne>::new(1., 2.),
        Measure2d::<Dyne>::new(1., 2.).convert::<Newton>(),
        MeasurePoint2d::<Dyne>::new(1., 2.),
        MeasurePoint2d::<Dyne>::new(1., 2.).convert::<Newton>(),
        Measure3d::<Dyne>::new(1., 2., 3.),
        Measure3d::<Dyne>::new(1., 2., 3.).convert::<Newton>(),
        MeasurePoint3d::<Dyne>::new(1., 2., 3.),
        MeasurePoint3d::<Dyne>::new(1., 2., 3.).convert::<Newton>(),
    );
    println!(
        "  KiloGramForce: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KiloGramForce>::new(1.),
        Measure::<KiloGramForce>::new(1.).convert::<Newton>(),
        MeasurePoint::<KiloGramForce>::new(1.),
        MeasurePoint::<KiloGramForce>::new(1.).convert::<Newton>(),
        Measure2d::<KiloGramForce>::new(1., 2.),
        Measure2d::<KiloGramForce>::new(1., 2.).convert::<Newton>(),
        MeasurePoint2d::<KiloGramForce>::new(1., 2.),
        MeasurePoint2d::<KiloGramForce>::new(1., 2.).convert::<Newton>(),
        Measure3d::<KiloGramForce>::new(1., 2., 3.),
        Measure3d::<KiloGramForce>::new(1., 2., 3.).convert::<Newton>(),
        MeasurePoint3d::<KiloGramForce>::new(1., 2., 3.),
        MeasurePoint3d::<KiloGramForce>::new(1., 2., 3.).convert::<Newton>(),
    );
    println!(
        "  PoundForce: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<PoundForce>::new(1.),
        Measure::<PoundForce>::new(1.).convert::<Newton>(),
        MeasurePoint::<PoundForce>::new(1.),
        MeasurePoint::<PoundForce>::new(1.).convert::<Newton>(),
        Measure2d::<PoundForce>::new(1., 2.),
        Measure2d::<PoundForce>::new(1., 2.).convert::<Newton>(),
        MeasurePoint2d::<PoundForce>::new(1., 2.),
        MeasurePoint2d::<PoundForce>::new(1., 2.).convert::<Newton>(),
        Measure3d::<PoundForce>::new(1., 2., 3.),
        Measure3d::<PoundForce>::new(1., 2., 3.).convert::<Newton>(),
        MeasurePoint3d::<PoundForce>::new(1., 2., 3.),
        MeasurePoint3d::<PoundForce>::new(1., 2., 3.).convert::<Newton>(),
    );
    println!(
        "  Poundal: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Poundal>::new(1.),
        Measure::<Poundal>::new(1.).convert::<Newton>(),
        MeasurePoint::<Poundal>::new(1.),
        MeasurePoint::<Poundal>::new(1.).convert::<Newton>(),
        Measure2d::<Poundal>::new(1., 2.),
        Measure2d::<Poundal>::new(1., 2.).convert::<Newton>(),
        MeasurePoint2d::<Poundal>::new(1., 2.),
        MeasurePoint2d::<Poundal>::new(1., 2.).convert::<Newton>(),
        Measure3d::<Poundal>::new(1., 2., 3.),
        Measure3d::<Poundal>::new(1., 2., 3.).convert::<Newton>(),
        MeasurePoint3d::<Poundal>::new(1., 2., 3.),
        MeasurePoint3d::<Poundal>::new(1., 2., 3.).convert::<Newton>(),
    );
    println!();
}

fn print_all_frequency_units() {
    println!("* All Frequency units");
    println!(
        "  Hertz: {}, {};",
        Measure::<Hertz>::new(1.),
        MeasurePoint::<Hertz>::new(1.),
    );
    println!(
        "  KiloHertz: {} == {}, {} == {};",
        Measure::<KiloHertz>::new(1.),
        Measure::<KiloHertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<KiloHertz>::new(1.),
        MeasurePoint::<KiloHertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  MegaHertz: {} == {}, {} == {};",
        Measure::<MegaHertz>::new(1.),
        Measure::<MegaHertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<MegaHertz>::new(1.),
        MeasurePoint::<MegaHertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  GigaHertz: {} == {}, {} == {};",
        Measure::<GigaHertz>::new(1.),
        Measure::<GigaHertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<GigaHertz>::new(1.),
        MeasurePoint::<GigaHertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  RadianPerSecond: {} == {}, {} == {};",
        Measure::<RadianPerSecond>::new(1.),
        Measure::<RadianPerSecond>::new(1.).convert::<Hertz>(),
        MeasurePoint::<RadianPerSecond>::new(1.),
        MeasurePoint::<RadianPerSecond>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  CyclePerMinute: {} == {}, {} == {};",
        Measure::<CyclePerMinute>::new(1.),
        Measure::<CyclePerMinute>::new(1.).convert::<Hertz>(),
        MeasurePoint::<CyclePerMinute>::new(1.),
        MeasurePoint::<CyclePerMinute>::new(1.).convert::<Hertz>(),
    );
    println!();
}

fn print_all_illuminance_units() {
    println!("* Illuminance units");
    println!(
        "  Lux: {}, {};",
        Measure::<Lux>::new(1.),
        MeasurePoint::<Lux>::new(1.),
    );
    println!(
        "  Phot: {} == {}, {} == {};",
        Measure::<Phot>::new(1.),
        Measure::<Phot>::new(1.).convert::<Lux>(),
        MeasurePoint::<Phot>::new(1.),
        MeasurePoint::<Phot>::new(1.).convert::<Lux>(),
    );
    println!(
        "  FootCandle: {} == {}, {} == {};",
        Measure::<FootCandle>::new(1.),
        Measure::<FootCandle>::new(1.).convert::<Lux>(),
        MeasurePoint::<FootCandle>::new(1.),
        MeasurePoint::<FootCandle>::new(1.).convert::<Lux>(),
    );
    println!();
}

fn print_all_inductance_units() {
    println!("* Inductance units");
    println!(
        "  Henry: {}, {};",
        Measure::<Henry>::new(1.),
        MeasurePoint::<Henry>::new(1.),
    );
    println!();
}

fn print_all_information_units() {
    println!("* Information units");
    println!(
        "  Bit: {}, {};",
        Measure::<Bit>::new(1.),
        MeasurePoint::<Bit>::new(1.),
    );
    println!(
        "  Byte: {} == {}, {} == {};",
        Measure::<Byte>::new(1.),
        Measure::<Byte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Byte>::new(1.),
        MeasurePoint::<Byte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  KiloBit: {} == {}, {} == {};",
        Measure::<KiloBit>::new(1.),
        Measure::<KiloBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<KiloBit>::new(1.),
        MeasurePoint::<KiloBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  KiloByte: {} == {}, {} == {};",
        Measure::<KiloByte>::new(1.),
        Measure::<KiloByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<KiloByte>::new(1.),
        MeasurePoint::<KiloByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  KibiBit: {} == {}, {} == {};",
        Measure::<KibiBit>::new(1.),
        Measure::<KibiBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<KibiBit>::new(1.),
        MeasurePoint::<KibiBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  KibiByte: {} == {}, {} == {};",
        Measure::<KibiByte>::new(1.),
        Measure::<KibiByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<KibiByte>::new(1.),
        MeasurePoint::<KibiByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  MegaBit: {} == {}, {} == {};",
        Measure::<MegaBit>::new(1.),
        Measure::<MegaBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<MegaBit>::new(1.),
        MeasurePoint::<MegaBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  MegaByte: {} == {}, {} == {};",
        Measure::<MegaByte>::new(1.),
        Measure::<MegaByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<MegaByte>::new(1.),
        MeasurePoint::<MegaByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  MebiBit: {} == {}, {} == {};",
        Measure::<MebiBit>::new(1.),
        Measure::<MebiBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<MebiBit>::new(1.),
        MeasurePoint::<MebiBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  MebiByte: {} == {}, {} == {};",
        Measure::<MebiByte>::new(1.),
        Measure::<MebiByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<MebiByte>::new(1.),
        MeasurePoint::<MebiByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  GigaBit: {} == {}, {} == {};",
        Measure::<GigaBit>::new(1.),
        Measure::<GigaBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<GigaBit>::new(1.),
        MeasurePoint::<GigaBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  GigaByte: {} == {}, {} == {};",
        Measure::<GigaByte>::new(1.),
        Measure::<GigaByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<GigaByte>::new(1.),
        MeasurePoint::<GigaByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  GibiBit: {} == {}, {} == {};",
        Measure::<GibiBit>::new(1.),
        Measure::<GibiBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<GibiBit>::new(1.),
        MeasurePoint::<GibiBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  GibiByte: {} == {}, {} == {};",
        Measure::<GibiByte>::new(1.),
        Measure::<GibiByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<GibiByte>::new(1.),
        MeasurePoint::<GibiByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  TeraBit: {} == {}, {} == {};",
        Measure::<TeraBit>::new(1.),
        Measure::<TeraBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<TeraBit>::new(1.),
        MeasurePoint::<TeraBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  TeraByte: {} == {}, {} == {};",
        Measure::<TeraByte>::new(1.),
        Measure::<TeraByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<TeraByte>::new(1.),
        MeasurePoint::<TeraByte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  TebiBit: {} == {}, {} == {};",
        Measure::<TebiBit>::new(1.),
        Measure::<TebiBit>::new(1.).convert::<Bit>(),
        MeasurePoint::<TebiBit>::new(1.),
        MeasurePoint::<TebiBit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  TebiByte: {} == {}, {} == {};",
        Measure::<TebiByte>::new(1.),
        Measure::<TebiByte>::new(1.).convert::<Bit>(),
        MeasurePoint::<TebiByte>::new(1.),
        MeasurePoint::<TebiByte>::new(1.).convert::<Bit>(),
    );
    println!();
}

fn print_all_information_rate_units() {
    println!("* InformationRate units");
    println!(
        "  BitPerSecond: {}, {};",
        Measure::<BitPerSecond>::new(1.),
        MeasurePoint::<BitPerSecond>::new(1.),
    );
    println!(
        "  BytePerSecond: {} == {}, {} == {};",
        Measure::<BytePerSecond>::new(1.),
        Measure::<BytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<BytePerSecond>::new(1.),
        MeasurePoint::<BytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  KiloBitPerSecond: {} == {}, {} == {};",
        Measure::<KiloBitPerSecond>::new(1.),
        Measure::<KiloBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KiloBitPerSecond>::new(1.),
        MeasurePoint::<KiloBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KiloBytePerSecond: {} == {}, {} == {};",
        Measure::<KiloBytePerSecond>::new(1.),
        Measure::<KiloBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KiloBytePerSecond>::new(1.),
        MeasurePoint::<KiloBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KibiBitPerSecond: {} == {}, {} == {};",
        Measure::<KibiBitPerSecond>::new(1.),
        Measure::<KibiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KibiBitPerSecond>::new(1.),
        MeasurePoint::<KibiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KibiBytePerSecond: {} == {}, {} == {};",
        Measure::<KibiBytePerSecond>::new(1.),
        Measure::<KibiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KibiBytePerSecond>::new(1.),
        MeasurePoint::<KibiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  MegaBitPerSecond: {} == {}, {} == {};",
        Measure::<MegaBitPerSecond>::new(1.),
        Measure::<MegaBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MegaBitPerSecond>::new(1.),
        MeasurePoint::<MegaBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MegaBytePerSecond: {} == {}, {} == {};",
        Measure::<MegaBytePerSecond>::new(1.),
        Measure::<MegaBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MegaBytePerSecond>::new(1.),
        MeasurePoint::<MegaBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MebiBitPerSecond: {} == {}, {} == {};",
        Measure::<MebiBitPerSecond>::new(1.),
        Measure::<MebiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MebiBitPerSecond>::new(1.),
        MeasurePoint::<MebiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MebiBytePerSecond: {} == {}, {} == {};",
        Measure::<MebiBytePerSecond>::new(1.),
        Measure::<MebiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MebiBytePerSecond>::new(1.),
        MeasurePoint::<MebiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  GigaBitPerSecond: {} == {}, {} == {};",
        Measure::<GigaBitPerSecond>::new(1.),
        Measure::<GigaBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GigaBitPerSecond>::new(1.),
        MeasurePoint::<GigaBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GigaBytePerSecond: {} == {}, {} == {};",
        Measure::<GigaBytePerSecond>::new(1.),
        Measure::<GigaBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GigaBytePerSecond>::new(1.),
        MeasurePoint::<GigaBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GibiBitPerSecond: {} == {}, {} == {};",
        Measure::<GibiBitPerSecond>::new(1.),
        Measure::<GibiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GibiBitPerSecond>::new(1.),
        MeasurePoint::<GibiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GibiBytePerSecond: {} == {}, {} == {};",
        Measure::<GibiBytePerSecond>::new(1.),
        Measure::<GibiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GibiBytePerSecond>::new(1.),
        MeasurePoint::<GibiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  TeraBitPerSecond: {} == {}, {} == {};",
        Measure::<TeraBitPerSecond>::new(1.),
        Measure::<TeraBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TeraBitPerSecond>::new(1.),
        MeasurePoint::<TeraBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TeraBytePerSecond: {} == {}, {} == {};",
        Measure::<TeraBytePerSecond>::new(1.),
        Measure::<TeraBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TeraBytePerSecond>::new(1.),
        MeasurePoint::<TeraBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TebiBitPerSecond: {} == {}, {} == {};",
        Measure::<TebiBitPerSecond>::new(1.),
        Measure::<TebiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TebiBitPerSecond>::new(1.),
        MeasurePoint::<TebiBitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TebiBytePerSecond: {} == {}, {} == {};",
        Measure::<TebiBytePerSecond>::new(1.),
        Measure::<TebiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TebiBytePerSecond>::new(1.),
        MeasurePoint::<TebiBytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!();
}

fn print_all_irradiance_units() {
    println!("* Irradiance units");
    println!(
        "  WattPerSquareMetre: {}, {};",
        Measure::<WattPerSquareMetre>::new(1.),
        MeasurePoint::<WattPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_kinematic_viscosity_units() {
    println!("* KinematicViscosity units");
    println!(
        "  SquareMetrePerSecond: {}, {};",
        Measure::<SquareMetrePerSecond>::new(1.),
        MeasurePoint::<SquareMetrePerSecond>::new(1.),
    );
    println!(
        "  Stoke: {} == {}, {} == {};",
        Measure::<Stoke>::new(1.),
        Measure::<Stoke>::new(1.).convert::<SquareMetrePerSecond>(),
        MeasurePoint::<Stoke>::new(1.),
        MeasurePoint::<Stoke>::new(1.).convert::<SquareMetrePerSecond>(),
    );
    println!(
        "  CentiStoke: {} == {}, {} == {};",
        Measure::<CentiStoke>::new(1.),
        Measure::<CentiStoke>::new(1.).convert::<SquareMetrePerSecond>(),
        MeasurePoint::<CentiStoke>::new(1.),
        MeasurePoint::<CentiStoke>::new(1.).convert::<SquareMetrePerSecond>(),
    );
    println!();
}

fn print_all_length_units() {
    println!("* Length units");
    println!(
        "  Metre: {}, {}, {}, {}, {}, {};",
        Measure::<Metre>::new(1.),
        MeasurePoint::<Metre>::new(1.),
        Measure2d::<Metre>::new(1., 2.),
        MeasurePoint2d::<Metre>::new(1., 2.),
        Measure3d::<Metre>::new(1., 2., 3.),
        MeasurePoint3d::<Metre>::new(1., 2., 3.),
    );
    println!(
        "  AstronomicalUnit: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<AstronomicalUnit>::new(1.),
        Measure::<AstronomicalUnit>::new(1.).convert::<Metre>(),
        MeasurePoint::<AstronomicalUnit>::new(1.),
        MeasurePoint::<AstronomicalUnit>::new(1.).convert::<Metre>(),
        Measure2d::<AstronomicalUnit>::new(1., 2.),
        Measure2d::<AstronomicalUnit>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<AstronomicalUnit>::new(1., 2.),
        MeasurePoint2d::<AstronomicalUnit>::new(1., 2.).convert::<Metre>(),
        Measure3d::<AstronomicalUnit>::new(1., 2., 3.),
        Measure3d::<AstronomicalUnit>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<AstronomicalUnit>::new(1., 2., 3.),
        MeasurePoint3d::<AstronomicalUnit>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Parsec: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Parsec>::new(1.),
        Measure::<Parsec>::new(1.).convert::<Metre>(),
        MeasurePoint::<Parsec>::new(1.),
        MeasurePoint::<Parsec>::new(1.).convert::<Metre>(),
        Measure2d::<Parsec>::new(1., 2.),
        Measure2d::<Parsec>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Parsec>::new(1., 2.),
        MeasurePoint2d::<Parsec>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Parsec>::new(1., 2., 3.),
        Measure3d::<Parsec>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Parsec>::new(1., 2., 3.),
        MeasurePoint3d::<Parsec>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  LightYear: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<LightYear>::new(1.),
        Measure::<LightYear>::new(1.).convert::<Metre>(),
        MeasurePoint::<LightYear>::new(1.),
        MeasurePoint::<LightYear>::new(1.).convert::<Metre>(),
        Measure2d::<LightYear>::new(1., 2.),
        Measure2d::<LightYear>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<LightYear>::new(1., 2.),
        MeasurePoint2d::<LightYear>::new(1., 2.).convert::<Metre>(),
        Measure3d::<LightYear>::new(1., 2., 3.),
        Measure3d::<LightYear>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<LightYear>::new(1., 2., 3.),
        MeasurePoint3d::<LightYear>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  KiloMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KiloMetre>::new(1.),
        Measure::<KiloMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<KiloMetre>::new(1.),
        MeasurePoint::<KiloMetre>::new(1.).convert::<Metre>(),
        Measure2d::<KiloMetre>::new(1., 2.),
        Measure2d::<KiloMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<KiloMetre>::new(1., 2.),
        MeasurePoint2d::<KiloMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<KiloMetre>::new(1., 2., 3.),
        Measure3d::<KiloMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<KiloMetre>::new(1., 2., 3.),
        MeasurePoint3d::<KiloMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  HectoMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<HectoMetre>::new(1.),
        Measure::<HectoMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<HectoMetre>::new(1.),
        MeasurePoint::<HectoMetre>::new(1.).convert::<Metre>(),
        Measure2d::<HectoMetre>::new(1., 2.),
        Measure2d::<HectoMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<HectoMetre>::new(1., 2.),
        MeasurePoint2d::<HectoMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<HectoMetre>::new(1., 2., 3.),
        Measure3d::<HectoMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<HectoMetre>::new(1., 2., 3.),
        MeasurePoint3d::<HectoMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  DecaMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<DecaMetre>::new(1.),
        Measure::<DecaMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<DecaMetre>::new(1.),
        MeasurePoint::<DecaMetre>::new(1.).convert::<Metre>(),
        Measure2d::<DecaMetre>::new(1., 2.),
        Measure2d::<DecaMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<DecaMetre>::new(1., 2.),
        MeasurePoint2d::<DecaMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<DecaMetre>::new(1., 2., 3.),
        Measure3d::<DecaMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<DecaMetre>::new(1., 2., 3.),
        MeasurePoint3d::<DecaMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  DeciMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<DeciMetre>::new(1.),
        Measure::<DeciMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<DeciMetre>::new(1.),
        MeasurePoint::<DeciMetre>::new(1.).convert::<Metre>(),
        Measure2d::<DeciMetre>::new(1., 2.),
        Measure2d::<DeciMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<DeciMetre>::new(1., 2.),
        MeasurePoint2d::<DeciMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<DeciMetre>::new(1., 2., 3.),
        Measure3d::<DeciMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<DeciMetre>::new(1., 2., 3.),
        MeasurePoint3d::<DeciMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  CentiMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<CentiMetre>::new(1.),
        Measure::<CentiMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<CentiMetre>::new(1.),
        MeasurePoint::<CentiMetre>::new(1.).convert::<Metre>(),
        Measure2d::<CentiMetre>::new(1., 2.),
        Measure2d::<CentiMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<CentiMetre>::new(1., 2.),
        MeasurePoint2d::<CentiMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<CentiMetre>::new(1., 2., 3.),
        Measure3d::<CentiMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<CentiMetre>::new(1., 2., 3.),
        MeasurePoint3d::<CentiMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  MilliMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<MilliMetre>::new(1.),
        Measure::<MilliMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<MilliMetre>::new(1.),
        MeasurePoint::<MilliMetre>::new(1.).convert::<Metre>(),
        Measure2d::<MilliMetre>::new(1., 2.),
        Measure2d::<MilliMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<MilliMetre>::new(1., 2.),
        MeasurePoint2d::<MilliMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<MilliMetre>::new(1., 2., 3.),
        Measure3d::<MilliMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<MilliMetre>::new(1., 2., 3.),
        MeasurePoint3d::<MilliMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  MicroMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<MicroMetre>::new(1.),
        Measure::<MicroMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<MicroMetre>::new(1.),
        MeasurePoint::<MicroMetre>::new(1.).convert::<Metre>(),
        Measure2d::<MicroMetre>::new(1., 2.),
        Measure2d::<MicroMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<MicroMetre>::new(1., 2.),
        MeasurePoint2d::<MicroMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<MicroMetre>::new(1., 2., 3.),
        Measure3d::<MicroMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<MicroMetre>::new(1., 2., 3.),
        MeasurePoint3d::<MicroMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  NanoMetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<NanoMetre>::new(1.),
        Measure::<NanoMetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<NanoMetre>::new(1.),
        MeasurePoint::<NanoMetre>::new(1.).convert::<Metre>(),
        Measure2d::<NanoMetre>::new(1., 2.),
        Measure2d::<NanoMetre>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<NanoMetre>::new(1., 2.),
        MeasurePoint2d::<NanoMetre>::new(1., 2.).convert::<Metre>(),
        Measure3d::<NanoMetre>::new(1., 2., 3.),
        Measure3d::<NanoMetre>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<NanoMetre>::new(1., 2., 3.),
        MeasurePoint3d::<NanoMetre>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Angstrom: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Angstrom>::new(1.),
        Measure::<Angstrom>::new(1.).convert::<Metre>(),
        MeasurePoint::<Angstrom>::new(1.),
        MeasurePoint::<Angstrom>::new(1.).convert::<Metre>(),
        Measure2d::<Angstrom>::new(1., 2.),
        Measure2d::<Angstrom>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Angstrom>::new(1., 2.),
        MeasurePoint2d::<Angstrom>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Angstrom>::new(1., 2., 3.),
        Measure3d::<Angstrom>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Angstrom>::new(1., 2., 3.),
        MeasurePoint3d::<Angstrom>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Inch: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Inch>::new(1.),
        Measure::<Inch>::new(1.).convert::<Metre>(),
        MeasurePoint::<Inch>::new(1.),
        MeasurePoint::<Inch>::new(1.).convert::<Metre>(),
        Measure2d::<Inch>::new(1., 2.),
        Measure2d::<Inch>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Inch>::new(1., 2.),
        MeasurePoint2d::<Inch>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Inch>::new(1., 2., 3.),
        Measure3d::<Inch>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Inch>::new(1., 2., 3.),
        MeasurePoint3d::<Inch>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Foot: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Foot>::new(1.),
        Measure::<Foot>::new(1.).convert::<Metre>(),
        MeasurePoint::<Foot>::new(1.),
        MeasurePoint::<Foot>::new(1.).convert::<Metre>(),
        Measure2d::<Foot>::new(1., 2.),
        Measure2d::<Foot>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Foot>::new(1., 2.),
        MeasurePoint2d::<Foot>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Foot>::new(1., 2., 3.),
        Measure3d::<Foot>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Foot>::new(1., 2., 3.),
        MeasurePoint3d::<Foot>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Yard: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Yard>::new(1.),
        Measure::<Yard>::new(1.).convert::<Metre>(),
        MeasurePoint::<Yard>::new(1.),
        MeasurePoint::<Yard>::new(1.).convert::<Metre>(),
        Measure2d::<Yard>::new(1., 2.),
        Measure2d::<Yard>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Yard>::new(1., 2.),
        MeasurePoint2d::<Yard>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Yard>::new(1., 2., 3.),
        Measure3d::<Yard>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Yard>::new(1., 2., 3.),
        MeasurePoint3d::<Yard>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  Mile: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Mile>::new(1.),
        Measure::<Mile>::new(1.).convert::<Metre>(),
        MeasurePoint::<Mile>::new(1.),
        MeasurePoint::<Mile>::new(1.).convert::<Metre>(),
        Measure2d::<Mile>::new(1., 2.),
        Measure2d::<Mile>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<Mile>::new(1., 2.),
        MeasurePoint2d::<Mile>::new(1., 2.).convert::<Metre>(),
        Measure3d::<Mile>::new(1., 2., 3.),
        Measure3d::<Mile>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<Mile>::new(1., 2., 3.),
        MeasurePoint3d::<Mile>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!(
        "  NauticalMile: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<NauticalMile>::new(1.),
        Measure::<NauticalMile>::new(1.).convert::<Metre>(),
        MeasurePoint::<NauticalMile>::new(1.),
        MeasurePoint::<NauticalMile>::new(1.).convert::<Metre>(),
        Measure2d::<NauticalMile>::new(1., 2.),
        Measure2d::<NauticalMile>::new(1., 2.).convert::<Metre>(),
        MeasurePoint2d::<NauticalMile>::new(1., 2.),
        MeasurePoint2d::<NauticalMile>::new(1., 2.).convert::<Metre>(),
        Measure3d::<NauticalMile>::new(1., 2., 3.),
        Measure3d::<NauticalMile>::new(1., 2., 3.).convert::<Metre>(),
        MeasurePoint3d::<NauticalMile>::new(1., 2., 3.),
        MeasurePoint3d::<NauticalMile>::new(1., 2., 3.).convert::<Metre>(),
    );
    println!();
}

fn print_all_linear_density_units() {
    println!("* LinearDensity units");
    println!(
        "  KiloGramPerMetre: {}, {};",
        Measure::<KiloGramPerMetre>::new(1.),
        MeasurePoint::<KiloGramPerMetre>::new(1.),
    );
    println!();
}

fn print_all_linear_electric_charge_density_units() {
    println!("* LinearElectricChargeDensity units");
    println!(
        "  CoulombPerMetre: {}, {};",
        Measure::<CoulombPerMetre>::new(1.),
        MeasurePoint::<CoulombPerMetre>::new(1.),
    );
    println!();
}

fn print_all_luminance_units() {
    println!("* Luminance units");
    println!(
        "  CandelaPerSquareMetre: {}, {};",
        Measure::<CandelaPerSquareMetre>::new(1.),
        MeasurePoint::<CandelaPerSquareMetre>::new(1.),
    );
    println!(
        "  Stilb: {} == {}, {} == {};",
        Measure::<Stilb>::new(1.),
        Measure::<Stilb>::new(1.).convert::<CandelaPerSquareMetre>(),
        MeasurePoint::<Stilb>::new(1.),
        MeasurePoint::<Stilb>::new(1.).convert::<CandelaPerSquareMetre>(),
    );
    println!();
}

fn print_all_luminous_flux_units() {
    println!("* LuminousFlux units");
    println!(
        "  Lumen: {}, {}, {}, {}, {}, {};",
        Measure::<Lumen>::new(1.),
        MeasurePoint::<Lumen>::new(1.),
        Measure2d::<Lumen>::new(1., 2.),
        MeasurePoint2d::<Lumen>::new(1., 2.),
        Measure3d::<Lumen>::new(1., 2., 3.),
        MeasurePoint3d::<Lumen>::new(1., 2., 3.),
    );
    println!();
}

fn print_all_luminous_intensity_units() {
    println!("* LuminousIntensity units");
    println!(
        "  Candela: {}, {};",
        Measure::<Candela>::new(1.),
        MeasurePoint::<Candela>::new(1.),
    );
    println!();
}

fn print_all_magnetic_field_strength_units() {
    println!("* MagneticFieldStrength units");
    println!(
        "  AmperePerMetre: {}, {}, {}, {}, {}, {};",
        Measure::<AmperePerMetre>::new(1.),
        MeasurePoint::<AmperePerMetre>::new(1.),
        Measure2d::<AmperePerMetre>::new(1., 2.),
        MeasurePoint2d::<AmperePerMetre>::new(1., 2.),
        Measure3d::<AmperePerMetre>::new(1., 2., 3.),
        MeasurePoint3d::<AmperePerMetre>::new(1., 2., 3.),
    );
    println!();
}

fn print_all_magnetic_flux_units() {
    println!("* MagneticFlux units");
    println!(
        "  Weber: {}, {};",
        Measure::<Weber>::new(1.),
        MeasurePoint::<Weber>::new(1.),
    );
    println!();
}

fn print_all_magnetic_flux_density_units() {
    println!("* MagneticFluxDensity units");
    println!(
        "  Tesla: {}, {}, {}, {}, {}, {};",
        Measure::<Tesla>::new(1.),
        MeasurePoint::<Tesla>::new(1.),
        Measure2d::<Tesla>::new(1., 2.),
        MeasurePoint2d::<Tesla>::new(1., 2.),
        Measure3d::<Tesla>::new(1., 2., 3.),
        MeasurePoint3d::<Tesla>::new(1., 2., 3.),
    );
    println!(
        "  Gauss: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Gauss>::new(1.),
        Measure::<Gauss>::new(1.).convert::<Tesla>(),
        MeasurePoint::<Gauss>::new(1.),
        MeasurePoint::<Gauss>::new(1.).convert::<Tesla>(),
        Measure2d::<Gauss>::new(1., 2.),
        Measure2d::<Gauss>::new(1., 2.).convert::<Tesla>(),
        MeasurePoint2d::<Gauss>::new(1., 2.),
        MeasurePoint2d::<Gauss>::new(1., 2.).convert::<Tesla>(),
        Measure3d::<Gauss>::new(1., 2., 3.),
        Measure3d::<Gauss>::new(1., 2., 3.).convert::<Tesla>(),
        MeasurePoint3d::<Gauss>::new(1., 2., 3.),
        MeasurePoint3d::<Gauss>::new(1., 2., 3.).convert::<Tesla>(),
    );
    println!();
}

fn print_all_magnetic_permeability_units() {
    println!("* MagneticPermeability units");
    println!(
        "  HenryPerMetre: {}, {};",
        Measure::<HenryPerMetre>::new(1.),
        MeasurePoint::<HenryPerMetre>::new(1.),
    );
    println!();
}

fn print_all_magnetic_reluctance_units() {
    println!("* MagneticReluctance units");
    println!(
        "  InverseHenry: {}, {};",
        Measure::<InverseHenry>::new(1.),
        MeasurePoint::<InverseHenry>::new(1.),
    );
    println!();
}

fn print_all_mass_units() {
    println!("* Mass units");
    println!(
        "  KiloGram: {}, {};",
        Measure::<KiloGram>::new(1.),
        MeasurePoint::<KiloGram>::new(1.),
    );
    println!(
        "  Tonne: {} == {}, {} == {};",
        Measure::<Tonne>::new(1.),
        Measure::<Tonne>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Tonne>::new(1.),
        MeasurePoint::<Tonne>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  MetricTon: {} == {}, {} == {};",
        Measure::<MetricTon>::new(1.),
        Measure::<MetricTon>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<MetricTon>::new(1.),
        MeasurePoint::<MetricTon>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  HectoGram: {} == {}, {} == {};",
        Measure::<HectoGram>::new(1.),
        Measure::<HectoGram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<HectoGram>::new(1.),
        MeasurePoint::<HectoGram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  DecaGram: {} == {}, {} == {};",
        Measure::<DecaGram>::new(1.),
        Measure::<DecaGram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<DecaGram>::new(1.),
        MeasurePoint::<DecaGram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  Gram: {} == {}, {} == {};",
        Measure::<Gram>::new(1.),
        Measure::<Gram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Gram>::new(1.),
        MeasurePoint::<Gram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  MilliGram: {} == {}, {} == {};",
        Measure::<MilliGram>::new(1.),
        Measure::<MilliGram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<MilliGram>::new(1.),
        MeasurePoint::<MilliGram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  MicroGram: {} == {}, {} == {};",
        Measure::<MicroGram>::new(1.),
        Measure::<MicroGram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<MicroGram>::new(1.),
        MeasurePoint::<MicroGram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  NanoGram: {} == {}, {} == {};",
        Measure::<NanoGram>::new(1.),
        Measure::<NanoGram>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<NanoGram>::new(1.),
        MeasurePoint::<NanoGram>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  ImperialTon: {} == {}, {} == {};",
        Measure::<ImperialTon>::new(1.),
        Measure::<ImperialTon>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<ImperialTon>::new(1.),
        MeasurePoint::<ImperialTon>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  LongTon: {} == {}, {} == {};",
        Measure::<LongTon>::new(1.),
        Measure::<LongTon>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<LongTon>::new(1.),
        MeasurePoint::<LongTon>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  USTon: {} == {}, {} == {};",
        Measure::<USTon>::new(1.),
        Measure::<USTon>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<USTon>::new(1.),
        MeasurePoint::<USTon>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  ShortTon: {} == {}, {} == {};",
        Measure::<ShortTon>::new(1.),
        Measure::<ShortTon>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<ShortTon>::new(1.),
        MeasurePoint::<ShortTon>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  Stone: {} == {}, {} == {};",
        Measure::<Stone>::new(1.),
        Measure::<Stone>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Stone>::new(1.),
        MeasurePoint::<Stone>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  Pound: {} == {}, {} == {};",
        Measure::<Pound>::new(1.),
        Measure::<Pound>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Pound>::new(1.),
        MeasurePoint::<Pound>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  Ounce: {} == {}, {} == {};",
        Measure::<Ounce>::new(1.),
        Measure::<Ounce>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Ounce>::new(1.),
        MeasurePoint::<Ounce>::new(1.).convert::<KiloGram>(),
    );
    println!(
        "  Carat: {} == {}, {} == {};",
        Measure::<Carat>::new(1.),
        Measure::<Carat>::new(1.).convert::<KiloGram>(),
        MeasurePoint::<Carat>::new(1.),
        MeasurePoint::<Carat>::new(1.).convert::<KiloGram>(),
    );
    println!();
}

fn print_all_mass_density_units() {
    println!("* MassDensity units");
    println!(
        "  KiloGramPerCubicMetre: {}, {};",
        Measure::<KiloGramPerCubicMetre>::new(1.),
        MeasurePoint::<KiloGramPerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_mass_flow_rate_units() {
    println!("* MassFlowRate units");
    println!(
        "  KiloGramPerSecond: {}, {};",
        Measure::<KiloGramPerSecond>::new(1.),
        MeasurePoint::<KiloGramPerSecond>::new(1.),
    );
    println!(
        "  GramPerSecond: {} == {}, {} == {};",
        Measure::<GramPerSecond>::new(1.),
        Measure::<GramPerSecond>::new(1.).convert::<KiloGramPerSecond>(),
        MeasurePoint::<GramPerSecond>::new(1.),
        MeasurePoint::<GramPerSecond>::new(1.).convert::<KiloGramPerSecond>(),
    );
    println!();
}

fn print_all_molar_concentration_units() {
    println!("* MolarConcentration units");
    println!(
        "  MolePerCubicMetre: {}, {};",
        Measure::<MolePerCubicMetre>::new(1.),
        MeasurePoint::<MolePerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_molar_heat_capacity_units() {
    println!("* MolarHeatCapacity units");
    println!(
        "  JoulePerKelvinPerMole: {}, {};",
        Measure::<JoulePerKelvinPerMole>::new(1.),
        MeasurePoint::<JoulePerKelvinPerMole>::new(1.),
    );
    println!();
}

fn print_all_moment_of_inertia_units() {
    println!("* MomentOfInertia units");
    println!(
        "  KiloGramSquareMetre: {}, {};",
        Measure::<KiloGramSquareMetre>::new(1.),
        MeasurePoint::<KiloGramSquareMetre>::new(1.),
    );
    println!(
        "  GramSquareCentiMetre: {} == {}, {} == {};",
        Measure::<GramSquareCentiMetre>::new(1.),
        Measure::<GramSquareCentiMetre>::new(1.).convert::<KiloGramSquareMetre>(),
        MeasurePoint::<GramSquareCentiMetre>::new(1.),
        MeasurePoint::<GramSquareCentiMetre>::new(1.).convert::<KiloGramSquareMetre>(),
    );
    println!();
}

fn print_all_momentum_units() {
    println!("* Momentum units");
    println!(
        "  NewtonSecond: {}, {}, {}, {}, {}, {};",
        Measure::<NewtonSecond>::new(1.),
        MeasurePoint::<NewtonSecond>::new(1.),
        Measure2d::<NewtonSecond>::new(1., 2.),
        MeasurePoint2d::<NewtonSecond>::new(1., 2.),
        Measure3d::<NewtonSecond>::new(1., 2., 3.),
        MeasurePoint3d::<NewtonSecond>::new(1., 2., 3.),
    );
    println!(
        "  KiloGramMetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KiloGramMetrePerSecond>::new(1.),
        Measure::<KiloGramMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<KiloGramMetrePerSecond>::new(1.),
        MeasurePoint::<KiloGramMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<KiloGramMetrePerSecond>::new(1., 2.),
        Measure2d::<KiloGramMetrePerSecond>::new(1., 2.).convert::<NewtonSecond>(),
        MeasurePoint2d::<KiloGramMetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<KiloGramMetrePerSecond>::new(1., 2.).convert::<NewtonSecond>(),
        Measure3d::<KiloGramMetrePerSecond>::new(1., 2., 3.),
        Measure3d::<KiloGramMetrePerSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
        MeasurePoint3d::<KiloGramMetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<KiloGramMetrePerSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
    );
    println!(
        "  DynSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<DyneSecond>::new(1.),
        Measure::<DyneSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<DyneSecond>::new(1.),
        MeasurePoint::<DyneSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<DyneSecond>::new(1., 2.),
        Measure2d::<DyneSecond>::new(1., 2.).convert::<NewtonSecond>(),
        MeasurePoint2d::<DyneSecond>::new(1., 2.),
        MeasurePoint2d::<DyneSecond>::new(1., 2.).convert::<NewtonSecond>(),
        Measure3d::<DyneSecond>::new(1., 2., 3.),
        Measure3d::<DyneSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
        MeasurePoint3d::<DyneSecond>::new(1., 2., 3.),
        MeasurePoint3d::<DyneSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
    );
    println!(
        "  GramCentiMetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<GramCentiMetrePerSecond>::new(1.),
        Measure::<GramCentiMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<GramCentiMetrePerSecond>::new(1.),
        MeasurePoint::<GramCentiMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<GramCentiMetrePerSecond>::new(1., 2.),
        Measure2d::<GramCentiMetrePerSecond>::new(1., 2.).convert::<NewtonSecond>(),
        MeasurePoint2d::<GramCentiMetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<GramCentiMetrePerSecond>::new(1., 2.).convert::<NewtonSecond>(),
        Measure3d::<GramCentiMetrePerSecond>::new(1., 2., 3.),
        Measure3d::<GramCentiMetrePerSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
        MeasurePoint3d::<GramCentiMetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<GramCentiMetrePerSecond>::new(1., 2., 3.).convert::<NewtonSecond>(),
    );
    println!();
}

fn print_all_permittivity_units() {
    println!("* Permittivity units");
    println!(
        "  FaradPerMetre: {}, {};",
        Measure::<FaradPerMetre>::new(1.),
        MeasurePoint::<FaradPerMetre>::new(1.),
    );
    println!();
}

fn print_all_power_units() {
    println!("* Power units");
    println!(
        "  Watt: {}, {};",
        Measure::<Watt>::new(1.),
        MeasurePoint::<Watt>::new(1.),
    );
    println!(
        "  MilliWatt: {} == {}, {} == {};",
        Measure::<MilliWatt>::new(1.),
        Measure::<MilliWatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<MilliWatt>::new(1.),
        MeasurePoint::<MilliWatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  KiloWatt: {} == {}, {} == {};",
        Measure::<KiloWatt>::new(1.),
        Measure::<KiloWatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<KiloWatt>::new(1.),
        MeasurePoint::<KiloWatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  MegaWatt: {} == {}, {} == {};",
        Measure::<MegaWatt>::new(1.),
        Measure::<MegaWatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<MegaWatt>::new(1.),
        MeasurePoint::<MegaWatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  GigaWatt: {} == {}, {} == {};",
        Measure::<GigaWatt>::new(1.),
        Measure::<GigaWatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<GigaWatt>::new(1.),
        MeasurePoint::<GigaWatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  HorsePower: {} == {}, {} == {};",
        Measure::<HorsePower>::new(1.),
        Measure::<HorsePower>::new(1.).convert::<Watt>(),
        MeasurePoint::<HorsePower>::new(1.),
        MeasurePoint::<HorsePower>::new(1.).convert::<Watt>(),
    );
    println!();
}

fn print_all_pressure_units() {
    println!("* Pressure units");
    println!(
        "  Pascal: {}, {};",
        Measure::<Pascal>::new(1.),
        MeasurePoint::<Pascal>::new(1.),
    );
    println!(
        "  HectoPascal: {} == {}, {} == {};",
        Measure::<HectoPascal>::new(1.),
        Measure::<HectoPascal>::new(1.).convert::<Pascal>(),
        MeasurePoint::<HectoPascal>::new(1.),
        MeasurePoint::<HectoPascal>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  Atmosphere: {} == {}, {} == {};",
        Measure::<Atmosphere>::new(1.),
        Measure::<Atmosphere>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Atmosphere>::new(1.),
        MeasurePoint::<Atmosphere>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  Bar: {} == {}, {} == {};",
        Measure::<Bar>::new(1.),
        Measure::<Bar>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Bar>::new(1.),
        MeasurePoint::<Bar>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  MilliBar: {} == {}, {} == {};",
        Measure::<MilliBar>::new(1.),
        Measure::<MilliBar>::new(1.).convert::<Pascal>(),
        MeasurePoint::<MilliBar>::new(1.),
        MeasurePoint::<MilliBar>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  MmHg: {} == {}, {} == {};",
        Measure::<MmHg>::new(1.),
        Measure::<MmHg>::new(1.).convert::<Pascal>(),
        MeasurePoint::<MmHg>::new(1.),
        MeasurePoint::<MmHg>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  PoundForcePerSquareInch: {} == {}, {} == {};",
        Measure::<PoundForcePerSquareInch>::new(1.),
        Measure::<PoundForcePerSquareInch>::new(1.).convert::<Pascal>(),
        MeasurePoint::<PoundForcePerSquareInch>::new(1.),
        MeasurePoint::<PoundForcePerSquareInch>::new(1.).convert::<Pascal>(),
    );
    println!();
}

fn print_all_radiance_units() {
    println!("* Radiance units");
    println!(
        "  WattPerSquareMetrePerSteradian: {}, {};",
        Measure::<WattPerSquareMetrePerSteradian>::new(1.),
        MeasurePoint::<WattPerSquareMetrePerSteradian>::new(1.),
    );
    println!();
}

fn print_all_radiant_intensity_units() {
    println!("* RadiantIntensity units");
    println!(
        "  WattPerSteradian: {}, {};",
        Measure::<WattPerSteradian>::new(1.),
        MeasurePoint::<WattPerSteradian>::new(1.),
    );
    println!();
}

fn print_all_radioactive_activity_units() {
    println!("* RadioactiveActivity units");
    println!(
        "  Becquerel: {}, {};",
        Measure::<Becquerel>::new(1.),
        MeasurePoint::<Becquerel>::new(1.),
    );
    println!(
        "  KiloBecquerel: {} == {}, {} == {};",
        Measure::<KiloBecquerel>::new(1.),
        Measure::<KiloBecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<KiloBecquerel>::new(1.),
        MeasurePoint::<KiloBecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!(
        "  MegaBecquerel: {} == {}, {} == {};",
        Measure::<MegaBecquerel>::new(1.),
        Measure::<MegaBecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<MegaBecquerel>::new(1.),
        MeasurePoint::<MegaBecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!(
        "  GigaBecquerel: {} == {}, {} == {};",
        Measure::<GigaBecquerel>::new(1.),
        Measure::<GigaBecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<GigaBecquerel>::new(1.),
        MeasurePoint::<GigaBecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!();
}

fn print_all_radioactive_dose_units() {
    println!("* RadioactiveDose units");
    println!(
        "  Gray: {}, {};",
        Measure::<Gray>::new(1.),
        MeasurePoint::<Gray>::new(1.),
    );
    println!(
        "  Rad: {} == {}, {} == {};",
        Measure::<Rad>::new(1.),
        Measure::<Rad>::new(1.).convert::<Gray>(),
        MeasurePoint::<Rad>::new(1.),
        MeasurePoint::<Rad>::new(1.).convert::<Gray>(),
    );
    println!();
}

fn print_all_radioactive_dose_rate_units() {
    println!("* RadioactiveDoseRate units");
    println!(
        "  GrayPerSecond: {}, {};",
        Measure::<GrayPerSecond>::new(1.),
        MeasurePoint::<GrayPerSecond>::new(1.),
    );
    println!();
}

fn print_all_reaction_rate_units() {
    println!("* ReactionRate units");
    println!(
        "  MolePerCubicMetrePerSecond: {}, {};",
        Measure::<MolePerCubicMetrePerSecond>::new(1.),
        MeasurePoint::<MolePerCubicMetrePerSecond>::new(1.),
    );
    println!();
}

fn print_all_solid_angle_units() {
    println!("* SolidAngle units");
    println!(
        "  Steradian: {}, {};",
        Measure::<Steradian>::new(1.),
        MeasurePoint::<Steradian>::new(1.),
    );
    println!(
        "  Spat: {} == {}, {} == {};",
        Measure::<Spat>::new(1.),
        Measure::<Spat>::new(1.).convert::<Steradian>(),
        MeasurePoint::<Spat>::new(1.),
        MeasurePoint::<Spat>::new(1.).convert::<Steradian>(),
    );
    println!(
        "  SquareDegree: {} == {}, {} == {};",
        Measure::<SquareDegree>::new(1.),
        Measure::<SquareDegree>::new(1.).convert::<Steradian>(),
        MeasurePoint::<SquareDegree>::new(1.),
        MeasurePoint::<SquareDegree>::new(1.).convert::<Steradian>(),
    );
    println!();
}

fn print_all_specific_energy_units() {
    println!("* SpecificEnergy units");
    println!(
        "  JoulePerKiloGram: {}, {};",
        Measure::<JoulePerKiloGram>::new(1.),
        MeasurePoint::<JoulePerKiloGram>::new(1.),
    );
    println!();
}

fn print_all_specific_heat_capacity_units() {
    println!("* SpecificHeatCapacity units");
    println!(
        "  JoulePerKiloGramPerKelvin: {}, {};",
        Measure::<JoulePerKiloGramPerKelvin>::new(1.),
        MeasurePoint::<JoulePerKiloGramPerKelvin>::new(1.),
    );
    println!();
}

fn print_all_specific_volume_units() {
    println!("* SpecificVolume units");
    println!(
        "  CubicMetrePerKiloGram: {}, {};",
        Measure::<CubicMetrePerKiloGram>::new(1.),
        MeasurePoint::<CubicMetrePerKiloGram>::new(1.),
    );
    println!();
}

fn print_all_square_time_units() {
    println!("* SquareTime units");
    println!(
        "  SquareSecond: {}, {};",
        Measure::<SquareSecond>::new(1.),
        MeasurePoint::<SquareSecond>::new(1.),
    );
    println!(
        "  HourSecond: {} == {}, {} == {};",
        Measure::<HourSecond>::new(1.),
        Measure::<HourSecond>::new(1.).convert::<SquareSecond>(),
        MeasurePoint::<HourSecond>::new(1.),
        MeasurePoint::<HourSecond>::new(1.).convert::<SquareSecond>(),
    );
    println!(
        "  HourHour: {} == {}, {} == {};",
        Measure::<HourHour>::new(1.),
        Measure::<HourHour>::new(1.).convert::<SquareSecond>(),
        MeasurePoint::<HourHour>::new(1.),
        MeasurePoint::<HourHour>::new(1.).convert::<SquareSecond>(),
    );
    println!();
}

fn print_all_surface_density_units() {
    println!("* SurfaceDensity units");
    println!(
        "  KiloGramPerSquareMetre: {}, {};",
        Measure::<KiloGramPerSquareMetre>::new(1.),
        MeasurePoint::<KiloGramPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_surface_tension_units() {
    println!("* SurfaceTension units");
    println!(
        "  JoulePerSquareMetre: {}, {};",
        Measure::<JoulePerSquareMetre>::new(1.),
        MeasurePoint::<JoulePerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_temperature_units() {
    println!("* Temperature units");
    println!(
        "  Kelvin: {}, {};",
        Measure::<Kelvin>::new(1.),
        MeasurePoint::<Kelvin>::new(1.),
    );
    println!(
        "  Celsius: {} == {}, {} == {};",
        Measure::<Celsius>::new(1.),
        Measure::<Celsius>::new(1.).convert::<Kelvin>(),
        MeasurePoint::<Celsius>::new(1.),
        MeasurePoint::<Celsius>::new(1.).convert::<Kelvin>(),
    );
    println!(
        "  Fahrenheit: {} == {}, {} == {};",
        Measure::<Fahrenheit>::new(1.),
        Measure::<Fahrenheit>::new(1.).convert::<Kelvin>(),
        MeasurePoint::<Fahrenheit>::new(1.),
        MeasurePoint::<Fahrenheit>::new(1.).convert::<Kelvin>(),
    );
    println!();
}

fn print_all_thermal_conductivity_units() {
    println!("* ThermalConductivity units");
    println!(
        "  WattPerMetrePerKelvin: {}, {};",
        Measure::<WattPerMetrePerKelvin>::new(1.),
        MeasurePoint::<WattPerMetrePerKelvin>::new(1.),
    );
    println!();
}

fn print_all_time_units() {
    println!("* Time units");
    println!(
        "  Second: {}, {};",
        Measure::<Second>::new(1.),
        MeasurePoint::<Second>::new(1.),
    );
    println!(
        "  Year: {} == {}, {} == {};",
        Measure::<Year>::new(1.),
        Measure::<Year>::new(1.).convert::<Second>(),
        MeasurePoint::<Year>::new(1.),
        MeasurePoint::<Year>::new(1.).convert::<Second>(),
    );
    println!(
        "  Month: {} == {}, {} == {};",
        Measure::<Month>::new(1.),
        Measure::<Month>::new(1.).convert::<Second>(),
        MeasurePoint::<Month>::new(1.),
        MeasurePoint::<Month>::new(1.).convert::<Second>(),
    );
    println!(
        "  Week: {} == {}, {} == {};",
        Measure::<Week>::new(1.),
        Measure::<Week>::new(1.).convert::<Second>(),
        MeasurePoint::<Week>::new(1.),
        MeasurePoint::<Week>::new(1.).convert::<Second>(),
    );
    println!(
        "  Day: {} == {}, {} == {};",
        Measure::<Day>::new(1.),
        Measure::<Day>::new(1.).convert::<Second>(),
        MeasurePoint::<Day>::new(1.),
        MeasurePoint::<Day>::new(1.).convert::<Second>(),
    );
    println!(
        "  Hour: {} == {}, {} == {};",
        Measure::<Hour>::new(1.),
        Measure::<Hour>::new(1.).convert::<Second>(),
        MeasurePoint::<Hour>::new(1.),
        MeasurePoint::<Hour>::new(1.).convert::<Second>(),
    );
    println!(
        "  Minute: {} == {}, {} == {};",
        Measure::<Minute>::new(1.),
        Measure::<Minute>::new(1.).convert::<Second>(),
        MeasurePoint::<Minute>::new(1.),
        MeasurePoint::<Minute>::new(1.).convert::<Second>(),
    );
    println!(
        "  MilliSecond: {} == {}, {} == {};",
        Measure::<MilliSecond>::new(1.),
        Measure::<MilliSecond>::new(1.).convert::<Second>(),
        MeasurePoint::<MilliSecond>::new(1.),
        MeasurePoint::<MilliSecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  MicroSecond: {} == {}, {} == {};",
        Measure::<MicroSecond>::new(1.),
        Measure::<MicroSecond>::new(1.).convert::<Second>(),
        MeasurePoint::<MicroSecond>::new(1.),
        MeasurePoint::<MicroSecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  NanoSecond: {} == {}, {} == {};",
        Measure::<NanoSecond>::new(1.),
        Measure::<NanoSecond>::new(1.).convert::<Second>(),
        MeasurePoint::<NanoSecond>::new(1.),
        MeasurePoint::<NanoSecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  PicoSecond: {} == {}, {} == {};",
        Measure::<PicoSecond>::new(1.),
        Measure::<PicoSecond>::new(1.).convert::<Second>(),
        MeasurePoint::<PicoSecond>::new(1.),
        MeasurePoint::<PicoSecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  FemtoSecond: {} == {}, {} == {};",
        Measure::<FemtoSecond>::new(1.),
        Measure::<FemtoSecond>::new(1.).convert::<Second>(),
        MeasurePoint::<FemtoSecond>::new(1.),
        MeasurePoint::<FemtoSecond>::new(1.).convert::<Second>(),
    );
    println!();
}

fn print_all_torque_units() {
    println!("* Torque units");
    println!(
        "  NewtonMetre: {}, {}, {}, {}, {}, {};",
        Measure::<NewtonMetre>::new(1.),
        MeasurePoint::<NewtonMetre>::new(1.),
        Measure2d::<NewtonMetre>::new(1., 2.),
        MeasurePoint2d::<NewtonMetre>::new(1., 2.),
        Measure3d::<NewtonMetre>::new(1., 2., 3.),
        MeasurePoint3d::<NewtonMetre>::new(1., 2., 3.),
    );
    println!();
}

fn print_all_velocity_units() {
    println!("* Velocity units");
    println!(
        "  MetrePerSecond: {}, {}, {}, {}, {}, {};",
        Measure::<MetrePerSecond>::new(1.),
        MeasurePoint::<MetrePerSecond>::new(1.),
        Measure2d::<MetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<MetrePerSecond>::new(1., 2.),
        Measure3d::<MetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<MetrePerSecond>::new(1., 2., 3.),
    );
    println!(
        "  Knot: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Knot>::new(1.),
        Measure::<Knot>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<Knot>::new(1.),
        MeasurePoint::<Knot>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<Knot>::new(1., 2.),
        Measure2d::<Knot>::new(1., 2.).convert::<MetrePerSecond>(),
        MeasurePoint2d::<Knot>::new(1., 2.),
        MeasurePoint2d::<Knot>::new(1., 2.).convert::<MetrePerSecond>(),
        Measure3d::<Knot>::new(1., 2., 3.),
        Measure3d::<Knot>::new(1., 2., 3.).convert::<MetrePerSecond>(),
        MeasurePoint3d::<Knot>::new(1., 2., 3.),
        MeasurePoint3d::<Knot>::new(1., 2., 3.).convert::<MetrePerSecond>(),
    );
    println!(
        "  KiloMetrePerHour: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KiloMetrePerHour>::new(1.),
        Measure::<KiloMetrePerHour>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<KiloMetrePerHour>::new(1.),
        MeasurePoint::<KiloMetrePerHour>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<KiloMetrePerHour>::new(1., 2.),
        Measure2d::<KiloMetrePerHour>::new(1., 2.).convert::<MetrePerSecond>(),
        MeasurePoint2d::<KiloMetrePerHour>::new(1., 2.),
        MeasurePoint2d::<KiloMetrePerHour>::new(1., 2.).convert::<MetrePerSecond>(),
        Measure3d::<KiloMetrePerHour>::new(1., 2., 3.),
        Measure3d::<KiloMetrePerHour>::new(1., 2., 3.).convert::<MetrePerSecond>(),
        MeasurePoint3d::<KiloMetrePerHour>::new(1., 2., 3.),
        MeasurePoint3d::<KiloMetrePerHour>::new(1., 2., 3.).convert::<MetrePerSecond>(),
    );
    println!(
        "  MilePerHour: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<MilePerHour>::new(1.),
        Measure::<MilePerHour>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<MilePerHour>::new(1.),
        MeasurePoint::<MilePerHour>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<MilePerHour>::new(1., 2.),
        Measure2d::<MilePerHour>::new(1., 2.).convert::<MetrePerSecond>(),
        MeasurePoint2d::<MilePerHour>::new(1., 2.),
        MeasurePoint2d::<MilePerHour>::new(1., 2.).convert::<MetrePerSecond>(),
        Measure3d::<MilePerHour>::new(1., 2., 3.),
        Measure3d::<MilePerHour>::new(1., 2., 3.).convert::<MetrePerSecond>(),
        MeasurePoint3d::<MilePerHour>::new(1., 2., 3.),
        MeasurePoint3d::<MilePerHour>::new(1., 2., 3.).convert::<MetrePerSecond>(),
    );
    println!(
        "  CentiMetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<CentiMetrePerSecond>::new(1.),
        Measure::<CentiMetrePerSecond>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<CentiMetrePerSecond>::new(1.),
        MeasurePoint::<CentiMetrePerSecond>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<CentiMetrePerSecond>::new(1., 2.),
        Measure2d::<CentiMetrePerSecond>::new(1., 2.).convert::<MetrePerSecond>(),
        MeasurePoint2d::<CentiMetrePerSecond>::new(1., 2.),
        MeasurePoint2d::<CentiMetrePerSecond>::new(1., 2.).convert::<MetrePerSecond>(),
        Measure3d::<CentiMetrePerSecond>::new(1., 2., 3.),
        Measure3d::<CentiMetrePerSecond>::new(1., 2., 3.).convert::<MetrePerSecond>(),
        MeasurePoint3d::<CentiMetrePerSecond>::new(1., 2., 3.),
        MeasurePoint3d::<CentiMetrePerSecond>::new(1., 2., 3.).convert::<MetrePerSecond>(),
    );
    println!();
}

fn print_all_volume_units() {
    println!("* Volume units");
    println!(
        "  CubicMetre: {}, {};",
        Measure::<CubicMetre>::new(1.),
        MeasurePoint::<CubicMetre>::new(1.),
    );
    println!(
        "  CubicKiloMetre: {} == {}, {} == {};",
        Measure::<CubicKiloMetre>::new(1.),
        Measure::<CubicKiloMetre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicKiloMetre>::new(1.),
        MeasurePoint::<CubicKiloMetre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicInch: {} == {}, {} == {};",
        Measure::<CubicInch>::new(1.),
        Measure::<CubicInch>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicInch>::new(1.),
        MeasurePoint::<CubicInch>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicFoot: {} == {}, {} == {};",
        Measure::<CubicFoot>::new(1.),
        Measure::<CubicFoot>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicFoot>::new(1.),
        MeasurePoint::<CubicFoot>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicYard: {} == {}, {} == {};",
        Measure::<CubicYard>::new(1.),
        Measure::<CubicYard>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicYard>::new(1.),
        MeasurePoint::<CubicYard>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicMile: {} == {}, {} == {};",
        Measure::<CubicMile>::new(1.),
        Measure::<CubicMile>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicMile>::new(1.),
        MeasurePoint::<CubicMile>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Litre: {} == {}, {} == {};",
        Measure::<Litre>::new(1.),
        Measure::<Litre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Litre>::new(1.),
        MeasurePoint::<Litre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  MilliLitre: {} == {}, {} == {};",
        Measure::<MilliLitre>::new(1.),
        Measure::<MilliLitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<MilliLitre>::new(1.),
        MeasurePoint::<MilliLitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  MicroLitre: {} == {}, {} == {};",
        Measure::<MicroLitre>::new(1.),
        Measure::<MicroLitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<MicroLitre>::new(1.),
        MeasurePoint::<MicroLitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  NanoLitre: {} == {}, {} == {};",
        Measure::<NanoLitre>::new(1.),
        Measure::<NanoLitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<NanoLitre>::new(1.),
        MeasurePoint::<NanoLitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  PicoLitre: {} == {}, {} == {};",
        Measure::<PicoLitre>::new(1.),
        Measure::<PicoLitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<PicoLitre>::new(1.),
        MeasurePoint::<PicoLitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Pint: {} == {}, {} == {};",
        Measure::<Pint>::new(1.),
        Measure::<Pint>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Pint>::new(1.),
        MeasurePoint::<Pint>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Gallon: {} == {}, {} == {};",
        Measure::<Gallon>::new(1.),
        Measure::<Gallon>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Gallon>::new(1.),
        MeasurePoint::<Gallon>::new(1.).convert::<CubicMetre>(),
    );
    println!();
}

fn print_all_volumetric_flow_rate_units() {
    println!("* VolumetricFlowRate units");
    println!(
        "  CubicMetrePerSecond: {}, {};",
        Measure::<CubicMetrePerSecond>::new(1.),
        MeasurePoint::<CubicMetrePerSecond>::new(1.),
    );
    println!(
        "  CubicCentiMetrePerSecond: {} == {}, {} == {};",
        Measure::<MilliLitrePerSecond>::new(1.),
        Measure::<MilliLitrePerSecond>::new(1.).convert::<CubicMetrePerSecond>(),
        MeasurePoint::<MilliLitrePerSecond>::new(1.),
        MeasurePoint::<MilliLitrePerSecond>::new(1.).convert::<CubicMetrePerSecond>(),
    );
    println!();
}

fn print_all_wave_number_units() {
    println!("* WaveNumber units");
    println!(
        "  CyclePerMetre: {}, {};",
        Measure::<CyclePerMetre>::new(1.),
        MeasurePoint::<CyclePerMetre>::new(1.),
    );
    println!(
        "  RadianPerMetre: {} == {}, {} == {};",
        Measure::<RadianPerMetre>::new(1.),
        Measure::<RadianPerMetre>::new(1.).convert::<CyclePerMetre>(),
        MeasurePoint::<RadianPerMetre>::new(1.),
        MeasurePoint::<RadianPerMetre>::new(1.).convert::<CyclePerMetre>(),
    );
    println!();
}

fn print_all_units() {
    print_all_acceleration_units();
    print_all_action_units();
    print_all_amount_units();
    print_all_angle_units();
    print_all_angular_acceleration_units();
    print_all_angular_momentum_units();
    print_all_area_units();
    print_all_area_density_units();
    print_all_capacitance_units();
    print_all_catalytic_activity_units();
    print_all_chemical_potential_units();
    print_all_current_density_units();
    print_all_dimensionless_units();
    print_all_dose_equivalent_units();
    print_all_dynamic_viscosity_units();
    print_all_electrical_conductance_units();
    print_all_electrical_conductivity_units();
    print_all_electrical_resistance_units();
    print_all_electrical_resistivity_units();
    print_all_electric_charge_units();
    print_all_electric_charge_density_units();
    print_all_electric_current_units();
    print_all_electric_displacement_units();
    print_all_electric_field_strength_units();
    print_all_electric_potential_units();
    print_all_energy_units();
    print_all_energy_density_units();
    print_all_entropy_units();
    print_all_force_units();
    print_all_frequency_units();
    print_all_illuminance_units();
    print_all_inductance_units();
    print_all_information_units();
    print_all_information_rate_units();
    print_all_irradiance_units();
    print_all_kinematic_viscosity_units();
    print_all_length_units();
    print_all_linear_density_units();
    print_all_linear_electric_charge_density_units();
    print_all_luminance_units();
    print_all_luminous_flux_units();
    print_all_luminous_intensity_units();
    print_all_magnetic_field_strength_units();
    print_all_magnetic_flux_units();
    print_all_magnetic_flux_density_units();
    print_all_magnetic_permeability_units();
    print_all_magnetic_reluctance_units();
    print_all_mass_units();
    print_all_mass_density_units();
    print_all_mass_flow_rate_units();
    print_all_molar_concentration_units();
    print_all_molar_heat_capacity_units();
    print_all_moment_of_inertia_units();
    print_all_momentum_units();
    print_all_permittivity_units();
    print_all_power_units();
    print_all_pressure_units();
    print_all_radiance_units();
    print_all_radiant_intensity_units();
    print_all_radioactive_activity_units();
    print_all_radioactive_dose_units();
    print_all_radioactive_dose_rate_units();
    print_all_reaction_rate_units();
    print_all_solid_angle_units();
    print_all_specific_energy_units();
    print_all_specific_heat_capacity_units();
    print_all_specific_volume_units();
    print_all_square_time_units();
    print_all_surface_density_units();
    print_all_surface_tension_units();
    print_all_temperature_units();
    print_all_thermal_conductivity_units();
    print_all_time_units();
    print_all_torque_units();
    print_all_velocity_units();
    print_all_volume_units();
    print_all_volumetric_flow_rate_units();
    print_all_wave_number_units();
}

fn print_all_single_unit_operations_for_measure_1d() {
    println!("* Single unit operations for measure 1d");
    {
        let m1 = Measure::<KiloMetrePerHour>::new(12.);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(1.234_567_890_123_456_7);
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure::<KiloMetrePerHour, f32>::new(1.234_567_9);
        println!(
            "{m1} can be lossless-converted to {m2}.",
            m2 = m1.lossless_into::<f64>(),
        );
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(-12.);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm(),);
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(-12.);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(12.);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure::<KiloMetrePerHour>::new(12.);
        let m2 = Measure::<KiloMetrePerHour>::new(13.);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure::<KiloMetrePerHour>::new(12.);
        let m2 = Measure::<KiloMetrePerHour>::new(13.);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure::<KiloMetrePerHour>::new(12.);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(12.);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure::<KiloMetrePerHour>::new(12.);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(12.);
        let m2 = Measure::<KiloMetrePerHour>::new(4.);
        println!("{m1} divided by {m2} is {m3}.", m3 = m1 / m2);
    }
    {
        let m1 = Measure::<KiloMetrePerHour>::new(12.);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
        println!("{m1} < {m1} is {result}.", result = m1 < m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_1d() {
    println!("* Single unit operations for measure point 1d");
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<Fahrenheit>()
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(1.234_567_890_123_456_7);
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius, f32>::new(1.234_567_9);
        println!(
            "{mp1} can be lossless-converted to {mp2}.",
            mp2 = mp1.lossless_into::<f64>(),
        );
    }
    {
        let mut mp1 = MeasurePoint::<Celsius>::new(12.);
        let m2 = Measure::<Celsius>::new(13.);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint::<Celsius>::new(12.);
        let m2 = Measure::<Celsius>::new(13.);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        let mp2 = MeasurePoint::<Celsius>::new(13.);
        println!("{mp1} minus {mp2} is {m3}.", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(10.);
        let mp2 = MeasurePoint::<Celsius>::new(20.);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(10.);
        let mp2 = MeasurePoint::<Celsius>::new(20.);
        let mp3 = MeasurePoint::<Celsius>::new(40.);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
        println!("{mp1} < {mp1} is {result}.", result = mp1 < mp2);
    }
    println!();
}

fn print_all_single_unit_operations_for_unsigned_directions() {
    println!("* Single unit operations for unsigned directions");
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{ud2} can be created from {mp1}.",
            ud2 = UnsignedDirection::<Degree>::from_measure_point(mp1)
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {mp2}.",
            mp2 = ud1.to_measure_point()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {sd2}.",
            sd2 = ud1.to_signed_direction()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {ud2}.",
            ud2 = ud1.convert::<Radian>()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(1.234_567_890_123_456_7);
        println!(
            "{ud1} can be lossy-converted to {ud2}.",
            ud2 = ud1.lossy_into::<f32>()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree, f32>::new(1.234_567_9);
        println!(
            "{ud1} can be lossless-converted to {ud2}.",
            ud2 = ud1.lossless_into::<f64>(),
        );
    }
    {
        let mut ud1 = UnsignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{ud1} plus {m2} is {ud3},", ud3 = ud1 + m2);

        ud1 += m2;
        println!(" and if incremented by {m2}, it becomes {ud1}.");
    }
    {
        let mut ud1 = UnsignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{ud1} minus {m2} is {ud3},", ud3 = ud1 - m2);

        ud1 -= m2;
        println!(" and if decremented by {m2}, it becomes {ud1}.");
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        let ud2 = UnsignedDirection::<Degree>::new(13.);
        println!("{ud1} minus {ud2} is {m3}.", m3 = ud1 - ud2);
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        let ud2 = ud1;
        println!("{ud1} == {ud1} is {result}.", result = ud1 == ud2);
        println!("{ud1} < {ud1} is {result}.", result = ud1 < ud2);
    }
    println!();
}

fn print_all_single_unit_operations_for_signed_directions() {
    println!("* Single unit operations for signed directions");
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{sd2} can be created from {mp1}.",
            sd2 = SignedDirection::<Degree>::from_measure_point(mp1)
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {mp2}.",
            mp2 = sd1.to_measure_point()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {ud2}.",
            ud2 = sd1.to_unsigned_direction()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {sd2}.",
            sd2 = sd1.convert::<Radian>()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(1.234_567_890_123_456_7);
        println!(
            "{sd1} can be lossy-converted to {sd2}.",
            sd2 = sd1.lossy_into::<f32>()
        );
    }
    {
        let sd1 = SignedDirection::<Degree, f32>::new(1.234_567_9);
        println!(
            "{sd1} can be lossless-converted to {sd2}.",
            sd2 = sd1.lossless_into::<f64>(),
        );
    }
    {
        let mut sd1 = SignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{sd1} plus {m2} is {sd3},", sd3 = sd1 + m2);

        sd1 += m2;
        println!(" and if incremented by {m2}, it becomes {sd1}.");
    }
    {
        let mut sd1 = SignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{sd1} minus {m2} is {sd3},", sd3 = sd1 - m2);

        sd1 -= m2;
        println!(" and if decremented by {m2}, it becomes {sd1}.");
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        let sd2 = SignedDirection::<Degree>::new(13.);
        println!("{sd1} minus {sd2} is {m3}.", m3 = sd1 - sd2);
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        let sd2 = sd1;
        println!("{sd1} == {sd1} is {result}.", result = sd1 == sd2);
        println!("{sd1} < {sd1} is {result}.", result = sd1 < sd2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_2d() {
    println!("* Single unit operations for measure 2d");
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{m1} has components X={m2} and Y={m3}.",
            m2 = m1.x(),
            m3 = m1.y(),
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(1.234_567_890_123_456_7, 2.345_678_901_234_568);
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour, f32>::new(1.234_567_9, 2.345_678_8);
        println!(
            "{m1} can be lossless-converted to {m2}.",
            m2 = m1.lossless_into::<f64>(),
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(-12., -13.);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm());
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(-12., -13.);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{m1} can be created from angle {mp1}.",
            m1 = Measure2d::<KiloMetrePerHour>::from_direction(mp1)
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{m1} has signed direction {sd2}.",
            sd2 = m1.signed_direction::<Degree>()
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{m1} has unsigned direction {ud2}.",
            ud2 = m1.unsigned_direction::<Degree>()
        );
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., -13.);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let m2 = Measure2d::<KiloMetrePerHour>::new(15., 19.);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let m2 = Measure2d::<KiloMetrePerHour>::new(15., 19.);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure2d::<KiloMetrePerHour>::new(12., 13.);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_2d() {
    println!("* Single unit operations for measure point 2d");
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{mp1} has components X={mp2} and Y={mp3}.",
            mp2 = mp1.x(),
            mp3 = mp1.y(),
        );
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<MilePerHour>()
        );
    }
    {
        let mp1 =
            MeasurePoint2d::<KiloMetrePerHour>::new(1.234_567_890_123_456_7, 2.345_678_901_234_568);
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour, f32>::new(1.234_567_9, 2.345_678_8);
        println!(
            "{mp1} can be lossless-converted to {mp2}.",
            mp2 = mp1.lossless_into::<f64>(),
        );
    }
    {
        let mut mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        let m2 = Measure2d::<KiloMetrePerHour>::new(15., 19.);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        let m2 = Measure2d::<KiloMetrePerHour>::new(15., 19.);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        let mp2 = MeasurePoint2d::<KiloMetrePerHour>::new(15., 19.);
        println!("{mp1} minus {mp2} is {m3},", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(10., -100.);
        let mp2 = MeasurePoint2d::<KiloMetrePerHour>::new(20., -200.);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint_2d(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint_2d(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(10., -100.);
        let mp2 = MeasurePoint2d::<KiloMetrePerHour>::new(20., -200.);
        let mp3 = MeasurePoint2d::<KiloMetrePerHour>::new(40., -400.);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination_2d(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint2d::<KiloMetrePerHour>::new(12., 13.);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_3d() {
    println!("* Single unit operations for measure 3d");
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        println!(
            "{m1} has components X={m2}, Y={m3}, and Z={m4}.",
            m2 = m1.x(),
            m3 = m1.y(),
            m4 = m1.z(),
        );
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(
            1.234_567_890_123_456_7,
            2.345_678_901_234_568,
            3.456_789_012_345_679,
        );
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour, f32>::new(1.234_567_9, 2.345_678_8, 3.456_789);
        println!(
            "{m1} can be lossless-converted to {m2}.",
            m2 = m1.lossless_into::<f64>(),
        );
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(-12., -13., -14.);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm());
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(-12., -13., -14.);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(12., -13., -14.);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let m2 = Measure3d::<KiloMetrePerHour>::new(15., 19., 26.);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let m2 = Measure3d::<KiloMetrePerHour>::new(15., 19., 26.);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_3d() {
    println!("* Single unit operations for measure point 3d");
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        println!(
            "{mp1} has components X={mp2} and Y={mp3}.",
            mp2 = mp1.x(),
            mp3 = mp1.y(),
        );
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<MilePerHour>()
        );
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(
            1.234_567_890_123_456_7,
            2.345_678_901_234_568,
            3.456_789_012_345_679,
        );
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour, f32>::new(1.234_567_9, 2.345_678_8, 3.456_789);
        println!(
            "{mp1} can be lossless-converted to {mp2}.",
            mp2 = mp1.lossless_into::<f64>(),
        );
    }
    {
        let mut mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let m2 = Measure3d::<KiloMetrePerHour>::new(15., 19., 26.);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let m2 = Measure3d::<KiloMetrePerHour>::new(15., 19., 26.);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let mp2 = MeasurePoint3d::<KiloMetrePerHour>::new(15., 19., 26.);
        println!("{mp1} minus {mp2} is {m3},", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(10., -100., 40.);
        let mp2 = MeasurePoint3d::<KiloMetrePerHour>::new(20., -200., 80.);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint_3d(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint_3d(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(10., -100., 40.);
        let mp2 = MeasurePoint3d::<KiloMetrePerHour>::new(20., -200., 80.);
        let mp3 = MeasurePoint3d::<KiloMetrePerHour>::new(40., -400., 160.);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination_3d(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint3d::<KiloMetrePerHour>::new(12., 13., 14.);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
    }
    println!();
}

fn print_all_single_unit_operations() {
    print_all_single_unit_operations_for_measure_1d();
    print_all_single_unit_operations_for_measure_point_1d();
    print_all_single_unit_operations_for_unsigned_directions();
    print_all_single_unit_operations_for_signed_directions();
    print_all_single_unit_operations_for_measure_2d();
    print_all_single_unit_operations_for_measure_point_2d();
    print_all_single_unit_operations_for_measure_3d();
    print_all_single_unit_operations_for_measure_point_3d();
}

fn print_all_computer_science_mixed_operations() {
    println!("* Computer science mixed operations");
    {
        let information = Measure::<Bit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<BitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Byte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<BytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<KiloBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KiloBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<KiloByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KiloBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<KibiBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KibiBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<KibiByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KibiBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<MegaBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MegaBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<MegaByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MegaBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}")
    }
    {
        let information = Measure::<MebiBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MebiBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<MebiByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MebiBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<GigaBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GigaBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<GigaByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GigaBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<GibiBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GibiBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<GibiByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GibiBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<TeraBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TeraBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<TeraByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TeraBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<TebiBit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TebiBitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<TebiByte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TebiBytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    println!();
}

fn print_all_geometry_mixed_operations() {
    println!("* Geometry mixed operations");
    {
        let length = Measure::<Metre>::new(6.);
        let width = Measure::<Metre>::new(2.);
        let area: Measure<SquareMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<KiloMetre>::new(6.);
        let width = Measure::<KiloMetre>::new(2.);
        let area: Measure<SquareKiloMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<HectoMetre>::new(6.);
        let width = Measure::<HectoMetre>::new(2.);
        let area: Measure<Hectare> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<DecaMetre>::new(6.);
        let width = Measure::<DecaMetre>::new(2.);
        let area: Measure<Are> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<DeciMetre>::new(6.);
        let width = Measure::<DeciMetre>::new(2.);
        let area: Measure<SquareDeciMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<CentiMetre>::new(6.);
        let width = Measure::<CentiMetre>::new(2.);
        let area: Measure<SquareCentiMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<MilliMetre>::new(6.);
        let width = Measure::<MilliMetre>::new(2.);
        let area: Measure<SquareMilliMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<MicroMetre>::new(6.);
        let width = Measure::<MicroMetre>::new(2.);
        let area: Measure<SquareMicroMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<NanoMetre>::new(6.);
        let width = Measure::<NanoMetre>::new(2.);
        let area: Measure<SquareNanoMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Inch>::new(6.);
        let width = Measure::<Inch>::new(2.);
        let area: Measure<SquareInch> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Foot>::new(6.);
        let width = Measure::<Foot>::new(2.);
        let area: Measure<SquareFoot> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Yard>::new(6.);
        let width = Measure::<Yard>::new(2.);
        let area: Measure<SquareYard> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Mile>::new(6.);
        let width = Measure::<Mile>::new(2.);
        let area: Measure<SquareMile> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let base = Measure::<SquareMetre>::new(6.);
        let height = Measure::<Metre>::new(2.);
        let volume: Measure<CubicMetre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareKiloMetre>::new(6.);
        let height = Measure::<KiloMetre>::new(2.);
        let volume: Measure<CubicKiloMetre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareDeciMetre>::new(6.);
        let height = Measure::<DeciMetre>::new(2.);
        let volume: Measure<Litre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareCentiMetre>::new(6.);
        let height = Measure::<CentiMetre>::new(2.);
        let volume: Measure<MilliLitre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMilliMetre>::new(6.);
        let height = Measure::<MilliMetre>::new(2.);
        let volume: Measure<MicroLitre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMicroMetre>::new(6.);
        let height = Measure::<MicroMetre>::new(2.);
        let volume: Measure<CubicMicroMetre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareNanoMetre>::new(6.);
        let height = Measure::<NanoMetre>::new(2.);
        let volume: Measure<CubicNanoMetre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareInch>::new(6.);
        let height = Measure::<Inch>::new(2.);
        let volume: Measure<CubicInch> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareFoot>::new(6.);
        let height = Measure::<Foot>::new(2.);
        let volume: Measure<CubicFoot> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareYard>::new(6.);
        let height = Measure::<Yard>::new(2.);
        let volume: Measure<CubicYard> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMile>::new(6.);
        let height = Measure::<Mile>::new(2.);
        let volume: Measure<CubicMile> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }

    {
        let phase = Measure::<Cycle>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let wave_number: Measure<CyclePerMetre> = phase / length;
        println!("A portion of wave having a phase of {phase} for a length of {length} has a wave number of {wave_number}.")
    }
    {
        let phase = Measure::<Radian>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let wave_number: Measure<RadianPerMetre> = phase / length;
        println!("A portion of wave having a phase of {phase} for a length of {length} has a wave number of {wave_number}.")
    }
    println!();
}

fn print_all_kinematics_mixed_operations() {
    println!("* Kinematics mixed operations");
    {
        let velocity = Measure::<MetrePerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<MetrePerSquareSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<MetrePerSecond>::new(6., 20.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<MetrePerSquareSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<MetrePerSecond>::new(6., 20., 36.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<MetrePerSquareSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure::<CentiMetrePerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<CentiMetrePerSquareSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<CentiMetrePerSecond>::new(6., 20.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<CentiMetrePerSquareSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<CentiMetrePerSecond>::new(6., 20., 36.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<CentiMetrePerSquareSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure::<KiloMetrePerHour>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<KiloMetrePerHourPerSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<KiloMetrePerHour>::new(6., 20.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<KiloMetrePerHourPerSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<KiloMetrePerHour>::new(6., 20., 36.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<KiloMetrePerHourPerSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let length = Measure::<Metre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure<MetrePerSecond> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Metre>::new(6., 20.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure2d<MetrePerSecond> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Metre>::new(6., 20., 36.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure3d<MetrePerSecond> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<NauticalMile>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<Knot> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<NauticalMile>::new(6., 20.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<Knot> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<NauticalMile>::new(6., 20., 36.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<Knot> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<KiloMetre>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<KiloMetrePerHour> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<KiloMetre>::new(6., 20.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<KiloMetrePerHour> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<KiloMetre>::new(6., 20., 36.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<KiloMetrePerHour> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<Mile>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<MilePerHour> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Mile>::new(6., 20.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<MilePerHour> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Mile>::new(6., 20., 36.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<MilePerHour> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<CentiMetre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure<CentiMetrePerSecond> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<CentiMetre>::new(6., 20.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure2d<CentiMetrePerSecond> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<CentiMetre>::new(6., 20., 36.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure3d<CentiMetrePerSecond> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let angular_velocity = Measure::<RadianPerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_acceleration: Measure<RadianPerSquareSecond> = angular_velocity / time;
        println!("If an object changes its angular velocity by {angular_velocity} in a time interval of {time}, it has an average angular acceleration of {angular_acceleration}.")
    }
    {
        let angular_position = Measure::<Cycle>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_velocity: Measure<Hertz> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    {
        let angular_position = Measure::<Radian>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_velocity: Measure<RadianPerSecond> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    {
        let angular_position = Measure::<Cycle>::new(6.);
        let time = Measure::<Minute>::new(2.);
        let angular_velocity: Measure<CyclePerMinute> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    // TODO KinematicViscosity
    // TODO SquareTime
    {
        let volume = Measure::<CubicMetre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let flow_rate: Measure<CubicMetrePerSecond> = volume / time;
        println!("If some fluid having a volume of {volume} crosses a surface in a time interval of {time}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let volume = Measure::<MilliLitre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let flow_rate: Measure<MilliLitrePerSecond> = volume / time;
        println!("If some fluid having a volume of {volume} crosses a surface in a time interval of {time}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let area = Measure::<SquareMetre>::new(6.);
        let velocity = Measure::<MetrePerSecond>::new(2.);
        let flow_rate: Measure<CubicMetrePerSecond> = area * velocity;
        println!("If some fluid crosses a surface with area {area} having a velocity of {velocity}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let area = Measure::<SquareCentiMetre>::new(6.);
        let velocity = Measure::<CentiMetrePerSecond>::new(2.);
        let flow_rate: Measure<MilliLitrePerSecond> = area * velocity;
        println!("If some fluid crosses a surface with area {area} having a velocity of {velocity}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    println!();
}

fn print_all_dynamics_mixed_operations() {
    println!("* Dynamics mixed operations");
    // TODO Action == Energy * Time
    // TODO Action == Power * SquareTime
    {
        let impulse = Measure2d::<KiloGramMetrePerSecond>::new(6., 2.);
        let arm = Measure2d::<Metre>::new(1., 4.);
        let angular_momentum: Measure<KiloGramSquareMetrePerSecond> = impulse.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives an impulse of {impulse} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a variation of the angular momentum by {angular_momentum}.")
    }
    {
        let impulse = Measure3d::<KiloGramMetrePerSecond>::new(6., 2., -3.);
        let arm = Measure3d::<Metre>::new(1., 4., 6.);
        let angular_momentum: Measure3d<KiloGramSquareMetrePerSecond> = impulse.cross_product(arm);
        println!("If some object constrained to rotate around a point receives an impulse of {impulse}, applied at a position from the rotation point of {arm}, that causes a variation of the angular momentum by {angular_momentum}.")
    }
    // TODO AngularMomentum == MomentOfInertia / Time
    // TODO DynamicViscosity == Pressure * Time
    {
        let force = Measure::<Newton>::new(6.);
        let movement = Measure::<Metre>::new(2.);
        let work: Measure<Joule> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in the same direction by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure2d::<Newton>::new(6., -3.);
        let movement = Measure2d::<Metre>::new(2., 5.);
        let work: Measure<Joule> = force * movement;
        println!("If a planar force of {force} is applied to an object while it moves in the same plane by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure3d::<Newton>::new(6., -3., 5.);
        let movement = Measure3d::<Metre>::new(2., 5., 7.);
        let work: Measure<Joule> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in space by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure::<Dyne>::new(6.);
        let movement = Measure::<CentiMetre>::new(2.);
        let work: Measure<Erg> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in the same direction by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure2d::<Dyne>::new(6., -3.);
        let movement = Measure2d::<CentiMetre>::new(2., 5.);
        let work: Measure<Erg> = force * movement;
        println!("If a planar force of {force} is applied to an object while it moves in the same plane by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure3d::<Dyne>::new(6., -3., 5.);
        let movement = Measure3d::<CentiMetre>::new(2., 5., 7.);
        let work: Measure<Erg> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in space by {movement}, that force gives to that object an energy of {work}.")
    }
    // TODO Energy == Momentum * Velocity
    // TODO Energy == MomentOfInertia / SquareTime
    // EnergyDensity == Energy / Volume
    {
        let energy = Measure::<Joule>::new(6.);
        let volume = Measure::<CubicMetre>::new(2.);
        let energy_density: Measure<JoulePerCubicMetre> = energy / volume;
        println!("If an energy of {energy} is contained in a volume of {volume}, it has an average energy density of {energy_density}.")
    }

    {
        let force = Measure::<Newton>::new(6.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<Newton>::new(6., -3.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure2d<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<Newton>::new(6., -3., 5.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure3d<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }
    {
        let force = Measure::<Dyne>::new(6.);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure<CentiMetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<Dyne>::new(6., -3.);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure2d<CentiMetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<Dyne>::new(6., -3., 5.);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure3d<CentiMetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }
    {
        let force = Measure::<KiloGramForce>::new(6.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure<GForce> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<KiloGramForce>::new(6., -3.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure2d<GForce> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<KiloGramForce>::new(6., -3., 5.);
        let mass = Measure::<KiloGram>::new(2.);
        let acceleration: Measure3d<GForce> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }

    {
        let mass = Measure::<KiloGram>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let linear_density: Measure<KiloGramPerMetre> = mass / length;
        println!("If a mass of {mass} is distributed along a length of {length}, that mass has a linear density by {linear_density}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let length = Measure::<CentiMetre>::new(2.);
        let linear_density: Measure<GramPerCentiMetre> = mass / length;
        println!("If a mass of {mass} is distributed along a length of {length}, that mass has a linear density by {linear_density}.")
    }
    {
        let mass = Measure::<KiloGram>::new(6.);
        let volume = Measure::<CubicMetre>::new(2.);
        let space_density: Measure<KiloGramPerCubicMetre> = mass / volume;
        println!("If a mass of {mass} occupies a volume of {volume}, it has a density by {space_density}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let volume = Measure::<MilliLitre>::new(2.);
        let space_density: Measure<GramPerMilliLitre> = mass / volume;
        println!("If a mass of {mass} occupies a volume of {volume}, it has a density by {space_density}.")
    }
    {
        let mass = Measure::<KiloGram>::new(6.);
        let time = Measure::<Second>::new(2.);
        let mass_flow_rate: Measure<KiloGramPerSecond> = mass / time;
        println!("If a mass of {mass} crosses a surface in a time interval of {time}, it has a mass flow rate of {mass_flow_rate}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let time = Measure::<Second>::new(2.);
        let mass_flow_rate: Measure<GramPerSecond> = mass / time;
        println!("If a mass of {mass} crosses a surface in a time interval of {time}, it has a mass flow rate of {mass_flow_rate}.")
    }
    // TODO MomentOfInertia == Mass * Area
    // TODO Momentum == Force * Time
    {
        let mass = Measure::<KiloGram>::new(6.);
        let velocity = Measure::<MetrePerSecond>::new(2.);
        let momentum: Measure<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a line of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<KiloGram>::new(6.);
        let velocity = Measure2d::<MetrePerSecond>::new(2., 5.);
        let momentum: Measure2d<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a plane of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<KiloGram>::new(6.);
        let velocity = Measure3d::<MetrePerSecond>::new(2., 5., -3.);
        let momentum: Measure3d<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in space of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure::<CentiMetrePerSecond>::new(2.);
        let momentum: Measure<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a line of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure2d::<CentiMetrePerSecond>::new(2., 5.);
        let momentum: Measure2d<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a plane of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure3d::<CentiMetrePerSecond>::new(2., 5., -3.);
        let momentum: Measure3d<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in space of {velocity} has a momentum of {momentum}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let time = Measure::<Second>::new(2.);
        let power: Measure<Watt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<WattHour>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let power: Measure<Watt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<KiloWattHour>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let power: Measure<KiloWatt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<Erg>::new(6.);
        let time = Measure::<Second>::new(2.);
        let power: Measure<ErgPerSecond> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let force = Measure::<Newton>::new(6.);
        let area = Measure::<SquareMetre>::new(2.);
        let pressure: Measure<Pascal> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let force = Measure::<PoundForce>::new(6.);
        let area = Measure::<SquareInch>::new(2.);
        let pressure: Measure<PoundForcePerSquareInch> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let force = Measure::<Newton>::new(6.);
        let area = Measure::<SquareDeciMetre>::new(2.);
        let pressure: Measure<HectoPascal> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let mass = Measure::<KiloGram>::new(2.);
        let specific_energy: Measure<JoulePerKiloGram> = energy / mass;
        println!("An energy of {energy} contained in a mass of {mass} has a specific energy of {specific_energy}.")
    }
    {
        let volume = Measure::<CubicMetre>::new(6.);
        let mass = Measure::<KiloGram>::new(2.);
        let specific_volume: Measure<CubicMetrePerKiloGram> = volume / mass;
        println!("A volume of {volume} containing a mass of {mass} has a specific volume of {specific_volume}.")
    }
    {
        let mass_density = Measure::<KiloGramPerCubicMetre>::new(2.);
        let specific_volume: Measure<CubicMetrePerKiloGram> = 1. / mass_density;
        println!("A mass density of {mass_density} is equivalent to a specific volume of {specific_volume}.")
    }
    {
        let mass = Measure::<KiloGram>::new(6.);
        let area = Measure::<SquareMetre>::new(6.);
        let surface_density: Measure<KiloGramPerSquareMetre> = mass / area;
        println!("A mass of {mass} distributed over an area of {area} has a surface density of {surface_density}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let area = Measure::<SquareMetre>::new(6.);
        let surface_tension: Measure<JoulePerSquareMetre> = energy / area;
        println!("An energy of {energy} distributed over an area of {area} has a surface tension of {surface_tension}.")
    }
    {
        let force = Measure2d::<Newton>::new(6., 2.);
        let arm = Measure2d::<Metre>::new(1., 4.);
        let torque: Measure<NewtonMetre> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<Newton>::new(6., 2., -3.);
        let arm = Measure3d::<Metre>::new(1., 4., 6.);
        let torque: Measure3d<NewtonMetre> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }

    {
        let force = Measure2d::<PoundForce>::new(6., 2.);
        let arm = Measure2d::<Foot>::new(1., 4.);
        let torque: Measure<PoundFoot> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<PoundForce>::new(6., 2., -3.);
        let arm = Measure3d::<Foot>::new(1., 4., 6.);
        let torque: Measure3d<PoundFoot> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }

    {
        let force = Measure2d::<PoundForce>::new(6., 2.);
        let arm = Measure2d::<Inch>::new(1., 4.);
        let torque: Measure<PoundInch> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<PoundForce>::new(6., 2., -3.);
        let arm = Measure3d::<Inch>::new(1., 4., 6.);
        let torque: Measure3d<PoundInch> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    println!();
}

fn print_all_thermodynamics_mixed_operations() {
    println!("* Thermodynamics mixed operations");
    // TODO Entropy == Energy / Temperature
    // TODO SpecificHeatCapacity == Entropy / Mass
    // TODO ThermalConductivity == Power / Length / Temperature
    println!();
}

fn print_all_chemistry_mixed_operations() {
    println!("* Chemistry mixed operations");
    //TODO
    println!();
}

fn print_all_radioactivity_mixed_operations() {
    println!("* Radioactivity mixed operations");
    //TODO
    println!();
}

fn print_all_optics_mixed_operations() {
    println!("* Optics mixed operations");
    //TODO
    println!();
}

fn print_all_electricity_mixed_operations() {
    println!("* Electricity mixed operations");
    //TODO
    println!();
}

fn print_all_magnetism_mixed_operations() {
    println!("* Magnetism mixed operations");
    //TODO
    println!();
}

fn print_all_other_mixed_operations() {
    println!("* Other mixed operations");
    //TODO
    println!();
}

fn print_all_mixed_operation() {
    print_all_computer_science_mixed_operations();
    print_all_geometry_mixed_operations();
    print_all_kinematics_mixed_operations();
    print_all_dynamics_mixed_operations();
    print_all_thermodynamics_mixed_operations();
    print_all_chemistry_mixed_operations();
    print_all_radioactivity_mixed_operations();
    print_all_optics_mixed_operations();
    print_all_electricity_mixed_operations();
    print_all_magnetism_mixed_operations();
    print_all_other_mixed_operations();
}

fn print_all_transformations() {
    // TODO
}

fn main() {
    println!("*** full ***");
    print_all_units();
    print_all_single_unit_operations();
    print_all_mixed_operation();
    print_all_transformations();
}
