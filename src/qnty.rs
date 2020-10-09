use std::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, Mul};

use crate::types::Real;
use crate::unit::*;

#[derive(Copy, Clone)]
pub struct Qnty<U: Unit> {
    value: Real,
    unit: PD<U>,
}

impl<U: Unit> Qnty<U> {
    pub fn new(value: Real) -> Qnty<U> {
        Qnty { value, unit: PD }
    }
}

impl<Ul, Ur> PartialEq<Qnty<Ur>> for Qnty<Ul>
where
    Ul: Unit+PartialEq<Ur>,
    Ur: Unit,
{
    fn eq(&self, other: &Qnty<Ur>) -> bool {
        self.value == other.value
    }
}

impl<Ul, Ur> Add<Qnty<Ur>> for Qnty<Ul>
where
    Ul: Unit,
    Ur: Unit<Dim = <Ul as Unit>::Dim>,
{
    type Output = Qnty<Ul>;
    fn add(self, rhs: Qnty<Ur>) -> Self::Output {
        //<Self as Add<Qnty<UR>>>
        Qnty::<Ul>::new(self.value + rhs.value)
    }
}

impl<Ul, Ur> Mul<Qnty<Ur>> for Qnty<Ul>
where
    Ul: Unit + Mul<Ur>,
    <Ul as Mul<Ur>>::Output: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
{
    type Output = Qnty<ProdUnit<Ul, Ur>>;
    fn mul(self, rhs: Qnty<Ur>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<U: UnitInfo> Debug for Qnty<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} {}", self.value, <U as UnitInfo>::abbr())
    }
}
