use std::marker::PhantomData as PD;
use std::ops::{Add, Mul};
use typenum::*;

pub trait Dimension {
    type Length;
    type Time;
}

#[derive(Debug, Copy, Clone)]
pub struct MakeDimension<L, T> {
    length_dim: PD<L>,
    time_dim: PD<T>,
}

impl<L, T> MakeDimension<L, T> {
    fn new() -> MakeDimension<L, T> {
        MakeDimension {
            length_dim: PD,
            time_dim: PD,
        }
    }
}

impl<L, T> Dimension for MakeDimension<L, T> {
    type Length = L;
    type Time = T;
}

impl<L, T, Dr> PartialEq<Dr> for MakeDimension<L,T> 
where
    Dr: Dimension<Length = L, Time = T>, {
        fn eq(&self, _other: &Dr) -> bool {
            true
        }
}

impl<Ll, Tl, Lr, Tr> Mul<MakeDimension<Lr, Tr>> for MakeDimension<Ll, Tl>
where
    Ll: Integer + Add<Lr>,
    Tl: Integer + Add<Tr>,
    Lr: Integer,
    Tr: Integer,
{
    type Output = MakeDimension<Sum<Ll, Lr>, Sum<Tl, Tr>>;
    fn mul(self, _: MakeDimension<Lr, Tr>) -> Self::Output {
        MakeDimension::new()
    }
}

pub type ProdDimension<Dl, Dr> = <Dl as Mul<Dr>>::Output;

pub type LengthDimension = MakeDimension<P1, Z0>;
pub type TimeDimension = MakeDimension<Z0, P1>;
