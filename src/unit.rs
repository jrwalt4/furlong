use std::marker::PhantomData as PD;
use std::ops::Mul;
use typenum::Integer;

use crate::base_unit::*;
use crate::dimension::*;
use crate::qnty::Qnty;
use crate::types::*;
use crate::unit_system::*;

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dimension;
    fn from_value(value: Real) -> Qnty<Self> {
        Qnty::<Self>::new(value)
    }
}

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug, Copy, Clone)]
pub struct MakeUnit<S: UnitSystem, D: Dimension> {
    system: PD<S>,
    dimension: PD<D>,
}
impl<S: UnitSystem, D: Dimension> MakeUnit<S, D> {
    fn new() -> MakeUnit<S, D> {
        MakeUnit {
            system: PD,
            dimension: PD,
        }
    }
}
impl<S: UnitSystem, D: Dimension> Unit for MakeUnit<S, D> {
    type System = S;
    type Dim = D;
}

impl<S, D> UnitInfo for MakeUnit<S, D>
where
    S: UnitSystem,
    <S as UnitSystem>::Mass: BaseUnitInfo,
    <S as UnitSystem>::Length: BaseUnitInfo,
    <S as UnitSystem>::Time: BaseUnitInfo,
    D: Dimension,
    <D as Dimension>::Mass: Integer,
    <D as Dimension>::Length: Integer,
    <D as Dimension>::Time: Integer,
{
    fn abbr() -> String {
        let mass_abbr = <<S as UnitSystem>::Mass as BaseUnitInfo>::SYMBOL;
        let mass_pwr = <<D as Dimension>::Mass as Integer>::I8;
        let mass_part = match mass_pwr {
            0 => String::from(""),
            1 => String::from(mass_abbr),
            _ => format!("{}^{}", mass_abbr, mass_pwr),
        };

        let length_abbr = <<S as UnitSystem>::Length as BaseUnitInfo>::SYMBOL;
        let length_pwr = <<D as Dimension>::Length as Integer>::I8;
        let length_part = match length_pwr {
            0 => String::from(""),
            1 => String::from(length_abbr),
            _ => format!("{}^{}", length_abbr, length_pwr),
        };

        let time_abbr = <<S as UnitSystem>::Time as BaseUnitInfo>::SYMBOL;
        let time_pwr = <<D as Dimension>::Time as Integer>::I8;
        let time_part = match time_pwr {
            0 => String::from(""),
            1 => String::from(time_abbr),
            _ => format!("{}^{}", time_abbr, time_pwr),
        };
        format!("{}{}{}", mass_part.as_str(), length_part.as_str(), time_part.as_str())
    }
}

impl<S, D, Ur> PartialEq<Ur> for MakeUnit<S, D>
where
    S: UnitSystem,
    D: Dimension + PartialEq<<Ur as Unit>::Dim>,
    Ur: Unit,
{
    fn eq(&self, _other: &Ur) -> bool {
        true
    }
}

impl<S, Dl, Dr> Mul<MakeUnit<S, Dr>> for MakeUnit<S, Dl>
where
    S: UnitSystem,
    Dl: Dimension + Mul<Dr>,
    <Dl as Mul<Dr>>::Output: Dimension,
    Dr: Dimension,
{
    type Output = MakeUnit<S, ProdDimension<Dl, Dr>>;
    fn mul(self, _: MakeUnit<S, Dr>) -> Self::Output {
        MakeUnit::new()
    }
}

pub type ProdUnit<Ul, Ur> = <Ul as Mul<Ur>>::Output;
