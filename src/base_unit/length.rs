use crate::{
    conversion::*,
    dimension::*,
    unit::*
};

use typenum::{Add1, consts::*, Shleft, UFrac};

pub struct MeterBaseUnit;
impl BaseUnitTag for MeterBaseUnit {
    type Dimension = LengthBase;
}
impl BaseUnitInfo for MeterBaseUnit {
    const NAME: Info = "meter";
    const SYMBOL: Info = "m";
}

pub struct YardBaseUnit;
impl BaseUnitTag for YardBaseUnit {
    type Dimension = LengthBase;
}
impl BaseUnitInfo for YardBaseUnit {
    const NAME: Info = "yard";
    const SYMBOL: Info = "yd";
}

type U1143 = Add1<Shleft<U571, U1>>;
type U1250 = Shleft<U625, U1>;
impl ConversionTo<MeterBaseUnit> for YardBaseUnit {
    type Factor = UFrac<U1143, U1250>;
}

impl ConversionTo<YardBaseUnit> for MeterBaseUnit {
    type Factor = UFrac<U1250, U1143>;
}

pub type FootBaseUnit = ScaledBaseUnit<YardBaseUnit, UFrac<U1, U3>>;
impl BaseUnitInfo for FootBaseUnit {
    const NAME: Info = "feet";
    const SYMBOL: Info = "ft";
}
