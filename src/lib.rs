extern crate typenum;

pub mod qnty;
pub mod unit;
pub mod system;

mod base_unit;
mod dimension;
mod types;
mod unit_system;

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::Qnty,
        unit::{Unit, UnitInfo},
        system::si::Length,
    };
    #[test]
    fn length() {
        let l1 = Qnty::<Length>::new(2.0);
        let l2 = <Length as Unit>::from_value(1.5);
        let l3 = Qnty::<Length>::new(3.5);
        assert_eq!(l1 + l2, l3);
    }

    #[test]
    fn unit_info() {
        type U = Length;
        assert_eq!(<U as UnitInfo>::abbr(), "m");
    }
}
