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

pub trait BaseUnit {
    /// If a scaled base unit, the base that it is scaled from
    type Base: BaseUnitTag;

    /// Conversion to [`BaseUnit::Base`]
    /// (i.e. how many `Base`'s are in 1 of Self)
    type Scale: ConversionFactor;
}

impl<B: BaseUnitTag> BaseUnit for B {
    type Base = Self;
    type Scale = ConvInt<1>;
}

impl<B: BaseUnitTag> ConversionTo<B> for B {
    type Factor = ConvInt<1>;
}

pub struct ScaledBaseUnit<B, const N: u32, const D: u32 = 1> {
    base: PD<B>,
}

impl<B: BaseUnit, const N: u32, const D: u32> BaseUnit for ScaledBaseUnit<B, N, D>  {
    type Base = <B as BaseUnit>::Base;
    type Scale = ConvProd<ConvRatio<N,D>,<B as BaseUnit>::Scale>;
}

impl<B1: BaseUnit, const N: u32, const D: u32, B2: BaseUnitTag> ConversionTo<B2> for ScaledBaseUnit<B1, N, D>
where <B1 as BaseUnit>::Base: ConversionTo<B2> {
    type Factor = ConvProd<<Self as BaseUnit>::Scale, Conversion<<B1 as BaseUnit>::Base, B2>>;
}

impl<B1: BaseUnitTag, const N: u32, const D: u32, B2: BaseUnit> ConversionTo<ScaledBaseUnit<B2, N, D>> for B1
where B1: ConversionTo<<B2 as BaseUnit>::Base> {
    type Factor = ConvQuot<Conversion<B1, <B2 as BaseUnit>::Base>, <ScaledBaseUnit<B2, N, D> as BaseUnit>::Scale>;
}

impl<
    B1: BaseUnit, const N1: u32, const D1: u32, 
    B2: BaseUnit, const N2: u32, const D2: u32
> ConversionTo<ScaledBaseUnit<B2, N2, D2>> for ScaledBaseUnit<B1, N1, D1>
where
    B1: ConversionTo<B2>
{
    type Factor = ConvQuot<ConvProd<<Self as BaseUnit>::Scale, Conversion<B1,B2>>,<ScaledBaseUnit<B2, N2, D2> as BaseUnit>::Scale>;
}

pub type Info = &'static str;

pub trait BaseUnitInfo: BaseUnit {
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
    where U: ConversionTo<GetSystemUnit<U>> {
        Qnty::from_raw_value(value.convert::<Conversion<Self, GetSystemUnit<U>>>())
    }
}

/// Convert between base units of different systems
impl<Sys1, Dim1, Sys2, Dim2> ConversionTo<SystemUnit<Sys2, Dim2>> for SystemUnit<Sys1, Dim1>
where
    Sys1: UnitSystem,
    Dim1: Dim,
    Sys2: UnitSystem,
    Dim2: Dim,
    Dim1: SameDimension<Dim2>,
    GetBase<Sys1, MassBaseDimension>: ConversionTo<GetBase<Sys2, MassBaseDimension>>,
    ConvPow<<GetBase<Sys1, MassBaseDimension> as ConversionTo<GetBase<Sys2, MassBaseDimension>>>::Factor, GetDim<Dim1, MassBaseDimension>>: ConversionFactor,
    GetBase<Sys1, LengthBaseDimension>: ConversionTo<GetBase<Sys2, LengthBaseDimension>>,
    ConvPow<<GetBase<Sys1, LengthBaseDimension> as ConversionTo<GetBase<Sys2, LengthBaseDimension>>>::Factor, GetDim<Dim1, LengthBaseDimension>>: ConversionFactor,
    GetBase<Sys1, TimeBaseDimension>: ConversionTo<GetBase<Sys2, TimeBaseDimension>>,
    ConvPow<<GetBase<Sys1, TimeBaseDimension> as ConversionTo<GetBase<Sys2, TimeBaseDimension>>>::Factor, GetDim<Dim1, TimeBaseDimension>>: ConversionFactor,
{
    type Factor = 
    ConvProd<
        ConvPow<Conversion<GetBase<Sys1, MassBaseDimension>, GetBase<Sys2, MassBaseDimension>>, GetDim::<Dim1, MassBaseDimension>>,
    ConvProd<
        ConvPow<Conversion<GetBase<Sys1, LengthBaseDimension>, GetBase<Sys2, LengthBaseDimension>>, GetDim::<Dim1, LengthBaseDimension>>,
        ConvPow<Conversion<GetBase<Sys1, TimeBaseDimension>, GetBase<Sys2, TimeBaseDimension>>, GetDim::<Dim1, TimeBaseDimension>>
    >>;
}

/// Convert from a scaled unit to the base unit of a system (used with `Unit::new()`)
impl<U: Unit, const NUM: u32, const DEN: u32, S2: UnitSystem, D2: Dim> ConversionTo<SystemUnit<S2, D2>> for ScaledUnit<U, NUM, DEN>
where 
    U: ConversionTo<SystemUnit<S2, D2>>
{
    type Factor = ConvProd<ConvRatio<NUM, DEN>, Conversion<U, SystemUnit<S2, D2>>>;
}

/// Convert from the base unit of system to a scaled unit (used with [`Qnty::value`])
impl<U: Unit, const NUM: u32, const DEN: u32, S1: UnitSystem, D1: Dim> ConversionTo<ScaledUnit<U, NUM, DEN>> for SystemUnit<S1, D1>
where 
    SystemUnit<S1, D1>: ConversionTo<U>
{
    type Factor = ConvProd<ConvRatio<DEN, NUM>, Conversion<SystemUnit<S1, D1>, U>>;
}

/// Convert between scaled units
impl<U1, const NUM1: u32, const DEN1: u32,
     U2, const NUM2: u32, const DEN2: u32> ConversionTo<ScaledUnit<U2, NUM2, DEN2>> for ScaledUnit<U1, NUM1, DEN1>
where
    U1: ConversionTo<U2>
{
    type Factor = ConvProd<ConvRatio<NUM1,DEN1>, ConvProd<ConvRatio<DEN2,NUM2>,Conversion<U1, U2>>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::system::{
        Area,
        si::{System as SI, Meters, Kilometers, Centimeters, Seconds as SecondsSI, Hours as HoursSI}, 
        imperial::{System as Imp, Feet, Yards, Seconds as SecondsIMP, Hours as HoursIMP, Miles}
    };

    macro_rules! assert_conv {
        ($val1:literal $U1:ty = $val2:literal $U2:ty) => {
            approx::assert_relative_eq!(<Conversion::<$U1, $U2> as ConversionFactor>::REAL, $val2 as f64 / $val1 as f64, epsilon=f32::EPSILON as f64);
            approx::assert_relative_eq!(<Conversion::<$U2, $U1> as ConversionFactor>::REAL, $val1 as f64 / $val2 as f64, epsilon=f32::EPSILON as f64);
        };
    }

    #[test]
    fn base_unit_conversions() {
        use crate::base_unit::length::*;
        assert_conv!(1 FootBaseUnit = 1 FootBaseUnit);
        assert_conv!(1 MeterBaseUnit = 1 MeterBaseUnit);
        assert_conv!(3 FootBaseUnit = 0.9144 MeterBaseUnit);
        assert_conv!(1 YardBaseUnit = 0.9144 MeterBaseUnit);
    }

    #[test]
    fn simple_conversions() {
        assert_conv!(1 Kilometers = 1_000 Meters);
        assert_conv!(1 Meters = 1_000 Centimeters);

        assert_conv!(0.9144 Meters = 3.0 Feet);
        assert_conv!(0.9144 Meters = 1.0 Yards);
        assert_conv!(1 Yards = 3 Feet);
        assert_conv!(1 Miles = 5280 Feet);

        assert_conv!(1 HoursSI = 3_600 SecondsSI);
        assert_conv!(1 HoursIMP = 3_600 SecondsIMP);
        assert_conv!(1 HoursIMP = 3_600 SecondsSI);
        assert_conv!(1 HoursSI = 3_600 SecondsIMP);
    }

    #[test]
    fn nontrivial_conversions() {
        type SquareMeters = Area<SI>;
        type SquareFeet = Area<Imp>;
        assert_conv!(1.0 SquareMeters = 10.763910416 SquareFeet);
    }

    #[test]
    fn convert_self() {
        assert_conv!(1.0 Meters = 1.0 Meters);
        assert_conv!(1.0 Feet = 1.0 Feet);
    }
}
