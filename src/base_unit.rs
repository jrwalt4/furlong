use crate::base_dimension::*;

use std::marker::PhantomData as PD;

pub type Info = &'static str;

pub trait BaseUnit {
    /// BaseDimension of this BaseUnit
    type Dimension: BaseDimension;

    /// Multiplier to get from base units (Meters, Grams, Seconds, etc.)
    /// to this BaseUnit (i.e. how many base units are in 1 of this BaseUnit?)
    /// Example: 
    /// impl BaseUnit for Kilometers {
    ///     type Dimension = LengthBaseDimension;
    ///     const MULTIPLIER: f64 = 1000;
    /// }
    const MULTIPLIER: f64;
}

#[derive(Debug, Copy, Clone)]
pub struct ScaledBaseUnit<B, const N: u16, const D: u16 = 1> {
    base: PD<B>,
}

impl<B: BaseUnit, const N: u16, const D: u16> BaseUnit for ScaledBaseUnit<B, N, D>  {
    type Dimension = <B as BaseUnit>::Dimension;
    const MULTIPLIER: f64 = <B as BaseUnit>::MULTIPLIER * (N as f64)/(D as f64);
}

pub trait BaseUnitInfo: BaseUnit {
    const NAME: Info;
    const SYMBOL: Info;
}

pub trait BaseUnitConversion<T> {
    const FACTOR: f64;
}

impl<U: BaseUnit, T> BaseUnitConversion<T> for U
where 
    T: BaseUnit<Dimension = U::Dimension>
{
    /// multiply by `Self::MULTIPLIER` to get to `base unit`, then 
    /// divide by `T::MULTIPLIER` to get to its unit.
    const FACTOR: f64 = <U as BaseUnit>::MULTIPLIER / <T as BaseUnit>::MULTIPLIER;
}

pub mod mass {
    use super::*;
    #[derive(Debug)]
    pub struct GramBaseUnit;
    impl BaseUnit for GramBaseUnit {
        type Dimension = MassBaseDimension;

        const MULTIPLIER: f64 = 1.0;
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
    impl BaseUnit for SlugBaseUnit {
        type Dimension = MassBaseDimension;

        const MULTIPLIER: f64 = 14590.0;
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
    impl BaseUnit for MeterBaseUnit {
        type Dimension = LengthBaseDimension;
        const MULTIPLIER: f64 = 1.0;
    }
    impl BaseUnitInfo for MeterBaseUnit {
        const NAME: Info = "meter";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug)]
    pub struct YardBaseUnit;
    impl BaseUnit for YardBaseUnit {
        const MULTIPLIER: f64 = 0.9144;
        type Dimension = LengthBaseDimension;
    }
    impl BaseUnitInfo for YardBaseUnit {
        const NAME: Info = "yard";
        const SYMBOL: Info = "yd";
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
    impl BaseUnit for SecondBaseUnit {
        type Dimension = TimeBaseDimension;
        const MULTIPLIER: f64 = 1.0;
    }
    impl BaseUnitInfo for SecondBaseUnit {
        const NAME: Info = "second";
        const SYMBOL: Info = "s";
    }

    #[allow(dead_code)]
    pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, 60>;
}
