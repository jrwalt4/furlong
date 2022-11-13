use std::fmt;
use std::marker::PhantomData as PD;
#[allow(unused)]
use std::ops::{Add, Div, Mul, Sub};
use typenum::*;

#[allow(unused)]
type Lcm<A, B> = Quot<AbsVal<Prod<A,B>>,Gcf<A,B>>;

pub mod marker {
    use typenum::{Integer, NonZero};

    pub trait RationalType {
        type Numer: Integer;
        type Denom: Integer + NonZero;
        type Reduced;
        const F32: f32 = (Self::Numer::I32 as f32)/(Self::Denom::I32 as f32);
        #[inline]
        fn to_f32() -> f32 {
            (Self::Numer::I32 as f32) / (Self::Denom::I32 as f32)
        }
    }
}
use marker::RationalType;
pub type Reduce<R> = <R as RationalType>::Reduced;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Rational<N, D> {
    numerator: PD<N>,
    denomenator: PD<D>
}

impl<D: Integer + NonZero> RationalType for Rational<Z0, D> {
    type Numer = Z0;
    type Denom = P1;
    type Reduced = Rational<Z0, P1>;
}

impl<U: Unsigned + NonZero, D: Integer + NonZero> RationalType for Rational<NInt<U>, D>
where
    NInt<U>: Gcd<D>,
    NInt<U>: PartialDiv<Gcf<NInt<U>, D>>,
    PartialQuot<NInt<U>, Gcf<NInt<U>, D>>: Integer,
    D: PartialDiv<Gcf<NInt<U>, D>>,
    PartialQuot<D, Gcf<NInt<U>, D>>: Integer + NonZero,
{
    type Numer = PartialQuot<NInt<U>, Gcf<NInt<U>, D>>;
    type Denom = PartialQuot<D, Gcf<NInt<U>, D>>;
    type Reduced = Rational<Self::Numer, Self::Denom>;
}

impl<U: Unsigned + NonZero, D: Integer + NonZero> RationalType for Rational<PInt<U>, D>
where
    PInt<U>: Gcd<D>,
    PInt<U>: PartialDiv<Gcf<PInt<U>, D>>,
    PartialQuot<PInt<U>, Gcf<PInt<U>, D>>: Integer,
    D: PartialDiv<Gcf<PInt<U>, D>>,
    PartialQuot<D, Gcf<PInt<U>, D>>: Integer + NonZero,
{
    type Numer = PartialQuot<PInt<U>, Gcf<PInt<U>, D>>;
    type Denom = PartialQuot<D, Gcf<PInt<U>, D>>;
    type Reduced = Rational<Self::Numer, Self::Denom>;
}

impl<N: Integer, D: Unsigned + NonZero, Rhs: RationalType> Cmp<Rhs> for Rational<N, D>
where
    Self: RationalType,
    <Self as RationalType>::Numer: IsEqual<Rhs::Numer, Output = True>,
    <Self as RationalType>::Denom: IsEqual<Rhs::Denom, Output = True>,
{
    type Output = Equal;
    fn compare<IM: private::InternalMarker>(&self, _: &Rhs) -> Self::Output {
        Equal
    }
}

impl<N: Integer, D: Integer + NonZero> fmt::Display for Rational<N, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", N::I32, D::I32)
    }
}

impl<Nl: Integer, Dl: Integer + NonZero, Rhs: RationalType> Add<Rhs> for Rational<Nl, Dl>
where
    Nl: Mul<Rhs::Denom>,
    Dl: Mul<Rhs::Numer>,
    Prod<Nl, Rhs::Denom>: Add<Prod<Dl, Rhs::Numer>>,
    Dl: Mul<Rhs::Denom>
{
    type Output = Rational<
                    Sum<
                        Prod<Nl, Rhs::Denom>,
                        Prod<Dl, Rhs::Numer>
                    >,
                    Prod<Dl, Rhs::Denom>
                >;
    fn add(self, _rhs: Rhs) -> Self::Output {
        unimplemented!()
    }
}


impl<Nl, Dl, Nr, Dr> Sub<Rational<Nr, Dr>> for Rational<Nl, Dl>
where
    Nl: Mul<Dr>,
    Nr: Mul<Dl>,
    Prod<Nl, Dr>: Sub<Prod<Nr, Dl>>,
    Dl: Mul<Dr>
{
    type Output = Rational<
                    Diff<
                        Prod<Nl, Dr>,
                        Prod<Nr, Dl>
                    >,
                    Prod<Dl, Dr>
                >;
    fn sub(self, _rhs:Rational<Nr, Dr>) -> Self::Output {
        Rational {
            numerator: PD,
            denomenator: PD
        }
    }
}

pub type FromInteger<I> = Rational<I, P1>;

#[cfg(test)]
mod rational {
    use super::*;
    fn assert_display<T: ToString + Default>(expect: &str) {
        assert_eq!(T::default().to_string(), expect);
    }
    #[test]
    fn to_f32() {
        assert_eq!(Rational::<P3, P2>::to_f32(), 1.5);
        assert_display::<Rational<P3, P2>>("3/2");
    }
    #[test]
    fn equal() {
        assert_type_eq!(Reduce<Rational<P3, P2>>, Reduce<Rational<P6, P4>>);
    }
    #[test]
    fn add() {
        type RationalSum = Sum<Rational<P3, P4>, Rational<P2, P3>>;
        assert_eq!(RationalSum::to_f32(), 17.0/12.0);
        assert_display::<RationalSum>("17/12");
    }
    #[test]
    fn sub() {
        type RationalDiff = Diff<Rational<P3, P4>, Rational<P2, P3>>;
        assert_eq!(RationalDiff::to_f32(), 1.0/12.0);
        assert_display::<RationalDiff>("1/12");
    }
    #[test]
    fn display() {
        let rat = Rational::<N2,P3>::default();
        assert_eq!(rat.to_string(), "-2/3");
        assert_eq!(format!("{}", rat), "-2/3");
    }
}
