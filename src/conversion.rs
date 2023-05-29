use std::{
    marker::PhantomData as PD,
    ops::{Mul, Div}
};

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

pub trait ConversionTo<T> {
    type Factor: ConversionFactor;
}

pub trait ConversionFactor {
    /// Conversion factor expressed as a floating point number
    const REAL: f64;

    /// Numerator of this conversion factor expressed as a fraction
    const NUM: u32;

    /// Denomenator of this conversion factor expressed as a fraction
    const DEN: u32;
}

pub struct ConvInt<const I: u32 = 1>;

impl<const I: u32> ConversionFactor for ConvInt<I> {
    const REAL: f64 = I as f64;
    const NUM: u32 = I;
    const DEN: u32 = 1;
}

impl<const I: u32, F: ConversionFactor> Mul<F> for ConvInt<I> {
    type Output = ConvProd<Self, F>;
    fn mul(self, _rhs: F) -> Self::Output {
        unimplemented!()
    }
}

impl<const I: u32, F: ConversionFactor> Div<F> for ConvInt<I> {
    type Output = ConvQuot<Self, F>;
    fn div(self, _rhs: F) -> Self::Output {
        unimplemented!()
    }
}

pub struct ConvRecip<C>(C);

impl<C: ConversionFactor> ConversionFactor for ConvRecip<C> {
    const REAL: f64 = 1.0 / C::REAL;
    const NUM: u32 = C::DEN;
    const DEN: u32 = C::NUM;
}

pub struct ConvRatio<const N: u32, const D: u32>;

impl<const N: u32, const D: u32> ConversionFactor for ConvRatio<N, D> {
    const REAL: f64 = N as f64 / D as f64;

    const NUM: u32 = N;

    const DEN: u32 = D;
}

impl<const N: u32, const D: u32, F: ConversionFactor> Mul<F> for ConvRatio<N, D> {
    type Output = ConvProd<Self, F>;
    fn mul(self, _rhs: F) -> Self::Output {
        unimplemented!()
    }
}

impl<const N: u32, const D: u32, F: ConversionFactor> Div<F> for ConvRatio<N, D> {
    type Output = ConvQuot<Self, F>;
    fn div(self, _rhs: F) -> Self::Output {
        unimplemented!()
    }
}

pub struct ConvProd<A, B>(A, B);

impl<A: ConversionFactor, B: ConversionFactor> ConversionFactor for ConvProd<A, B> {
    const REAL: f64 = A::REAL * B::REAL;
    const NUM: u32 = A::NUM * B::NUM;
    const DEN: u32 = A::DEN * B::DEN;
}

pub struct ConvQuot<A, B>(A, B);

impl<A: ConversionFactor, B: ConversionFactor> ConversionFactor for ConvQuot<A, B> {
    const REAL: f64 = A::REAL / B::REAL;
    const NUM: u32 = A::NUM * B::DEN;
    const DEN: u32 = A::DEN * B::NUM;
}

pub struct ConvPow<C, const N: u32>(C);

impl<C: ConversionFactor, const N: u32> ConversionFactor for ConvPow<C, N> {
    const REAL: f64 = power_n!(C::REAL, N);
    const NUM: u32 = u32::pow(C::NUM, N);
    const DEN: u32 = u32::pow(C::DEN, N);
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::*;

    use crate::conversion::{ConvInt, ConvProd};
    #[test]
    fn conversion_factor() {
        type Half = ConvRatio<1,2>;
        assert_eq!(Half::REAL, 0.5);
        assert_eq!(Half::NUM, 1);
        assert_eq!(Half::DEN, 2);

        type FiveQuarters = ConvRatio<5,4>;
        assert_eq!(FiveQuarters::REAL, 1.25);
        assert_eq!(FiveQuarters::NUM, 5);
        assert_eq!(FiveQuarters::DEN, 4);

        assert_eq!(<ConvProd<ConvProd<ConvRatio<1,3>, ConvRatio<3, 1>>, ConvInt<2>> as ConversionFactor>::REAL, 2.0);

        type Recip = ConvRecip<ConvInt<2>>;
        assert_eq!(<Recip as ConversionFactor>::REAL, 0.5);
        assert_eq!(<Recip as ConversionFactor>::NUM, 1);
        assert_eq!(<Recip as ConversionFactor>::DEN, 2);
    }

    #[test]
    fn operations() {
        type P = ConvProd<ConvInt<2>, ConvInt<2>>;
        assert_type_eq!(P, Prod<ConvInt<2>, ConvInt<2>>);
        assert_eq!(P::REAL, 4.0);
    }
}
