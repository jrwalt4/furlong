use crate::types::{Real, Info};

pub trait BaseUnit {
    const CONV: Real;
}

pub trait BaseUnitInfo {
    const NAME: Info;
    const SYMBOL: Info;
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
