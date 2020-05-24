//! definition of types and structs
use si_time::SiTime;
// based on https://en.wikipedia.org/wiki/International_System_of_Units#Base_units
// base units
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum Si {
    time(SiTime), //time
    m,            //meter, length
    m_per_s,      //meter per second, speed
    kg,           //kilogram, mass
    A,            //ampere, electrict current
    K,            //kelvin, thermodynamic temprature
    mol,          //mole, amount of substance
    cd,           //candela, lumiunous intensity
    v,            //velocity (m / s)
    a,            //acceleration (m / s^2)
    Pa,           // Pascal (N / m^2 = kg/(m * s^2) ) - https://en.wikipedia.org/wiki/Pascal_(unit)
    N,            // Newton ((kg * m) / s^2 ) - https://en.wikipedia.org/wiki/Newton_(unit)
    Da,           // Dalton - https://en.wikipedia.org/wiki/Dalton_(unit)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct Variable {
    pub value: f64,
    pub unit: Si,
}

//  converts to SI based value of speed: meter per second
pub fn conv2_sibase_speed(_input: Variable) -> f64 {
    1.0
}
