use crate::{
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
impl BaseUnitTagConversion<GramBaseUnit> for SlugBaseUnit {
    const SCALE: f64 = 14590.0;
}
impl BaseUnitTagConversion<SlugBaseUnit> for GramBaseUnit {
    const SCALE: f64 = 6.85218e-5;
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
