use crate::base_unit::{mass, length, time};
use crate::unit_system::MakeSystem;
use crate::unit::*;
use crate::dimension::*;

macro_rules! export_base_units {
    ($SYS:ty) => {
        pub type Mass = SystemUnit<$SYS, MassDimension>;
        pub type Length = SystemUnit<$SYS, LengthDimension>;
        pub type Area = SystemUnit<$SYS, AreaDimension>;
        pub type Time = SystemUnit<$SYS, TimeDimension>;
    };
}
pub mod si {
    use super::*;

    pub type System = MakeSystem<
        mass::KilogramBaseUnit,
        length::MeterBaseUnit,
        time::SecondBaseUnit
    >;
    export_base_units!(System);
    pub const METERS: Length = Length::new();
}

pub mod imperial {
    use super::*;
    
    pub type System = MakeSystem<
        mass::SlugBaseUnit,
        length::FootBaseUnit,
        time::SecondBaseUnit
    >;
    export_base_units!(System);
    pub const FEET: Length = Length::new();
}
