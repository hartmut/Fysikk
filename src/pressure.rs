use valuetype::*;

// partial pressure
pub fn partial_pressure(_abs_pressue: Variable, _fraction: f64) -> Variable {
    Variable {
        value: 1.0,
        unit: Si::Pa,
    }
} // partial pressure = (total absolute pressure) × (volume fraction of gas component) - https://en.wikipedia.org/wiki/Partial_pressure
