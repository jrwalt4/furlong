use crate::unit_system;
use crate::base_unit::*;
use crate::unit::*;
use crate::dimension::*;
use crate::types::*;

pub mod si {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    pub struct KilogramBaseUnit;
    impl BaseUnit for KilogramBaseUnit {
        const CONV: Real = 1.0;
    }
    impl BaseUnitInfo for KilogramBaseUnit {
        const NAME: Info = "kilo";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug, Copy, Clone)]
    pub struct MeterBaseUnit;
    impl BaseUnit for MeterBaseUnit {
        const CONV: Real = 1.0;
    }
    impl BaseUnitInfo for MeterBaseUnit {
        const NAME: Info = "meter";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug, Copy, Clone)]
    pub struct SecondBaseUnit;
    impl BaseUnit for SecondBaseUnit {
        const CONV: Real = 1.0;
    }
    impl BaseUnitInfo for SecondBaseUnit {
        const NAME: Info = "second";
        const SYMBOL: Info = "s";
    }

    pub type System = unit_system::System<KilogramBaseUnit, MeterBaseUnit, SecondBaseUnit>;

    pub type Mass = MakeUnit<System, MassDimension>;
    pub type Length = MakeUnit<System, LengthDimension>;
    pub type Area = MakeUnit<System, AreaDimension>;
    pub type Time = MakeUnit<System, TimeDimension>;
}
