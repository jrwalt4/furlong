use std::marker::PhantomData as PD;
use std::ops::{Add, Mul};
use typenum::Integer;

use crate::dimension::*;
use crate::qnty::Qnty;
use crate::types::{Real, Info};
use crate::unit_system::*;

pub trait BaseUnit {
    type Dimension: BaseDimension;
}

pub struct ScaledBaseUnit<B, const N: u16, const D: u16 = 1> {
    base: PD<B>,
    //scale: PD<R>
}

impl<B: BaseUnit, const N: u16, const D: u16> BaseUnit for ScaledBaseUnit<B, N, D>  {
    type Dimension = <B as BaseUnit>::Dimension;
}

pub trait BaseUnitInfo {
    const NAME: Info;
    const SYMBOL: Info;
}

pub trait FromUnit<U> {
    const FROM: f32;
}

pub trait IntoUnit<U> {
    const INTO: f32;
}

impl<T, U: FromUnit<T>> IntoUnit<U> for T {
    const INTO: f32 = 1.0 / <U as FromUnit<T>>::FROM;
}

impl<B: FromUnit<U>, U, const N: u16, const D: u16> FromUnit<U> for ScaledBaseUnit<B, N, D> {
    const FROM: f32 = <B as FromUnit<U>>::FROM / (N as f32) * (D as f32) ;
}

pub mod base_unit {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    pub struct KilogramBaseUnit;
    impl BaseUnit for KilogramBaseUnit {
        type Dimension = MassBaseDimension;
    }
    impl BaseUnitInfo for KilogramBaseUnit {
        const NAME: Info = "kilo";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug, Copy, Clone)]
    pub struct MeterBaseUnit;
    impl BaseUnit for MeterBaseUnit {
        type Dimension = LengthBaseDimension;
    }
    impl BaseUnitInfo for MeterBaseUnit {
        const NAME: Info = "meter";
        const SYMBOL: Info = "m";
    }

    #[derive(Debug, Copy, Clone)]
    pub struct FootBaseUnit;
    impl BaseUnit for FootBaseUnit {
        type Dimension = LengthBaseDimension;
    }
    impl BaseUnitInfo for FootBaseUnit {
        const NAME: Info = "feet";
        const SYMBOL: Info = "ft";
    }

    impl FromUnit<MeterBaseUnit> for FootBaseUnit {
        const FROM: f32 = 3.281;
    }

    pub type YardBaseUnit = ScaledBaseUnit<FootBaseUnit, 3>;
    impl BaseUnitInfo for YardBaseUnit {
        const NAME: Info = "yard";
        const SYMBOL: Info = "yd";
    }

    #[derive(Debug, Copy, Clone)]
    pub struct SecondBaseUnit;
    impl BaseUnit for SecondBaseUnit {
        type Dimension = TimeBaseDimension;
    }
    impl BaseUnitInfo for SecondBaseUnit {
        const NAME: Info = "second";
        const SYMBOL: Info = "s";
    }

    pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, 60>;

    #[cfg(test)]
    mod base_unit_test {
        use super::*;
        use approx::assert_abs_diff_eq;

        #[test]
        fn conversion() {
            let meters_to_feet = <FootBaseUnit as FromUnit<MeterBaseUnit>>::FROM;
            assert_abs_diff_eq!(meters_to_feet, 3.281, epsilon = 0.001);
            let meters_to_yards = <YardBaseUnit as FromUnit<MeterBaseUnit>>::FROM;
            assert_abs_diff_eq!(meters_to_yards, 1.094, epsilon = 0.001);
        }
    }
}

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dim;
}

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug, Copy, Clone)]
pub struct SystemUnit<S: UnitSystem, D: Dim> {
    system: PD<S>,
    dimension: PD<D>,
}

impl<S: UnitSystem, D: Dim> SystemUnit<S, D> {
    pub const fn new() -> Self {
        Self {
            system: PD,
            dimension: PD,
        }
    }
}

impl<S: UnitSystem, D: Dim> Unit for SystemUnit<S, D> {
    type System = S;
    type Dim = D;
}

impl<S, D> UnitInfo for SystemUnit<S, D>
where
    S: UnitSystem,
    <S as UnitSystem>::Mass: BaseUnitInfo,
    <S as UnitSystem>::Length: BaseUnitInfo,
    <S as UnitSystem>::Time: BaseUnitInfo,
    D: Dim,
    <D as Dim>::Mass: Integer,
    <D as Dim>::Length: Integer,
    <D as Dim>::Time: Integer,
{
    fn abbr() -> String {
        let mass_abbr = <<S as UnitSystem>::Mass as BaseUnitInfo>::SYMBOL;
        let mass_pwr = <<D as Dim>::Mass as Integer>::I8;
        let mass_part = match mass_pwr {
            0 => String::from(""),
            1 => String::from(mass_abbr),
            _ => format!("{}^{}", mass_abbr, mass_pwr),
        };

        let length_abbr = <<S as UnitSystem>::Length as BaseUnitInfo>::SYMBOL;
        let length_pwr = <<D as Dim>::Length as Integer>::I8;
        let length_part = match length_pwr {
            0 => String::from(""),
            1 => String::from(length_abbr),
            _ => format!("{}^{}", length_abbr, length_pwr),
        };

        let time_abbr = <<S as UnitSystem>::Time as BaseUnitInfo>::SYMBOL;
        let time_pwr = <<D as Dim>::Time as Integer>::I8;
        let time_part = match time_pwr {
            0 => String::from(""),
            1 => String::from(time_abbr),
            _ => format!("{}^{}", time_abbr, time_pwr),
        };
        format!("{}{}{}", mass_part.as_str(), length_part.as_str(), time_part.as_str())
    }
}

impl<S, D, Ur> PartialEq<Ur> for SystemUnit<S, D>
where
    S: UnitSystem,
    D: Dim + PartialEq<<Ur as Unit>::Dim>,
    Ur: Unit,
{
    fn eq(&self, _other: &Ur) -> bool {
        true
    }
}

impl<S, D> Add for SystemUnit<S, D>
where
    S: UnitSystem,
    D: Dim,
{
    type Output = Self;
    fn add(self, _: Self) -> Self {
        unimplemented!()
    }
}

impl<S, Dl, Dr> Mul<SystemUnit<S, Dr>> for SystemUnit<S, Dl>
where
    S: UnitSystem,
    Dl: Dim + Mul<Dr>,
    <Dl as Mul<Dr>>::Output: Dim,
    Dr: Dim,
{
    type Output = SystemUnit<S, ProdDimension<Dl, Dr>>;
    fn mul(self, _: SystemUnit<S, Dr>) -> Self::Output {
        unimplemented!()
    }
}

pub type ProdUnit<Ul, Ur> = <Ul as Mul<Ur>>::Output;

impl<S: UnitSystem, D: Dim> Mul<SystemUnit<S, D>> for Real {
    type Output = Qnty<SystemUnit<S, D>>;
    fn mul(self, _unit: SystemUnit<S, D>) -> Self::Output {
        Qnty::<SystemUnit<S, D>>::new(self)
    }
}
