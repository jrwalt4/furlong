use crate::unit_system::UnitSystem;
use crate::unit::*;
use crate::dimension::*;

macro_rules! export_base_units {
    () => {
        pub type Mass = SystemUnit<System, MassDimension>;
        pub type Length = SystemUnit<System, LengthDimension>;
        pub type Area = SystemUnit<System, AreaDimension>;
        pub type Time = SystemUnit<System, TimeDimension>;
    };
}
pub mod si {
    use super::*;
    pub struct System;
    impl UnitSystem for System {
        type Mass = base_unit::KilogramBaseUnit;
        type Length = base_unit::MeterBaseUnit;
        type Time = base_unit::SecondBaseUnit;
    }
    export_base_units!();
    pub const METERS: Length = Length::new();
}

pub mod imperial {
    use super::*;
    pub struct System;
    impl UnitSystem for System {
        type Mass = base_unit::SlugBaseUnit;
        type Length = base_unit::FootBaseUnit;
        type Time = base_unit::SecondBaseUnit;
    }
    export_base_units!();
    pub const FEET: Length = Length::new();
}
