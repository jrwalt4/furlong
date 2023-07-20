use typenum::*;

use crate::{mpl::*, tmap};

/// A base dimension of mass, length, time, electrical current, etc.
///
/// The [`Ordinal`](BaseDimension::Ordinal) is used as the key
/// for a dimension list.
pub trait BaseDimension {
    type Ordinal: Unsigned;
}

#[macro_export]
macro_rules! base_dim {
    ($D:ident, $Ord:ty) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $D;
        impl $crate::dimension::BaseDimension for $D {
            type Ordinal = $Ord;
        }
        impl<BD: $crate::dimension::BaseDimension> ::typenum::Cmp<BD> for $D
        where
            $Ord: Cmp<<BD as $crate::dimension::BaseDimension>::Ordinal>,
        {
            type Output = Compare<$Ord, <BD as $crate::dimension::BaseDimension>::Ordinal>;
            fn compare<IM: private::InternalMarker>(&self, _: &BD) -> Self::Output {
                unimplemented!()
            }
        }
    };
}

base_dim!(MassBase, U0);
base_dim!(LengthBase, U1);
base_dim!(TimeBase, U2);

pub type Dimensionless = TEnd;

pub trait SameDimension<D> {}

/// [`SameDimension`] if each exponent is the same throughout the list
impl<E, D1, D2: SameDimension<D1>> SameDimension<TMap<E, D2>> for TMap<E, D1> {}

impl SameDimension<Dimensionless> for Dimensionless {}

#[cfg(test)]
mod dim_list {
    use super::*;
    use crate::tmap;

    #[test]
    fn dim_list() {
        #[allow(dead_code)]
        type DList = tmap! {MassBase: P1, LengthBase: P2};
        assert_type_eq!(Entry<DList, MassBase>, P1);
        assert_type_eq!(Entry<DList, LengthBase>, P2);

        // No Time dimension given, so defaults to Z0.
        assert_type_eq!(EntryOr<DList, TimeBase, Z0>, Z0);
    }

    #[test]
    fn dim_list_operations() {
        #[allow(dead_code)]
        type Dim1 = tmap! {MassBase: P1, LengthBase: P2, TimeBase: P1};
        #[allow(dead_code)]
        type Dim2 = tmap! {MassBase: Z0, LengthBase: N1, TimeBase: N1};
        assert_type_eq!(Sum<Dim1, Dim2>, tmap!{MassBase: P1, LengthBase: P1, TimeBase: Z0});
        assert_type_eq!(Diff<Dim1, Dim2>, tmap!{MassBase: P1, LengthBase: P3, TimeBase: P2});
        assert_type_eq!(Prod<Dim1, P2>, tmap!{MassBase: P2, LengthBase: P4, TimeBase: P2});
        assert_type_eq!(Prod<Dim2, P2>, tmap!{MassBase: Z0, LengthBase: N2, TimeBase: N2});
    }

    #[test]
    fn same_dimension() {
        use std::marker::PhantomData;
        fn assert_same_dimension<D1, D2: SameDimension<D1>>(
            _: PhantomData<D1>,
            _: PhantomData<D2>,
        ) {
        }
        assert_same_dimension::<
            tmap! {MassBase: P1, LengthBase: P2},
            tmap! {MassBase: P1, LengthBase: P2},
        >(PhantomData, PhantomData);
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
> = Sorted<tmap! {
    MassBase: Mass,
    LengthBase: Length,
    TimeBase: Time,
    // Current,
    // Temperature,
    // Light,
    // Amount
}>;

pub type MassDimension = Dimension<P1, Z0, Z0>;

pub type LengthDimension = Dimension<Z0, P1, Z0>;
pub type AreaDimension = Prod<LengthDimension, P2>;
pub type VolumeDimension = Prod<LengthDimension, P3>;

pub type TimeDimension = Dimension<Z0, Z0, P1>;

pub type VelocityDimension = Diff<LengthDimension, TimeDimension>;
