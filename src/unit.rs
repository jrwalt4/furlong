use std::marker::PhantomData as PD;
use std::ops::{Mul, Div};
use typenum::{Integer, Prod, Quot};

use crate::{
    conversion::*,
    dimension::*,
    qnty::Qnty,
};

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

pub struct ScaledBaseUnit<B, const N: u32, const D: u32 = 1> {
    base: PD<B>,
}

impl<B: BaseUnit, const N: u32, const D: u32> BaseUnit for ScaledBaseUnit<B, N, D>  {
    type Base = <B as BaseUnit>::Base;

    const SCALE: f64 = N as f64 / D as f64 * <B as BaseUnit>::SCALE;
}

impl<B: BaseUnitTag, const N: u32, const D: u32> ConversionTo<B> for ScaledBaseUnit<B, N, D> {
    type Factor = ConvRatio<N, D>;
}

pub type Info = &'static str;

pub trait BaseUnitInfo: BaseUnit {
    const NAME: Info;
    const SYMBOL: Info;
}

pub trait BaseUnitConversion<T> {
    const SCALE: f64;
}

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

impl<
    B1: BaseUnit, const N1: u32, const D1: u32, 
    B2: BaseUnit, const N2: u32, const D2: u32
> ConversionTo<ScaledBaseUnit<B2, N2, D2>> for ScaledBaseUnit<B1, N1, D1>
where
    <B1 as BaseUnit>::Base: BaseUnitTag<Dimension = <<B2 as BaseUnit>::Base as BaseUnitTag>::Dimension>,
    B1: ConversionTo<B2>
{
    type Factor = ConvProd<ConvProd<ConvRatio<N1,D1>,ConvRatio<N2, D2>>,<B1 as ConversionTo<B2>>::Factor>;
}

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dim;
}

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

pub trait UnitSystemPart<D: BaseDimension> {
    type Base: BaseUnit;
}

pub type GetBase<S, D> = <S as UnitSystemPart<D>>::Base;

pub trait UnitSystem:
    UnitSystemPart<MassBaseDimension> +
    UnitSystemPart<LengthBaseDimension> +
    UnitSystemPart<TimeBaseDimension> {}

pub struct MakeSystem<MB, LB, TB> {
    mass_base: PD<MB>,
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<Mass: BaseUnit, Length, Time> UnitSystemPart<MassBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Mass;
}

impl<Mass, Length: BaseUnit, Time> UnitSystemPart<LengthBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Length;
}

impl<Mass, Length, Time: BaseUnit> UnitSystemPart<TimeBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Time;
}

impl<M: BaseUnit, L: BaseUnit, T: BaseUnit> UnitSystem for MakeSystem<M, L, T> {}

pub struct SystemUnit<S: UnitSystem, D: Dim> {
    system: PD<S>,
    dimension: PD<D>,
}

impl<S: UnitSystem, D: Dim> SystemUnit<S, D> {
    pub fn new<T>(value: T) -> Qnty<Self, T> {
        Qnty::from_raw_value(value)
    }
}

impl<S: UnitSystem, D: Dim> Unit for SystemUnit<S, D> {
    type System = S;
    type Dim = D;
}

impl<S, D> UnitInfo for SystemUnit<S, D>
where
    S: UnitSystem,
    GetBase<S, MassBaseDimension>: BaseUnitInfo,
    GetBase<S, LengthBaseDimension>: BaseUnitInfo,
    GetBase<S, TimeBaseDimension>: BaseUnitInfo,
    D: Dim,
    GetDim<D, MassBaseDimension>: Integer,
    GetDim<D, LengthBaseDimension>: Integer,
    GetDim<D, TimeBaseDimension>: Integer,
{
    fn abbr() -> String {
        let mass_abbr = <GetBase<S, MassBaseDimension> as BaseUnitInfo>::SYMBOL;
        let mass_pwr = <GetDim<D, MassBaseDimension> as Integer>::I8;
        let mass_part = match mass_pwr {
            0 => String::from(""),
            1 => String::from(mass_abbr),
            _ => format!("{}^{}", mass_abbr, mass_pwr),
        };

        let length_abbr = <GetBase<S, LengthBaseDimension> as BaseUnitInfo>::SYMBOL;
        let length_pwr = <GetDim<D, LengthBaseDimension> as Integer>::I8;
        let length_part = match length_pwr {
            0 => String::from(""),
            1 => String::from(length_abbr),
            _ => format!("{}^{}", length_abbr, length_pwr),
        };

        let time_abbr = <GetBase<S, TimeBaseDimension> as BaseUnitInfo>::SYMBOL;
        let time_pwr = <GetDim<D, TimeBaseDimension> as Integer>::I8;
        let time_part = match time_pwr {
            0 => String::from(""),
            1 => String::from(time_abbr),
            _ => format!("{}^{}", time_abbr, time_pwr),
        };
        format!("{}{}{}", mass_part.as_str(), length_part.as_str(), time_part.as_str())
    }
}

impl<S: UnitSystem, D: Dim, Ur: Unit> Mul<Ur> for SystemUnit<S, D>
where
    D: Mul<<Ur as Unit>::Dim>,
    Prod<D, <Ur as Unit>::Dim>: Dim
{
    type Output = SystemUnit<S, Prod<D, <Ur as Unit>::Dim>>;

    fn mul(self, _: Ur) -> Self::Output {
        unimplemented!()
    }
}

impl<S: UnitSystem, D: Dim, Ur: Unit> Div<Ur> for SystemUnit<S, D>
where
    D: Div<<Ur as Unit>::Dim>,
    Quot<D, <Ur as Unit>::Dim>: Dim
{
    type Output = SystemUnit<S, Quot<D, <Ur as Unit>::Dim>>;

    fn div(self, _: Ur) -> Self::Output {
        unimplemented!();
    }
}

pub type GetSystemUnit<U> = SystemUnit<<U as Unit>::System, <U as Unit>::Dim>;

pub struct ScaledUnit<U, const NUM: u32, const DEN: u32 = 1> {
    unit: PD<U>
}

impl<U: Unit, const NUM: u32, const DEN: u32> Unit for ScaledUnit<U, NUM, DEN> {
    type System = <U as Unit>::System;
    type Dim = <U as Unit>::Dim;
}

impl<U: Unit, const NUM: u32, const DEN: u32> ScaledUnit<U, NUM, DEN> {
    pub fn new<T: Convertible>(value: T) -> Qnty<Self, T> 
    where Conversion<U, GetSystemUnit<U>>: UnitConversion {
        Qnty::from_raw_value(value.convert::<Conversion<Self, GetSystemUnit<U>>>())
    }
}
