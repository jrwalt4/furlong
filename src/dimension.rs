use std::marker::PhantomData as PD;
use std::ops::{Add, Sub, Mul, Div};
use typenum::*;

use crate::base_dimension::{BaseDimension, MassBaseDimension, LengthBaseDimension, TimeBaseDimension};

pub trait DimPart<D: BaseDimension> {
    type Exponent;
}

pub type GetDim<D, P> = <D as DimPart<P>>::Exponent;

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

impl<M, L, T> Add for Dimension<M, L, T>
{
    type Output = Self;
    fn add(self, _rhs: Self) -> Self::Output {
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
