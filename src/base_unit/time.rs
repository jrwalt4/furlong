use crate::{
    dimension::*,
    unit::*
};

use typenum::consts::U60;

pub struct SecondBaseUnit;
impl BaseUnitTag for SecondBaseUnit {
    type Dimension = TimeBase;
}
impl BaseUnitInfo for SecondBaseUnit {
    const NAME: Info = "second";
    const SYMBOL: Info = "s";
}

pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, U60>;
