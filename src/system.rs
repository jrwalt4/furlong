use crate::unit_system;
use crate::unit::*;
use crate::dimension::*;

pub mod si {
    use super::*;
    pub type System = unit_system::System<
                                        base_unit::KilogramBaseUnit, 
                                        base_unit::MeterBaseUnit, 
                                        base_unit::SecondBaseUnit
                                    >;

    pub type Mass = SystemUnit<System, MassDimension>;
    pub type Length = SystemUnit<System, LengthDimension>;
    pub const METERS: Length = Length::new();
    pub type Area = SystemUnit<System, AreaDimension>;
    pub type Time = SystemUnit<System, TimeDimension>;
}
