use crate::unit_system;
use crate::unit::*;
use crate::dimension::*;

pub mod si {
    use super::*;
    pub struct System;
    impl unit_system::UnitSystem for System {
        type Mass = base_unit::KilogramBaseUnit;
        type Length = base_unit::MeterBaseUnit;
        type Time = base_unit::SecondBaseUnit;
    }

    pub type Mass = SystemUnit<System, MassDimension>;
    pub type Length = SystemUnit<System, LengthDimension>;
    pub const METERS: Length = Length::new();
    pub type Area = SystemUnit<System, AreaDimension>;
    pub type Time = SystemUnit<System, TimeDimension>;
}
