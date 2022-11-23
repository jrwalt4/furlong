use std::marker::PhantomData as PD;
use std::ops::{Mul, Div};
use typenum::{Integer, Prod, Quot};

use crate::dimension::*;
use crate::qnty::Qnty;
use crate::unit_system::*;

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

pub mod base_unit {
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

    pub type PoundMassBaseUnit = ScaledBaseUnit<SlugBaseUnit, 547, 17>; // 547/17 = 32.17647 ~ 32.1740
    impl BaseUnitInfo for PoundMassBaseUnit {
        const NAME: Info = "pound";
        const SYMBOL: Info = "lbm";
    }

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

    pub type MinuteBaseUnit = ScaledBaseUnit<SecondBaseUnit, 60>;
}

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dim;
}

pub(crate) type GetMassBase<U> = <<U as Unit>::System as UnitSystem>::Mass;
pub(crate) type GetMassDim<U> = <<U as Unit>::Dim as Dim>::Mass;

pub(crate) type GetLengthBase<U> = <<U as Unit>::System as UnitSystem>::Length;
pub(crate) type GetLengthDim<U> = <<U as Unit>::Dim as Dim>::Length;

pub(crate) type GetTimeBase<U> = <<U as Unit>::System as UnitSystem>::Time;
pub(crate) type GetTimeDim<U> = <<U as Unit>::Dim as Dim>::Time;

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug)]
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

impl<S: UnitSystem, D: Dim> Clone for SystemUnit<S, D> {
    fn clone(&self) -> Self {
        SystemUnit::new()
    }
}

impl<S: UnitSystem, D: Dim> Copy for SystemUnit<S, D> {}

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

macro_rules! impl_shorthand_ctor {
    ($($T:ty)*) => {
        $(
            impl<S: UnitSystem, D: Dim> Mul<SystemUnit<S, D>> for $T {
                type Output = Qnty<SystemUnit<S, D>, $T>;
                fn mul(self, _unit: SystemUnit<S, D>) -> Self::Output {
                    Qnty::<SystemUnit<S, D>, $T>::new(self)
                }
            }
        )*
    };
}

impl_shorthand_ctor!(f32 f64 i32);

pub trait UnitConversion<T = f64> {
    const FACTOR: T;

    fn convert(value: T) -> <T as Mul>::Output
    where T: Mul {
        value * Self::FACTOR
    }
}

macro_rules! power_n {
    ( $X:expr, $N:expr) => {{
        let mut x = $X;
        let mut y = 1.0;
        let mut n = $N;
        while n > 1 {
            match n % 2 {
                0 => {
                    x *= x;
                    n /= 2;
                },
                1 => {
                    y *= x;
                    x *= x;
                    n = (n-1)/2;
                },
                _ => {
                    unreachable!();
                }
            }
        }
        let f;
        if n == 0 {
            f = 1.0;
        } else {
            f = x * y;
        }
        f
    }};
}

pub struct Conversion<From, To, T=f64>(PD<From>,PD<To>, PD<T>);

impl<From, To: Unit> UnitConversion for Conversion<From, To>
where
    From: Unit<Dim = To::Dim>,
    GetMassBase<From>: BaseUnitConversion<GetMassBase<To>>,
    GetMassDim<From>: typenum::Integer,
    GetLengthBase<From>: BaseUnitConversion<GetLengthBase<To>>,
    GetLengthDim<From>: typenum::Integer,
    GetTimeBase<From>: BaseUnitConversion<GetTimeBase<To>>,
    GetTimeDim<From>: typenum::Integer
{
    const FACTOR: f64 = 
    power_n!(
        <GetMassBase<From> as BaseUnitConversion<GetMassBase<To>>>::FACTOR,
        <GetMassDim<From> as typenum::Integer>::I32
    ) * 
    power_n!(
        <GetLengthBase<From> as BaseUnitConversion<GetLengthBase<To>>>::FACTOR,
        <GetLengthDim<From> as typenum::Integer>::I32
    ) *
    power_n!(
        <GetTimeBase<From> as BaseUnitConversion<GetTimeBase<To>>>::FACTOR,
        <GetTimeDim<From> as typenum::Integer>::I32
    );
}
