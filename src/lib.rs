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
        system::si::{self, Length as MetersUnit, METERS},
        system::imperial::{self, Length as FeetUnit, FEET}
    };
    #[test]
    fn add_same_unit() {
        let l1 = Qnty::<MetersUnit>::new(2.0);
        let l2 = 1.5 * METERS;
        let l3 = Qnty::<MetersUnit>::new(3.5);
        assert_eq!(l1 + l2, l3);
    }

    #[test]
    fn add_different_units() {
        let l1 = Qnty::<FeetUnit>::new(2.0);
        let l2 = Qnty::<MetersUnit>::new(1.0);
        let l3 = 5.28084 * FEET;
        approx::assert_abs_diff_eq!(l1 + l2, l3, epsilon = 0.0001 * FEET);
    }

    #[test]
    fn unit_info() {
        type U = MetersUnit;
        assert_eq!(<U as UnitInfo>::abbr(), "m");
    }
}
