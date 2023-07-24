use crate::{
    conversion::*,
    dimension::*,
    unit::*
};

use typenum::{consts::*, Shleft, Sum, UFrac};

pub struct GramBaseUnit;
impl BaseUnitTag for GramBaseUnit {
    type Dimension = MassBase;
}
impl BaseUnitInfo for GramBaseUnit {
    const NAME: Info = "gram";
    const SYMBOL: Info = "g";
}

pub type KilogramBaseUnit = ScaledBaseUnit<GramBaseUnit, UFrac<U1000>>;
impl BaseUnitInfo for KilogramBaseUnit {
    const NAME: Info = "kilo";
    const SYMBOL: Info = "kg";
}

pub struct SlugBaseUnit;
impl BaseUnitTag for SlugBaseUnit {
    type Dimension = MassBase;
}

// 14590 = (56 << 8) + 254
type U14590 = Sum<Shleft<U56, U8>, U254>;
impl ConversionTo<GramBaseUnit> for SlugBaseUnit {
    type Factor = UFrac<U14590>;
}

impl ConversionTo<SlugBaseUnit> for GramBaseUnit {
    type Factor = UFrac<U1, U14590>;
}

impl BaseUnitInfo for SlugBaseUnit {
    const NAME: Info = "slug";
    const SYMBOL: Info = "slug";
}

// 16087 = (1005 << 4) + 7
type U16087 = Sum<Shleft<U1005, U4>, U7>;
pub type PoundMassBaseUnit = ScaledBaseUnit<SlugBaseUnit, UFrac<U16087, U500>>; // 16087/500 = 32.174
impl BaseUnitInfo for PoundMassBaseUnit {
    const NAME: Info = "pound";
    const SYMBOL: Info = "lbm";
}
