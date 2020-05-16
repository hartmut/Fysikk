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

pub enum Time {
    Sec,  //second, time
    Min,  // minutes, time
    Hour, //hour, time
    Year, //year, time
}

// derived and base si units
// based on https://en.wikipedia.org/wiki/International_System_of_Units
#[allow(non_camel_case_types)]
pub enum Si {
    time(Time), //time
    m,          //meter, length
    m_per_s,    //meter per second, speed
    kg,         //kilogram, mass
    A,          //ampere, electrict current
    K,          //kelvin, thermodynamic temprature
    mol,        //mole, amount of substance
    cd,         //candela, lumiunous intensity
    v,          //velocity (m / s)
    a,          //acceleration (m / s^2)
    Pa,         // Pascal (N / m^2 = kg/(m * s^2) ) - https://en.wikipedia.org/wiki/Pascal_(unit)
    N,          // Newton ((kg * m) / s^2 ) - https://en.wikipedia.org/wiki/Newton_(unit)
    Da,         // Dalton - https://en.wikipedia.org/wiki/Dalton_(unit)
}

pub struct Variable {
    pub value: f64,
    pub unit: Si,
}

// seconds
pub fn conv2_sibase_time(input: Variable) -> f64 {
    1.0
}

// meter per second
pub fn conv2_sibase_speed(input: Variable) -> f64 {
    1.0
}
