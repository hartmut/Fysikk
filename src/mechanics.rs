//! classical mechanics
use si_time::*;
use valuetype::*;

pub fn Distance_from_speed(time: Variable, speed: Variable) -> Variable {
    let _t = conv2_sibase_time(time);
    let _s = conv2_sibase_speed(speed);
    Variable {
        value: 1.0,
        unit: Si::m_per_s,
    }
}

pub fn Distance_from_acceleration() {}
