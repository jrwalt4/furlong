use std::marker::PhantomData as PD;
use std::ops::{Mul, Div};
use typenum::{Integer, Prod, Quot};

use crate::base_dimension::*;
use crate::dimension::*;
use crate::base_unit::*; 
use crate::qnty::Qnty;
use crate::unit_system::*;

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dim;
}

pub(crate) type GetUnitBase<U, D> = GetBase<<U as Unit>::System, D>;
pub(crate) type GetUnitDim<U, D> = GetDim<<U as Unit>::Dim, D>;

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug)]
pub struct SystemUnit<S: UnitSystem, D: Dim> {
    system: PD<S>,
    dimension: PD<D>,
}

impl<S: UnitSystem, D: Dim> SystemUnit<S, D> {
    pub fn new<T>(value: T) -> Qnty<Self, T> {
        Qnty::new(value)
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

pub trait UnitConversion {
    const SCALE: f64;
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

impl<From, To> UnitConversion for Conversion<From, To>
where
    To: Unit,
    From: Unit,
    <From as Unit>::Dim: SameDimension<<To as Unit>::Dim>,
    GetUnitBase<From, MassBaseDimension>: BaseUnitConversion<GetUnitBase<To, MassBaseDimension>>,
    GetUnitDim<From, MassBaseDimension>: typenum::Integer,
    GetUnitBase<From, LengthBaseDimension>: BaseUnitConversion<GetUnitBase<To, LengthBaseDimension>>,
    GetUnitDim<From, LengthBaseDimension>: typenum::Integer,
    GetUnitBase<From, TimeBaseDimension>: BaseUnitConversion<GetUnitBase<To, TimeBaseDimension>>,
    GetUnitDim<From, TimeBaseDimension>: typenum::Integer
{
    const SCALE: f64 = 
    power_n!(
        <GetUnitBase<From, MassBaseDimension> as BaseUnitConversion<GetUnitBase<To, MassBaseDimension>>>::SCALE,
        <GetUnitDim<From, MassBaseDimension> as typenum::Integer>::I32
    ) * 
    power_n!(
        <GetUnitBase<From, LengthBaseDimension> as BaseUnitConversion<GetUnitBase<To, LengthBaseDimension>>>::SCALE,
        <GetUnitDim<From, LengthBaseDimension> as typenum::Integer>::I32
    ) *
    power_n!(
        <GetUnitBase<From, TimeBaseDimension> as BaseUnitConversion<GetUnitBase<To, TimeBaseDimension>>>::SCALE,
        <GetUnitDim<From, TimeBaseDimension> as typenum::Integer>::I32
    );
}
