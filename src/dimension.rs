use std::marker::PhantomData as PD;
use std::ops::{Add, Sub, Mul, Div};
use typenum::{
    array::{ATerm, TArr},
    consts::*,
    marker_traits::{Bit, TypeArray, Unsigned},
    operator_aliases::{Diff, Prod, Quot, Sub1, Sum},
    uint::UInt
};

/// A base dimension of mass, length, time, electrical current, etc.
/// 
/// The [`Ordinal`](BaseDimension::Ordinal) is used to index base dimension
/// exponents within a [`DimList`]
pub trait BaseDimension {
    type Ordinal: Unsigned;
}

pub struct MassBaseDimension;
impl BaseDimension for MassBaseDimension {
    type Ordinal = U0;
}

pub struct LengthBaseDimension;
impl BaseDimension for LengthBaseDimension {
    type Ordinal = U1;
}

pub struct TimeBaseDimension;
impl BaseDimension for TimeBaseDimension {
    type Ordinal = U2;
}

pub trait DimPart<D: BaseDimension> {
    type Exponent;
}

/// List of [`BaseDimension`] exponents, indexed by [`Ordinal`](BaseDimension::Ordinal).
pub type DimList<E, DL> = TArr<E, DL>;

/// Type Operator for accessing the `I`th element of a [`DimList`]. 
/// 
/// A list is searched in linear order, reducing the index `I` at each step and
/// continuing with the inner [`TArr`]. 
/// Once the index has reach [`U0`], we have found the item. 
pub trait Item<I> {
    type Output;
}

impl<V, A, U: Unsigned, B: Bit> Item<UInt<U, B>> for TArr<V, A>
where
    UInt<U, B>: std::ops::Sub<B1>,
    A: Item<Sub1<UInt<U, B>>>,
    Sub1<UInt<U, B>>: Unsigned
{
    type Output = <A as Item<Sub1<UInt<U, B>>>>::Output;
}

/// The search index has reached U0, so we've found our item. 
impl<V, A> Item<U0> for TArr<V, A> {
    type Output = V;
}

/// Type Alias for the `I`'th item of list `L`.
type GetItem<L, I> = <L as Item<I>>::Output;

/// Type representing no dimension where all exponents are 0. 
/// Since every [`TArr`] ends with [`ATerm`], having dimensionless
/// at the end will default all base dimensions to 0 if they are not
/// present in the [`DimList`]. 
pub type Dimensionless = ATerm;

/// [`Dimensionless`] (i.e. [`ATerm`]) is always [`Z0`]
impl<U> Item<U> for Dimensionless {
    type Output = Z0;
}

impl<BD: BaseDimension> DimPart<BD> for Dimensionless {
    type Exponent = Z0;
}

impl<BD: BaseDimension, DI, DL: TypeArray> DimPart<BD> for TArr<DI, DL>
where
    Self: Item<BD::Ordinal>
{
    type Exponent = GetItem<Self, BD::Ordinal>;
}

pub type GetDimPart<D, P> = <D as DimPart<P>>::Exponent;

#[cfg(test)]
mod dim_list {
    use super::*;
    use typenum::{assert_type_eq, tarr};
    #[test]
    fn get_item() {
        #[allow(dead_code)]
        type TA = tarr![P1, P2, P3];
        assert_type_eq!(GetItem<TA, U0>, P1);
        assert_type_eq!(GetItem<TA, U1>, P2);
        assert_type_eq!(GetItem<TA, U2>, P3);
    }

    #[test]
    fn dim_list() {
        #[allow(dead_code)]
        type DList = tarr![P1, P2];
        assert_type_eq!(GetDimPart<DList, MassBaseDimension>, P1);
        assert_type_eq!(GetDimPart<DList, LengthBaseDimension>, P2);

        // No Time dimension given, so defaults to Z0.
        assert_type_eq!(GetDimPart<DList, TimeBaseDimension>, Z0);
    }

    #[test]
    fn dim_list_operations() {
        #[allow(dead_code)]
        type Dim1 = tarr![P1, P2, P1];
        #[allow(dead_code)]
        type Dim2 = tarr![Z0, N1, N1];

        assert_type_eq!(Sum<Dim1, Dim2>, tarr![P1, P1, Z0]);

        assert_type_eq!(Diff<Dim1, Dim2>, tarr![P1, P3, P2]);

        assert_type_eq!(Prod<Dim1, P2>, tarr![P2, P4, P2]);
        assert_type_eq!(Prod<Dim2, P2>, tarr![Z0, N2, N2]);
    }
}

pub trait Dim:
    DimPart<MassBaseDimension> +
    DimPart<LengthBaseDimension> +
    DimPart<TimeBaseDimension> {}

#[derive(Debug)]
pub struct Dimension<M, L, T>(PD<M>,PD<L>,PD<T>);

impl<M, L, T> DimPart<MassBaseDimension> for Dimension<M, L, T> {
    type Exponent = M;
}

impl<M, L, T> DimPart<LengthBaseDimension> for Dimension<M, L, T> {
    type Exponent = L;
}

impl<M, L, T> DimPart<TimeBaseDimension> for Dimension<M, L, T> {
    type Exponent = T;
}

impl<M, L, T> Dim for Dimension<M, L, T> {}

pub trait SameDimension<D> {}

/// [`SameDimension`] if each exponent is the same throughout the list
impl<E, D1, D2: SameDimension<D1>> SameDimension<DimList<E, D2>> for DimList<E, D1> {}

impl<DL: SameDimension<Dimensionless>> SameDimension<Dimensionless> for TArr<Z0, DL> {}

impl SameDimension<Dimensionless> for Dimensionless {}

impl<D1: Dim, D2: Dim> SameDimension<D2> for D1
where
    D1: DimPart<MassBaseDimension, Exponent = <D2 as DimPart<MassBaseDimension>>::Exponent>,
    D1: DimPart<LengthBaseDimension, Exponent = <D2 as DimPart<LengthBaseDimension>>::Exponent>,
    D1: DimPart<TimeBaseDimension, Exponent = <D2 as DimPart<TimeBaseDimension>>::Exponent>,
{}

#[test]
fn same_dimension() {
    use typenum::tarr;
    use std::marker::PhantomData;

    fn assert_same_dimension<D1, D2: SameDimension<D1>>(_: PhantomData<D1>, _: PhantomData<D2>) {}
    assert_same_dimension::<tarr![P1, P2], tarr![P1, P2, Z0, Z0]>(PhantomData, PhantomData);
}

/// If we define with [`SameDimension`] then custom types that implement Dim can be used as well
impl<M, L, T, Other: SameDimension<Dimension<M, L, T>>> Add<Other> for Dimension<M, L, T> {
    type Output = Self;
    fn add(self, _rhs: Other) -> Self::Output {
        unimplemented!()
    }
}

impl<M, L, T> Sub for Dimension<M, L, T>
{
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<Ml, Ll, Tl, Mr, Lr, Tr> Mul<Dimension<Mr, Lr, Tr>> for Dimension<Ml, Ll, Tl>
where
    Ml: Add<Mr>,
    Ll: Add<Lr>,
    Tl: Add<Tr>,
{
    type Output = Dimension<Sum<Ml, Mr>, Sum<Ll, Lr>, Sum<Tl, Tr>>;
    fn mul(self, _: Dimension<Mr, Lr, Tr>) -> Self::Output {
        unimplemented!()
    }
}

impl<Ml, Ll, Tl, Mr, Lr, Tr> Div<Dimension<Mr, Lr, Tr>> for Dimension<Ml, Ll, Tl>
where
    Ml: Sub<Mr>,
    Ll: Sub<Lr>,
    Tl: Sub<Tr>,
{
    type Output = Dimension<Diff<Ml, Mr>, Diff<Ll, Lr>, Diff<Tl, Tr>>;
    fn div(self, _: Dimension<Mr, Lr, Tr>) -> Self::Output {
        unimplemented!()
    }
}

pub type MassDimension = Dimension<P1, Z0, Z0>;

pub type LengthDimension = Dimension<Z0, P1, Z0>;
pub type AreaDimension = Prod<LengthDimension, LengthDimension>;
pub type VolumeDimension = Prod<LengthDimension, AreaDimension>;

pub type TimeDimension = Dimension<Z0, Z0, P1>;

pub type VelocityDimension = Quot<LengthDimension, TimeDimension>;
