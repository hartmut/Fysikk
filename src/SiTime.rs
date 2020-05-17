/// SI Based time Conversion
/// Number of seconds in a minute
pub const SECONDS_MINUTES_FACTOR: f64 = 60.0;
/// Number of minutes in a hour
pub const MINUTES_HOURS_FACTOR: f64 = 60.0;

use valuetype::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum SiTime {
    Sec,  //second, time
    Min,  // minutes, time
    Hour, //hour, time
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct Time {
    seconds: f64,
}

impl Time {
    /// Create a new Time from a floating point number of Seconds
    pub fn from_seconds(seconds: f64) -> Time {
        Time { seconds }
    }

    /// Create a new Time from a floating point number of Minutes
    pub fn from_minutes(minutes: f64) -> Time {
        Time::from_seconds(minutes * SECONDS_MINUTES_FACTOR)
    }

    /// Create a new Time from a floating point number of Hours
    pub fn from_hours(hours: f64) -> Time {
        Time::from_minutes(hours * MINUTES_HOURS_FACTOR)
    }
}

// converts to SI value of time: seconds
pub fn conv2_sibase_time(input: Variable) -> Option<f64> {
    match input {
        Variable {
            value,
            unit: Si::time(x),
        } => match x {
            SiTime::Sec => Some(value),
            _ => Some(value),
        },
        _ => None,
    }
}
