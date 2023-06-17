use typenum::{
    array::{ATerm, TArr},
    consts::*,
    marker_traits::{Bit, Unsigned},
    operator_aliases::{Diff, Prod, Sub1},
    uint::UInt,
    tarr
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
pub(crate) type GetItem<L, I> = <L as Item<I>>::Output;

pub(crate) trait TypeIter {
    type Next;
}

impl<V, A> TypeIter for TArr<V, A> {
    type Next = A;
}

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

impl<BD: BaseDimension, DI, DL> DimPart<BD> for TArr<DI, DL>
where
    Self: Item<BD::Ordinal>
{
    type Exponent = GetItem<Self, BD::Ordinal>;
}

pub type GetDimPart<D, P> = <D as DimPart<P>>::Exponent;

pub trait SameDimension<D> {}

/// [`SameDimension`] if each exponent is the same throughout the list
impl<E, D1, D2: SameDimension<D1>> SameDimension<DimList<E, D2>> for DimList<E, D1> {}

impl<DL: SameDimension<Dimensionless>> SameDimension<Dimensionless> for TArr<Z0, DL> {}

impl SameDimension<Dimensionless> for Dimensionless {}

#[cfg(test)]
mod dim_list {
    use super::*;
    use typenum::*;
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

    #[test]
    fn same_dimension() {
        use std::marker::PhantomData;
        fn assert_same_dimension<D1, D2: SameDimension<D1>>(_: PhantomData<D1>, _: PhantomData<D2>) {}
        assert_same_dimension::<tarr![P1, P2], tarr![P1, P2, Z0, Z0]>(PhantomData, PhantomData);
    }
}

pub type Dimension<
    Mass=Z0, 
    Length=Z0, 
    Time=Z0, 
    // Current=Z0, 
    // Temperature=Z0, 
    // Light=Z0, 
    // Amount=Z0
> = tarr![
    Mass, 
    Length, 
    Time, 
    // Current, 
    // Temperature, 
    // Light, 
    // Amount
];

pub type MassDimension = Dimension<P1, Z0, Z0>;

pub type LengthDimension = Dimension<Z0, P1, Z0>;
pub type AreaDimension = Prod<LengthDimension, P2>;
pub type VolumeDimension = Prod<LengthDimension, P3>;

pub type TimeDimension = Dimension<Z0, Z0, P1>;

pub type VelocityDimension = Diff<LengthDimension, TimeDimension>;
