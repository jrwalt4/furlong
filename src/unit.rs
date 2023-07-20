use std::marker::PhantomData as PD;
use std::ops::{Add, Div, Mul, Sub};
use typenum::{Cmp, Compare, Diff, Equal, Exp, Less, Pow, Sum};

use crate::{
    conversion::*,
    dimension::*,
    mpl::*,
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
    type System;
    type Dim;
}

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

pub struct SystemUnit<S, D> {
    system: PD<S>,
    dimension: PD<D>,
}

impl<S, D> SystemUnit<S, D> {
    pub fn new<T>(value: T) -> Qnty<Self, T> {
        Qnty::from_raw_value(value)
    }
}

impl<S, D> Unit for SystemUnit<S, D> {
    type System = S;
    type Dim = D;
}

#[cfg(fmt)]
impl<S, D> UnitInfo for SystemUnit<S, D>
where
    S: UnitSystemPart<MassBaseDimension> + UnitSystemPart<LengthBaseDimension> + UnitSystemPart<TimeBaseDimension>,
    D: DimPart<MassBaseDimension> + DimPart<LengthBaseDimension> + DimPart<TimeBaseDimension>,
    GetBase<S, MassBaseDimension>: BaseUnitInfo,
    GetBase<S, LengthBaseDimension>: BaseUnitInfo,
    GetBase<S, TimeBaseDimension>: BaseUnitInfo,
    GetDimPart<D, MassBaseDimension>: Integer,
    GetDimPart<D, LengthBaseDimension>: Integer,
    GetDimPart<D, TimeBaseDimension>: Integer,
{
    fn abbr() -> String {
        let mass_abbr = <GetBase<S, MassBaseDimension> as BaseUnitInfo>::SYMBOL;
        let mass_pwr = <GetDimPart<D, MassBaseDimension> as Integer>::I8;
        let mass_part = match mass_pwr {
            0 => String::from(""),
            1 => String::from(mass_abbr),
            _ => format!("{}^{}", mass_abbr, mass_pwr),
        };

        let length_abbr = <GetBase<S, LengthBaseDimension> as BaseUnitInfo>::SYMBOL;
        let length_pwr = <GetDimPart<D, LengthBaseDimension> as Integer>::I8;
        let length_part = match length_pwr {
            0 => String::from(""),
            1 => String::from(length_abbr),
            _ => format!("{}^{}", length_abbr, length_pwr),
        };

        let time_abbr = <GetBase<S, TimeBaseDimension> as BaseUnitInfo>::SYMBOL;
        let time_pwr = <GetDimPart<D, TimeBaseDimension> as Integer>::I8;
        let time_part = match time_pwr {
            0 => String::from(""),
            1 => String::from(time_abbr),
            _ => format!("{}^{}", time_abbr, time_pwr),
        };
        format!("{}{}{}", mass_part.as_str(), length_part.as_str(), time_part.as_str())
    }
}

impl<S, D, Ur: Unit> Mul<Ur> for SystemUnit<S, D>
where
    D: Add<<Ur as Unit>::Dim>,
{
    type Output = SystemUnit<S, Sum<D, <Ur as Unit>::Dim>>;

    fn mul(self, _: Ur) -> Self::Output {
        unimplemented!()
    }
}

impl<S, D, Ur: Unit> Div<Ur> for SystemUnit<S, D>
where
    D: Sub<<Ur as Unit>::Dim>,
{
    type Output = SystemUnit<S, Diff<D, <Ur as Unit>::Dim>>;

    fn div(self, _: Ur) -> Self::Output {
        unimplemented!();
    }
}

type AsSystemUnit<U> = SystemUnit<<U as Unit>::System, <U as Unit>::Dim>;

pub struct ScaledUnit<U, const NUM: u32, const DEN: u32 = 1> {
    _unit: U,
}

impl<U: Unit, const NUM: u32, const DEN: u32> Unit for ScaledUnit<U, NUM, DEN> {
    type System = <U as Unit>::System;
    type Dim = <U as Unit>::Dim;
}

impl<U: Unit, const NUM: u32, const DEN: u32> ScaledUnit<U, NUM, DEN> {
    pub fn new<T: Convertible>(value: T) -> Qnty<Self, T>
    where
        U: ConversionTo<AsSystemUnit<U>>,
    {
        Qnty::from_raw_value(value.convert::<Conversion<Self, AsSystemUnit<U>>>())
    }
}

/// Private helper for calculating conversion factor for individual base dimensions
/// within a system/dimension pair.
pub trait SystemConversionTo<Sys, Dim> {
    type Output;
}

pub type SystemConversion<From, To, Dim> = <From as SystemConversionTo<To, Dim>>::Output;

/// Private-Private helper for calculating conversion factor for individual base dimensions
/// within a system/dimension pair.
///
/// Sys = System converting to
///
/// Dim = Exponent of dimension
///
/// C = Comparison of base dimension keys in the Sys and Dimension parameters
/// (so we know we're using the correct base)
pub trait InnerSystemConversionTo<Sys, Dim, C> {
    type Output;
}

pub type InnerSystemConversion<From, To, Dim, C> =
    <From as InnerSystemConversionTo<To, Dim, C>>::Output;

impl<El, Ml, Er, Mr, Ed, Md> SystemConversionTo<TMap<Er, Mr>, TMap<Ed, Md>> for TMap<El, Ml>
where
    El: TypeMapEntry,
    Ed: TypeMapEntry,
    TKey<El>: Cmp<TKey<Ed>>,
    TMap<El, Ml>: InnerSystemConversionTo<TMap<Er, Mr>, TMap<Ed, Md>, Compare<TKey<El>, TKey<Ed>>>,
{
    type Output = InnerSystemConversion<
        TMap<El, Ml>,
        TMap<Er, Mr>,
        TMap<Ed, Md>,
        Compare<TKey<El>, TKey<Ed>>,
    >;
}

impl SystemConversionTo<TEnd, TEnd> for TEnd {
    type Output = ConvInt<1>;
}

impl<El, Ml, Er, Mr> SystemConversionTo<TMap<Er, Mr>, TEnd> for TMap<El, Ml> {
    type Output = ConvInt<1>;
}

/// Key of dimension list matches key of base unit list, so apply exponent to conversion
impl<El, Ml, Er, Mr, Ed, Md> InnerSystemConversionTo<TMap<Er, Mr>, TMap<Ed, Md>, Equal>
    for TMap<El, Ml>
where
    El: TypeMapEntry,
    Er: TypeMapEntry,
    Ed: TypeMapEntry,
    TVal<El>: ConversionTo<TVal<Er>>,
    Conversion<TVal<El>, TVal<Er>>: Pow<TVal<Ed>>,
    Ml: SystemConversionTo<Mr, Md>,
    ConvProd<Exp<Conversion<TVal<El>, TVal<Er>>, TVal<Ed>>, SystemConversion<Ml, Mr, Md>>:
        ConversionFactor,
{
    type Output =
        ConvProd<Exp<Conversion<TVal<El>, TVal<Er>>, TVal<Ed>>, SystemConversion<Ml, Mr, Md>>;
}

/// We have an entry in the system unit list that doesn't existing in the dimension, keep looking
impl<El, Ml, Er, Mr, Ed, Md> InnerSystemConversionTo<TMap<Er, Mr>, TMap<Ed, Md>, Less>
    for TMap<El, Ml>
where
    Ml: SystemConversionTo<Mr, TMap<Ed, Md>>,
{
    type Output = SystemConversion<Ml, Mr, TMap<Ed, Md>>;
}

impl<Sys1, Dim1, Sys2, Dim2> ConversionTo<SystemUnit<Sys2, Dim2>> for SystemUnit<Sys1, Dim1>
where
    Dim1: SameDimension<Dim2>,
    Sys1: SystemConversionTo<Sys2, Dim1>,
    SystemConversion<Sys1, Sys2, Dim1>: ConversionFactor,
{
    type Factor = SystemConversion<Sys1, Sys2, Dim1>;
}

/// Convert from a scaled unit to the base unit of a system (used with `Unit::new()`)
impl<U: Unit, const NUM: u32, const DEN: u32, S2, D2> ConversionTo<SystemUnit<S2, D2>> for ScaledUnit<U, NUM, DEN>
where 
    U: ConversionTo<SystemUnit<S2, D2>>
{
    type Factor = ConvProd<ConvRatio<NUM, DEN>, Conversion<U, SystemUnit<S2, D2>>>;
}

/// Convert from the base unit of system to a scaled unit (used with [`Qnty::value`])
impl<U: Unit, const NUM: u32, const DEN: u32, S1, D1> ConversionTo<ScaledUnit<U, NUM, DEN>> for SystemUnit<S1, D1>
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
            approx::assert_relative_eq!(
                <Conversion::<$U1, $U2> as ConversionFactor>::REAL,
                $val2 as f64 / $val1 as f64,
                epsilon = f32::EPSILON as f64
            );
            approx::assert_relative_eq!(
                <Conversion::<$U2, $U1> as ConversionFactor>::REAL,
                $val1 as f64 / $val2 as f64,
                epsilon = f32::EPSILON as f64
            );
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
    fn conversion_impl() {
        assert_eq!(<<SI as SystemConversionTo<SI, LengthDimension>>::Output as ConversionFactor>::REAL, 1.0);
    }

    #[test]
    fn simple_conversions() {
        assert_conv!(1 Meters = 1 Meters);        
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
