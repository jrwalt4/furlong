//! A crate that provides static typing for scientific units with minimal runtime impact. 
//! 
//! The design is intended to mirror the [boost/units](http://boost.org/libs/units) library. 
//! 
//! # Examples
//! ```rust
//! use furlong::{Qnty, system::si};
//! let length = si::Length::new(2.0); // 2.0 meters (si::Length)
//! let width = si::Length::new(3.0); // 3.0 meters
//! let area = length * width;
//! assert_eq!(area, si::Area::new(6.0));
//! 
//! use typenum::Quot;
//! let time = si::Time::new(3.0); // 3.0 seconds
//! let velocity = length / time;
//! type Velocity = Quot<si::Length, si::Time>;
//! let expected = Velocity::new(2.0 / 3.0);
//! assert_eq!(velocity, expected);
//! ```
//! 

extern crate typenum;

pub mod base_unit;
mod qnty;
pub use qnty::Qnty;
pub mod unit;
pub mod system;
pub mod conversion;
pub mod dimension;
