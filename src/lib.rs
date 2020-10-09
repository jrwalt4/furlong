extern crate typenum;

pub mod qnty;
pub mod unit;

mod base_unit;
mod dimension;
mod types;
mod unit_system;

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::Qnty,
        unit::{LengthUnit, Unit},
    };
    #[test]
    fn length() {
        let l1 = Qnty::<LengthUnit>::new(2.0);
        let l2 = <LengthUnit as Unit>::from_value(1.5);
        let l3 = Qnty::<LengthUnit>::new(3.5);
        assert_eq!(l1 + l2, l3);
    }
}
