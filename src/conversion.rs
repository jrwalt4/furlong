use std::marker::PhantomData as PD;

use crate::{
    dimension::*,
    unit::*, 
};

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

/// Convert between base units of different systems
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

/// Convert from a scaled unit to the base unit of a system (used with `Unit::new()`)
impl<U: Unit, const NUM: u32, const DEN: u32, S2: UnitSystem, D2: Dim> UnitConversion
for Conversion<ScaledUnit<U, NUM, DEN>, SystemUnit<S2, D2>>
where 
    Conversion<U, SystemUnit<S2, D2>>: UnitConversion
{
    const SCALE: f64 = NUM as f64 / DEN as f64 * Conversion::<U, SystemUnit<S2, D2>>::SCALE;
}

/// Convert from the base unit of system to a scaled unit (used with [`Qnty::value`])
impl<U: Unit, const NUM: u32, const DEN: u32, S1: UnitSystem, D1: Dim> UnitConversion
for Conversion<SystemUnit<S1, D1>, ScaledUnit<U, NUM, DEN>>
where 
    Conversion<SystemUnit<S1, D1>, U>: UnitConversion
{
    const SCALE: f64 = DEN as f64 / NUM as f64 * Conversion::<SystemUnit<S1, D1>, U>::SCALE;
}

/// Convert between scaled units
impl<U1, const NUM1: u32, const DEN1: u32,
     U2, const NUM2: u32, const DEN2: u32> UnitConversion 
for Conversion<ScaledUnit<U1, NUM1, DEN1>, ScaledUnit<U2, NUM2, DEN2>>
where
    Conversion<U1, U2>: UnitConversion
{
    const SCALE: f64 = NUM1 as f64 / DEN1 as f64
                    * Conversion::<U1, U2>::SCALE
                    * DEN2 as f64 / NUM2 as f64;
}

/// A value that can apply a conversion factor
pub trait Convertible: Sized {
    fn convert<C: UnitConversion>(&self) -> Self;

    /// in place conversion
    fn convert_mut<C: UnitConversion>(&mut self) {
        *self = self.convert::<C>();
    }
}

macro_rules! impl_conv_float {
    ($T:ty) => {
        impl Convertible for $T {
            fn convert<C: UnitConversion>(&self) -> Self {
                (*self as f64 * C::SCALE) as Self
            }
        }
    };
    ($T:ty, $($Ts:ty),+) => {
        impl_conv_float!{$T}
        impl_conv_float!{$($Ts),+}
    }
}

impl_conv_float!{f32, f64, u32, i32, u64, i64}

#[cfg(test)]
mod test {
    use super::*;
    use crate::system::{
        si::{Meters, Kilometers, Centimeters, Seconds as SecondsSI, Hours as HoursSI}, 
        imperial::{Feet, Yards, Seconds as SecondsIMP, Hours as HoursIMP, Miles}
    };

    macro_rules! assert_conv {
        ($val1:literal $U1:ty = $val2:literal $U2:ty) => {
            approx::assert_relative_eq!(<Conversion::<$U1, $U2> as UnitConversion>::SCALE, $val2 as f64 / $val1 as f64);
            approx::assert_relative_eq!(<Conversion::<$U2, $U1> as UnitConversion>::SCALE, $val1 as f64 / $val2 as f64);
        };
    }

    #[test]
    fn simple_conversions() {
        assert_conv!(1 Kilometers= 1_000 Meters);
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
        assert_conv!(1 Kilometers = 1_000 Meters);
    }

    #[test]
    fn convert_self() {
        assert_conv!(1.0 Meters = 1.0 Meters);
        assert_conv!(1.0 Feet = 1.0 Feet);
    }
}
