use std::marker::PhantomData as PD;
use std::ops::{Mul, Div};
use std::cmp::{PartialEq, PartialOrd, Ordering};
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

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug, Copy, Clone)]
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

pub struct ScaledUnit<U, const NUM: u32, const DEN: u32 = 1> {
    unit: PD<U>
}

impl<S: UnitSystem, D: Dim, const NUM: u32, const DEN: u32> Unit for ScaledUnit<SystemUnit<S, D>, NUM, DEN> {
    type System = S;
    type Dim = D;
}

impl<S: UnitSystem, D: Dim, const NUM: u32, const DEN: u32> ScaledUnit<SystemUnit<S, D>, NUM, DEN> {
    pub fn new<T: Convertible>(value: T) -> Qnty<Self, T> {
        Qnty::from_raw_value(value.convert::<Conversion<Self, SystemUnit<S, D>>>())
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

impl<Sys1, Dim1, Sys2, Dim2> UnitConversion for Conversion<SystemUnit<Sys1, Dim1>, SystemUnit<Sys2, Dim2>>
where
    Sys1: UnitSystem,
    Dim1: Dim,
    Sys2: UnitSystem,
    Dim2: Dim,
    Dim1: SameDimension<Dim2>,
    GetBase<Sys1, MassBaseDimension>: BaseUnitConversion<GetBase<Sys2, MassBaseDimension>>,
    GetDim<Dim1, MassBaseDimension>: typenum::Integer,
    GetBase<Sys1, LengthBaseDimension>: BaseUnitConversion<GetBase<Sys2, LengthBaseDimension>>,
    GetDim<Dim1, LengthBaseDimension>: typenum::Integer,
    GetBase<Sys1, TimeBaseDimension>: BaseUnitConversion<GetBase<Sys2, TimeBaseDimension>>,
    GetDim<Dim1, TimeBaseDimension>: typenum::Integer,
{
    const SCALE: f64 = 
    power_n!(
        <GetBase<Sys1, MassBaseDimension> as BaseUnitConversion<GetBase<Sys2, MassBaseDimension>>>::SCALE,
        <GetDim<Dim1, MassBaseDimension> as typenum::Integer>::I32
    ) * 
    power_n!(
        <GetBase<Sys1, LengthBaseDimension> as BaseUnitConversion<GetBase<Sys2, LengthBaseDimension>>>::SCALE,
        <GetDim<Dim1, LengthBaseDimension> as typenum::Integer>::I32
    ) *
    power_n!(
        <GetBase<Sys1, TimeBaseDimension> as BaseUnitConversion<GetBase<Sys2, TimeBaseDimension>>>::SCALE,
        <GetDim<Dim1, TimeBaseDimension> as typenum::Integer>::I32
    );
}

impl<S: UnitSystem, D: Dim, const NUM: u32, const DEN: u32> UnitConversion 
for Conversion<ScaledUnit<SystemUnit<S, D>, NUM, DEN>, SystemUnit<S, D>> {
    const SCALE: f64 = NUM as f64 / DEN as f64;
}

impl<S: UnitSystem, D: Dim, const NUM: u32, const DEN: u32> UnitConversion 
for Conversion<SystemUnit<S, D>, ScaledUnit<SystemUnit<S, D>, NUM, DEN>> {
    const SCALE: f64 = DEN as f64 / NUM as f64;
}

/// A value that can apply a conversion factor
pub trait Convertible: Sized {
    fn convert<C: UnitConversion>(&self) -> Self;

    /// in place conversion
    fn convert_mut<C: UnitConversion>(&mut self) {
        *self = self.convert::<C>();
    }
}

pub trait UnitOrd<Rhs = Self> {
    fn cmp<C: UnitConversion>(&self, other: &Rhs) -> Option<Ordering>;
}

pub trait UnitEq<Rhs = Self> {
    fn eq<C: UnitConversion>(&self, other: &Rhs) -> bool;
}

macro_rules! impl_conv_float {
    ($T:ty) => {
        impl Convertible for $T {
            fn convert<C: UnitConversion>(&self) -> Self {
                (*self as f64 * C::SCALE) as Self
            }
        }
        impl<U: PartialOrd<$T>> UnitOrd<U> for $T {
            fn cmp<C: UnitConversion>(&self, other: &U) -> Option<Ordering> {
                other.partial_cmp(&((*self as f64 * <C as UnitConversion>::SCALE) as $T))
            }
        }
        impl<U: PartialEq<$T>> UnitEq<U> for $T {
            fn eq<C: UnitConversion>(&self, other: &U) -> bool {
                other.eq(&((*self as f64 * <C as UnitConversion>::SCALE) as $T))
            }
        }
    };
    ($T:ty, $($Ts:ty),+) => {
        impl_conv_float!{$T}
        impl_conv_float!{$($Ts),+}
    }
}

impl_conv_float!{f32, f64, u32, i32, u64, i64}
