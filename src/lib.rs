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

pub mod base_dimension;
mod base_unit;
mod qnty;
pub use qnty::{Qnty, IntoUnit};
pub mod unit;
pub mod system;
pub use system::{si, imperial};
mod dimension;
mod unit_system;

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::{Qnty, IntoUnit},
        base_unit::{BaseUnit, FootBaseUnit},
        unit::UnitInfo,
        system::si::{self, Length as MetersUnit, METERS},
        system::imperial::{self, Length as FeetUnit, FEET}
    };
    use std::fmt::Display;
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
        let l3 = l1 + l2.as_type::<f64>();
        assert_eq!(l3, 3.5 * METERS);
    }

    #[test]
    fn qnty_into() {
        let m = (0.9144/3.0) as f64 * METERS;
        let ft: Qnty<FeetUnit> = m.into_unit();
        assert_eq!(ft.value(), &1.0);
        assert_eq!(ft, m);
    }

    #[test]
    fn add_different_units() {
        let l1 = Qnty::<FeetUnit>::new(2.0);
        let l2 = Qnty::<MetersUnit>::new(1.0);
        let l3 = 5.28084 * FEET;
        assert_eq!(l1 + l2, l3);
    }

    #[test]
    fn add_different_types() {
        let mut l_f64 = Qnty::<FeetUnit>::new(2.0);
        let l_i32 = Qnty::<FeetUnit, i32>::new(1);
        l_f64 += l_i32.as_type::<f64>();
        assert_eq!(l_f64, 3.0 * FEET);
    }

    #[test]
    fn with_vectors() {
        use std::ops::{Mul, Add};

        #[derive(Debug, Clone, Copy)]
        struct Vec3<T>(T, T, T);

        impl<T: Mul<f64, Output = T>> Mul<f64> for Vec3<T> {
            type Output = Vec3<T>;
            fn mul(self, rhs: f64) -> Self::Output {
                Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
            }
        }

        impl<T: Add<T2>, T2> Add<Vec3<T2>> for Vec3<T> {
            type Output = Vec3<<T as Add<T2>>::Output>;
            fn add(self, rhs: Vec3<T2>) -> Self::Output {
                Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
            }
        }

        impl std::cmp::PartialEq for Vec3<f64> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 &&
                self.1 == other.1 &&
                self.2 == other.2
            }
        }

        let length_v = Qnty::<FeetUnit, Vec3<f64>>::new(Vec3::<f64>(1.0, 2.0, 3.0));
        let width_v = length_v;
        let perimeter_v = length_v + width_v;
        assert_eq!(perimeter_v, Qnty::<FeetUnit, Vec3<f64>>::new(Vec3::<f64>(2.0, 4.0, 6.0)));
    }

    #[test]
    fn add_complex_units() {
        let a1 = Qnty::<si::Area>::new(2.0);
        let a2 = Qnty::<imperial::Area>::new(2.0);
        let a1a2 = Qnty::<si::Area>::new(2.0*(1.0 + <FootBaseUnit as BaseUnit>::MULTIPLIER.powi(2)));
        assert_eq!(a1 + a2, a1a2);
    }

    #[test]
    fn subtract_units() {
        let l1 = 3.0 * METERS;
        let l2 = 3.0 * FEET;
        assert_eq!(l1 - l2, 3.0*(1.0-0.3048)*METERS);
    }

    #[test]
    fn multiply_units() {
        let l1 = 2.0 * METERS;
        let l2 = 3.0 * FEET;
        let a1 = l1 * l2;
        assert_eq!(a1, Qnty::<si::Area>::new(2.0*3.0*0.3048));
    }

    #[test]
    fn divide_units() {
        let l = 2.0 as f64 * METERS;
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

    #[test]
    fn generic_unit_info() {
        // test Display for i32
        let q = Qnty::<FeetUnit, i32>::new(2);
        assert_eq!(format!("{q}"), "2 ft");

        // test Display for custom type
        #[derive(Debug, Clone, Copy)]
        struct MyValue(f32);
        impl Display for MyValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
        let mv = MyValue(3.0);
        let q = Qnty::<FeetUnit, MyValue>::new(mv);
        assert_eq!(format!("{q}"), format!("{mv} ft"));
    }
}
