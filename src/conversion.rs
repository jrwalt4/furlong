use std::{
    ops::{Mul, Div}
};
use typenum::*;

/// A value that can apply a conversion factor
pub trait Convertible: Sized {
    fn convert<C: ConversionFactor>(&self) -> Self;

}

macro_rules! impl_conv_float {
    ($T:ty) => {
        impl Convertible for $T {
            fn convert<C: ConversionFactor>(&self) -> Self {
                (*self as f64 * C::REAL) as Self
            }
        }
    };
    ($T:ty, $($Ts:ty),+) => {
        impl_conv_float!{$T}
        impl_conv_float!{$($Ts),+}
    }
}

impl_conv_float!{f32, f64, u32, i32, u64, i64}

pub trait ConversionTo<T> {
    type Factor: ConversionFactor;
}

pub type Conversion<From, To> = <From as ConversionTo<To>>::Factor;

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

pub type ConvRatio<const N: u32, const D: u32> = ConvQuot<ConvInt<N>, ConvInt<D>>;

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

pub struct ConvPow<C, N>(C, N);

macro_rules! power_n {
    ($X:expr, $N:expr, $T:ty) => {{
        let mut x = $X;
        let mut y:$T = 1 as $T;
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
            f = 1 as $T;
        } else {
            f = x * y;
        }
        f
    }};
    ($X:expr, $N:expr) => {{ power_n!($X, $N, f64) }};
}

impl<C> ConversionFactor for ConvPow<C, Z0> {
    const REAL: f64 = 1.0;
    const NUM: u32 = 1;
    const DEN: u32 = 1;
}

impl<C: ConversionFactor> ConversionFactor for ConvPow<C, P1> {
    const REAL: f64 = C::REAL;
    const NUM: u32 = C::NUM;
    const DEN: u32 = C::DEN;
}

impl<C: ConversionFactor, U: Unsigned, B1: Bit, B2: Bit> ConversionFactor for ConvPow<C, PInt<UInt<UInt<U, B1>,B2>>> {
    const REAL: f64 = power_n!(C::REAL, <UInt<UInt<U, B1>,B2> as Unsigned>::U32);
    const NUM: u32 = power_n!(C::NUM, <UInt<UInt<U, B1>,B2> as Unsigned>::U32, u32);
    const DEN: u32 = power_n!(C::DEN, <UInt<UInt<U, B1>,B2> as Unsigned>::U32, u32);
}

impl<C: ConversionFactor> ConversionFactor for ConvPow<C, N1> {
    const REAL: f64 = 1.0 / C::REAL;
    const NUM: u32 = C::DEN;
    const DEN: u32 = C::NUM;
}

impl<C: ConversionFactor, U: Unsigned, B1: Bit, B2: Bit> ConversionFactor for ConvPow<C, NInt<UInt<UInt<U, B1>,B2>>> {
    const REAL: f64 = 1.0 / power_n!(C::REAL, <UInt<UInt<U, B1>,B2> as Unsigned>::U32);
    const NUM: u32 = power_n!(C::DEN, <UInt<UInt<U, B1>,B2> as Unsigned>::U32, u32);
    const DEN: u32 = power_n!(C::NUM, <UInt<UInt<U, B1>,B2> as Unsigned>::U32, u32);
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn pow() {
        type P = ConvPow<ConvRatio<3,4>, P2>;
        assert_eq!(P::REAL, 9.0 / 16.0);
        assert_eq!(P::NUM, 9);
        assert_eq!(P::DEN, 16);

        type N = ConvPow<ConvRatio<2,3>, N2>;
        assert_eq!(N::REAL, 9.0 / 4.0);
        assert_eq!(N::NUM, 9);
        assert_eq!(N::DEN, 4);

        type Cube = ConvPow<ConvRatio<3,4>, P3>;
        assert_eq!(Cube::REAL, 27.0 / 64.0);
        assert_eq!(Cube::NUM, 27);
        assert_eq!(Cube::DEN, 64);
    }

    #[test]
    fn operations() {
        // Multiplication
        type P = ConvProd<ConvInt<2>, ConvInt<2>>;
        assert_type_eq!(P, Prod<ConvInt<2>, ConvInt<2>>);
        assert_eq!(P::REAL, 4.0);

        // Inversion
        type PInv = ConvRecip<P>;
        assert_eq!(PInv::REAL, 1.0 / P::REAL);

        // Compound multiplication
        type ABC = ConvProd<ConvProd<ConvRatio<1,2>, ConvRatio<3,4>>,ConvRatio<5,6>>;
        assert_eq!(ABC::REAL, 1.0 * 3.0 * 5.0 / (2.0 * 4.0 * 6.0));
    }
}
