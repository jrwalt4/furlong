use std::marker::PhantomData as PD;
use std::ops::{Add, Mul};
use typenum::*;

pub trait Dimension {
    type Mass;
    type Length;
    type Time;
}

#[derive(Debug, Copy, Clone)]
pub struct MakeDimension<M, L, T> {
    mass_dim: PD<M>,
    length_dim: PD<L>,
    time_dim: PD<T>,
}

impl<M, L, T> MakeDimension<M, L, T> {
    fn new() -> MakeDimension<M, L, T> {
        MakeDimension {
            mass_dim: PD,
            length_dim: PD,
            time_dim: PD,
        }
    }
}

impl<M, L, T> Dimension for MakeDimension<M, L, T> {
    type Mass = M;
    type Length = L;
    type Time = T;
}

impl<M, L, T, Dr> PartialEq<Dr> for MakeDimension<M, L,T>
where
    Dr: Dimension<Mass = M, Length = L, Time = T>, {
        fn eq(&self, _other: &Dr) -> bool {
            true
        }
}

impl<Ml, Ll, Tl, Mr, Lr, Tr> Mul<MakeDimension<Mr, Lr, Tr>> for MakeDimension<Ml, Ll, Tl>
where
    Ml: Integer + Add<Mr>,
    Ll: Integer + Add<Lr>,
    Tl: Integer + Add<Tr>,
    Mr: Integer,
    Lr: Integer,
    Tr: Integer,
{
    type Output = MakeDimension<Sum<Ml, Mr>, Sum<Ll, Lr>, Sum<Tl, Tr>>;
    fn mul(self, _: MakeDimension<Mr, Lr, Tr>) -> Self::Output {
        MakeDimension::new()
    }
}

pub type ProdDimension<Dl, Dr> = <Dl as Mul<Dr>>::Output;

pub type MassDimension = MakeDimension<P1, Z0, Z0>;
pub type LengthDimension = MakeDimension<Z0, P1, Z0>;
pub type AreaDimension = MakeDimension<Z0, P2, Z0>;
pub type TimeDimension = MakeDimension<Z0, Z0, P1>;
