extern crate typenum;

pub mod qnty;
pub mod unit;

mod base_unit;
mod dimension;
mod types;
mod unit_system;

pub mod si {
    use super::unit_system;
    use super::base_unit;
    use super::unit::MakeUnit;
    use super::dimension::*;

    pub type System = unit_system::System<base_unit::MeterBaseUnit, base_unit::SecondBaseUnit>;

    pub type Length = MakeUnit<System, LengthDimension>;
    pub type Area = MakeUnit<System, AreaDimension>;
    pub type Time = MakeUnit<System, TimeDimension>;
}

#[cfg(test)]
mod unit_test {
    use super::{
        qnty::Qnty,
        unit::Unit,
        si::Length,
    };
    #[test]
    fn length() {
        let l1 = Qnty::<Length>::new(2.0);
        let l2 = <Length as Unit>::from_value(1.5);
        let l3 = Qnty::<Length>::new(3.5);
        assert_eq!(l1 + l2, l3);
    }
}
