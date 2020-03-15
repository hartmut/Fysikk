//! definition of types and structs

// based on https://en.wikipedia.org/wiki/International_System_of_Units#Base_units
// base units
#[allow(non_camel_case_types)]
pub enum SiBase {
    s,   //second, time
    m,   //meter, length
    kg,  //kilogram, mass
    A,   //ampere, electrict current
    K,   //kelvin, thermodynamic tempreature
    mol, //mole, amount of substance
    cd,  //candela, lumiunous intensity
}

// derived and base si units
// based on https://en.wikipedia.org/wiki/International_System_of_Units
#[allow(non_camel_case_types)]
pub enum Si {
    s,   //second, time
    m,   //meter, length
    kg,  //kilogram, mass
    A,   //ampere, electrict current
    K,   //kelvin, thermodynamic tempreature
    mol, //mole, amount of substance
    cd,  //candela, lumiunous intensity
    v,   //velocity (m/s)
    a,   //acceleration (m/s^2)
}

pub struct Variable {
    value: f64,
    unit: Si,
}
