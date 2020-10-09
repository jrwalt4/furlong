use crate::types::*;

pub trait BaseUnit {
    const CONV: Real;
}

pub trait BaseUnitInfo {
    const ABBR: Abbr;
}

#[derive(Debug, Copy, Clone)]
pub struct MeterBaseUnit;
impl BaseUnit for MeterBaseUnit {
    const CONV: Real = 1.0;
}
impl BaseUnitInfo for MeterBaseUnit {
    const ABBR: Abbr = "m";
}

#[derive(Debug, Copy, Clone)]
pub struct SecondBaseUnit;
impl BaseUnit for SecondBaseUnit {
    const CONV: Real = 1.0;
}
impl BaseUnitInfo for SecondBaseUnit {
    const ABBR: Abbr = "s";
}
