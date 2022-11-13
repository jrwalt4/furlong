use std::marker::PhantomData as PD;
use std::ops::{Add, Mul};
use typenum::Integer;

use crate::dimension::*;
use crate::qnty::Qnty;
use crate::types::{Real, Info};
use crate::unit_system::*;

pub trait BaseUnit {
    const CONV: Real;
}

pub trait BaseUnitInfo {
    const NAME: Info;
    const SYMBOL: Info;
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
