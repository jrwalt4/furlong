use std::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, AddAssign, Mul};

use crate::types::Real;
use crate::unit::*;

#[derive(Copy, Clone)]
pub struct Qnty<U> {
    value: Real,
    unit: PD<U>,
}

impl<U> Qnty<U> {
    pub fn new(value: Real) -> Qnty<U> {
        Qnty { value, unit: PD }
    }

    pub fn value(&self) -> Real {
        self.value
    }
}

trait QuantityFrom<T> {
    fn from_quantity(other: &Qnty<T>) -> Self;
}

trait QuantityInto<T> {
    fn into_quantity(self) -> Qnty<T>;
}

impl<U: UnitInto<T>, T> QuantityInto<T> for Qnty<U> {
    fn into_quantity(self) -> Qnty<T> {
        Qnty::new(self.value / <U as UnitInto<T>>::DIVIDE_BY as Real)
    }
}

impl<Ul, Ur: UnitInto<Ul>> PartialEq<Qnty<Ur>> for Qnty<Ul> {
    fn eq(&self, other: &Qnty<Ur>) -> bool {
        self.value == other.value / <Ur as UnitInto<Ul>>::DIVIDE_BY as Real
    }
}

impl<Ul, Ur> Add<Qnty<Ur>> for Qnty<Ul>
where
    Ul: Add<Ur>,
{
    type Output = Qnty<Ul>;
    fn add(mut self, rhs: Qnty<Ur>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<Ul, Ur> AddAssign<Qnty<Ur>> for Qnty<Ul>
where
    Ul: Add<Ur>
{
    fn add_assign(&mut self, rhs: Qnty<Ur>) {
        self.value += rhs.value;
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
