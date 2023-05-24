use crate::{
    dimension::*,
    unit::*
};

pub struct SecondBaseUnit;
impl BaseUnitTag for SecondBaseUnit {
    type Dimension = TimeBaseDimension;
}
impl BaseUnitInfo for SecondBaseUnit {
    const NAME: Info = "second";
    const SYMBOL: Info = "s";
}

pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, 60>;
