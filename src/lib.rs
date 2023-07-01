//! A crate that provides static typing for scientific units with minimal runtime impact. 
//! 
//! The design is intended to mirror the [boost/units](http://boost.org/libs/units) library. 
//! 
//! # Examples
//! ```rust
//! use furlong::{Qnty, system::si::{Meters, Seconds}};
//! use typenum::{Prod, Quot};
//! 
//! let length = Meters::new(2.0); // 2.0 meters
//! let width = Meters::new(3.0); // 3.0 meters
//! let area = length * width;
//! type SquareMeters = Prod<Meters, Meters>;
//! assert_eq!(area, SquareMeters::new(6.0));
//! 
//! let time = Seconds::new(3.0); // 3.0 seconds
//! // length is Copy because f64 is Copy
//! let velocity = length / time;
//! type Velocity = Quot<Meters, Seconds>;
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
pub mod mp;
