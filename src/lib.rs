// furlong

//! A crate that provides static typing for scientific units with minimal
//! runtime impact. This is a work in progress, and actual "zero-overhead"
//! is not yet gauranteed for unit conversions during runtime. 
//! 
//! ```rust
//! extern crate furlong;
//! extern crate typenum;
//! 
//! use furlong::Qnty;
//! use furlong::system::si::{self, METERS};
//! 
//! # fn main() {
//! let length = Qnty::<si::Length>::new(2.0); // 2.0 meters (si::Length)
//! let width = 3.0 * METERS; // 3.0 meters
//! let area = length * width;
//! assert_eq!(area, 6.0 * METERS);
//! 
//! let time = Qnty::<si::Time>::new(3.0); // 3.0 seconds
//! let velocity = length / time;
//! type Velocity = typenum::Quot<si::Length, si::Time>;
//! let expected = Qnty::<Velocity>::new(2.0 / 3.0);
//! assert_eq!(velocity, expected);
//! # }
//! ```
//! 
//! The design is intended to mirror the [boost/units](http://boost.org/libs/units) library. 
//! 

extern crate typenum;

mod qnty;
pub use qnty::Qnty;
pub mod unit;
pub mod system;
pub use system::{si, imperial};
pub mod rational;

mod dimension;
mod types;
mod unit_system;

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::Qnty,
        unit::UnitInfo,
        system::si::{self, Length as MetersUnit, METERS},
        system::imperial::{self, Length as FeetUnit, FEET}
    };
    use approx::assert_abs_diff_eq;
    #[test]
    fn add_same_unit() {
        let l1 = Qnty::<MetersUnit>::new(2.0);
        let l2 = 1.5 * METERS;
        let l3 = Qnty::<MetersUnit>::new(3.5);
        assert_eq!(l1 + l2, l3);
    }

    #[test]
    fn add_int_units() {
        let l1 = 1.5 * METERS;
        let l2 = Qnty::<si::Length, i32>::new(2);
        let l3 = l1 + l2;
        assert_eq!(l3, 3.5 * METERS);
    }

    #[test]
    fn add_different_units() {
        let l1 = Qnty::<FeetUnit>::new(2.0);
        let l2 = Qnty::<MetersUnit>::new(1.0);
        let l3 = 5.28084 * FEET;
        assert_abs_diff_eq!(l1 + l2, l3, epsilon = 0.0001 * FEET);
    }

    #[test]
    fn add_complex_units() {
        let a1 = Qnty::<si::Area>::new(2.0);
        let a2 = Qnty::<imperial::Area>::new(2.0);
        let a1a2 = Qnty::<si::Area>::new(2.18581);
        let eps = Qnty::<si::Area>::new(0.001);
        assert_abs_diff_eq!(a1 + a2, a1a2, epsilon = eps );
    }

    #[test]
    fn subtract_units() {
        let l1 = 3.0 * METERS;
        let l2 = 3.0 * FEET;
        let eps = 0.0001 * METERS;
        assert_abs_diff_eq!(l1 - l2, 3.0*(1.0-0.3048)*METERS, epsilon = eps);
    }

    #[test]
    fn multiply_units() {
        let l1 = 2.0 * METERS;
        let l2 = 3.0 * FEET;
        let a1 = l1 * l2;
        assert_abs_diff_eq!(a1, Qnty::<si::Area>::new(2.0*3.0*0.3048));
    }

    #[test]
    fn divide_units() {
        let l = 2.0 * METERS;
        let t = Qnty::<si::Time>::new(1.0);
        let v = l / t;
        assert_eq!(v.value(), &2.0);
    }

    #[test]
    fn copy() {
        let l1 = 1.0 * METERS;
        let l2 = l1;
        assert_eq!(l1, l2);
    }

    #[test]
    fn unit_info() {
        type U = MetersUnit;
        assert_eq!(<U as UnitInfo>::abbr(), "m");
        let length = 3.0 * METERS;
        assert_eq!(format!("{length:.3}"), "3.000 m");
    }
}
