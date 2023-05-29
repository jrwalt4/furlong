use crate::{
    conversion::*,
    dimension::*,
    unit::*
};

pub struct GramBaseUnit;
impl BaseUnitTag for GramBaseUnit {
    type Dimension = MassBaseDimension;
}
impl BaseUnitInfo for GramBaseUnit {
    const NAME: Info = "gram";
    const SYMBOL: Info = "g";
}

pub type KilogramBaseUnit = ScaledBaseUnit<GramBaseUnit, 1000>;
impl BaseUnitInfo for KilogramBaseUnit {
    const NAME: Info = "kilo";
    const SYMBOL: Info = "kg";
}

pub struct SlugBaseUnit;
impl BaseUnitTag for SlugBaseUnit {
    type Dimension = MassBaseDimension;
}

impl ConversionTo<GramBaseUnit> for SlugBaseUnit {
    type Factor = ConvInt<14590>;
}

impl ConversionTo<SlugBaseUnit> for GramBaseUnit {
    type Factor = ConvRatio<1,14590>;
}

impl BaseUnitInfo for SlugBaseUnit {
    const NAME: Info = "slug";
    const SYMBOL: Info = "slug";
}

pub type PoundMassBaseUnit = ScaledBaseUnit<SlugBaseUnit, 16087, 500>; // 16087/500 = 32.174
impl BaseUnitInfo for PoundMassBaseUnit {
    const NAME: Info = "pound";
    const SYMBOL: Info = "lbm";
}
