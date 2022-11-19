extern crate typenum;

pub mod qnty;
pub mod unit;
pub mod system;
pub mod rational;

mod dimension;
mod types;
mod unit_system;

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::Qnty,
        unit::UnitInfo,
        system::si::{Length, METERS},
    };
    #[test]
    fn length() {
        let l1 = Qnty::<Length>::new(2.0);
        let l2 = 1.5 * METERS;
        //let l3 = Qnty::<Length>::new(3.5);
        assert_eq!((l1 + l2).value(), 3.5);
    }

    #[test]
    fn unit_info() {
        type U = Length;
        assert_eq!(<U as UnitInfo>::abbr(), "m");
    }
}
