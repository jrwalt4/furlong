use crate::{
    conversion::*,
    dimension::*,
    unit::*
};

pub struct MeterBaseUnit;
impl BaseUnitTag for MeterBaseUnit {
    type Dimension = LengthBaseDimension;
}
impl BaseUnitInfo for MeterBaseUnit {
    const NAME: Info = "meter";
    const SYMBOL: Info = "m";
}

pub struct YardBaseUnit;
impl BaseUnitTag for YardBaseUnit {
    type Dimension = LengthBaseDimension;
}
impl BaseUnitInfo for YardBaseUnit {
    const NAME: Info = "yard";
    const SYMBOL: Info = "yd";
}

impl ConversionTo<MeterBaseUnit> for YardBaseUnit {
    type Factor = ConvRatio<1143, 1250>;
}

impl ConversionTo<YardBaseUnit> for MeterBaseUnit {
    type Factor = ConvRatio<1250, 1143>;
}

impl BaseUnitTagConversion<MeterBaseUnit> for YardBaseUnit {
    const SCALE: f64 = 0.9144;
}

impl BaseUnitTagConversion<YardBaseUnit> for MeterBaseUnit {
    const SCALE: f64 = 1.0936132983377078;
}

pub type FootBaseUnit = ScaledBaseUnit<YardBaseUnit, 1, 3>;
impl BaseUnitInfo for FootBaseUnit {
    const NAME: Info = "feet";
    const SYMBOL: Info = "ft";
}
