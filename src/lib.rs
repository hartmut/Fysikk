//! library for physical simulations
//! Written by Hartmut Prochaska, Version 0.1 (2020)
//!
//! Repository: https://github.com/hartmut/Fysikk
//!
//! License: GPLv3
//!
//! # Example:
//! ```

extern crate measurements;

pub mod SiTime;
#[allow(non_snake_case)]
pub mod mechanics;
pub mod pressure;
pub mod valuetype;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
