use crate::{
    base_dimension::*
};

use std::marker::PhantomData as PD;

pub type Info = &'static str;

pub trait BaseUnitTag {
    /// BaseDimension of this BaseUnit
    type Dimension: BaseDimension;
}

pub trait BaseUnitTagConversion<B: BaseUnitTag> {
    const SCALE: f64;
}

impl<B: BaseUnitTag> BaseUnitTagConversion<B> for B {
    const SCALE: f64 = 1.0;
}

pub trait BaseUnit {
    /// If a scaled base unit, the base that it is scaled from
    type Base: BaseUnitTag;

    /// Conversion to [`BaseUnit::Base`]
    /// (i.e. how many `Base`'s are in 1 of Self)
    const SCALE: f64;
}

impl<B: BaseUnitTag> BaseUnit for B {
    type Base = Self;
    const SCALE: f64 = 1.0;
}

#[derive(Debug, Copy, Clone)]
pub struct ScaledBaseUnit<B, const N: u16, const D: u16 = 1> {
    base: PD<B>,
}

impl<B: BaseUnit, const N: u16, const D: u16> BaseUnit for ScaledBaseUnit<B, N, D>  {
    type Base = <B as BaseUnit>::Base;

    const SCALE: f64 = N as f64 / D as f64 * <B as BaseUnit>::SCALE;
}

pub trait BaseUnitInfo: BaseUnit {
    const NAME: Info;
    const SYMBOL: Info;
}

pub trait BaseUnitConversion<T> {
    const SCALE: f64;
}

#[allow(non_upper_case_globals)]
impl<B1: BaseUnit, B2: BaseUnit> BaseUnitConversion<B2> for B1
where
    <B1 as BaseUnit>::Base: BaseUnitTag<Dimension = <<B2 as BaseUnit>::Base as BaseUnitTag>::Dimension>
    + BaseUnitTagConversion<<B2 as BaseUnit>::Base>
{
    const SCALE: f64 =
        <B1 as BaseUnit>::SCALE
        * <<B1 as BaseUnit>::Base as BaseUnitTagConversion<<B2 as BaseUnit>::Base>>::SCALE
        / <B2 as BaseUnit>::SCALE;
}

pub mod mass {
    use super::*;
    #[derive(Debug)]
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
  
    #[derive(Debug)]
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
}

pub mod length {
    use super::*;
    #[derive(Debug)]
    pub struct MeterBaseUnit;
    impl BaseUnitTag for MeterBaseUnit {
        type Dimension = LengthBaseDimension;
    }
    impl BaseUnitInfo for MeterBaseUnit {
        const NAME: Info = "meter";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug)]
    pub struct YardBaseUnit;
    impl BaseUnitTag for YardBaseUnit {
        type Dimension = LengthBaseDimension;
    }
    impl BaseUnitInfo for YardBaseUnit {
        const NAME: Info = "yard";
        const SYMBOL: Info = "yd";
    }

    impl BaseUnitTagConversion<MeterBaseUnit> for YardBaseUnit {
        const SCALE: f64 = 0.9144;
    }

    impl BaseUnitTagConversion<YardBaseUnit> for MeterBaseUnit {
        const SCALE: f64 = 1.0936132983;
    }

    pub type FootBaseUnit = ScaledBaseUnit<YardBaseUnit, 1, 3>;
    impl BaseUnitInfo for FootBaseUnit {
        const NAME: Info = "feet";
        const SYMBOL: Info = "ft";
    }
}

pub mod time {
    use super::*;
    #[derive(Debug)]
    pub struct SecondBaseUnit;
    impl BaseUnitTag for SecondBaseUnit {
        type Dimension = TimeBaseDimension;
    }
    impl BaseUnitInfo for SecondBaseUnit {
        const NAME: Info = "second";
        const SYMBOL: Info = "s";
    }

    #[allow(dead_code)]
    pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, 60>;
}
