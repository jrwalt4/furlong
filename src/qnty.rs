use std::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, AddAssign, Mul, Div, SubAssign, Sub};

use approx::AbsDiffEq;
use typenum::{Prod, Quot};

use crate::dimension::*;
use crate::types::Real;
use crate::unit::*;
use crate::unit_system::UnitSystem;

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

impl<U: UnitConversion<T>, T> QuantityInto<T> for Qnty<U> {
    fn into_quantity(self) -> Qnty<T> {
        Qnty::new(self.value / <U as UnitConversion<T>>::FACTOR as Real)
    }
}

impl<U1, U2> AbsDiffEq<Qnty<U2>> for Qnty<U1>
where
    U2: UnitConversion<U1>
{
    type Epsilon = Qnty<U1>;

    fn default_epsilon() -> Self::Epsilon {
        Qnty::<U1>::new(0.0001)
    }

    fn abs_diff_eq(&self, other: &Qnty<U2>, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&(other.value * Conversion::<U2, U1>::FACTOR), epsilon.value)
    }
}

impl<Ul, Ur> PartialEq<Qnty<Ur>> for Qnty<Ul>
where
    Self: AbsDiffEq<Qnty<Ur>>
{
    fn eq(&self, other: &Qnty<Ur>) -> bool {
        self.abs_diff_eq(other, <Self as AbsDiffEq<Qnty<Ur>>>::default_epsilon())
    }
}

impl<Ul, Ur> Add<Qnty<Ur>> for Qnty<Ul>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Ur: UnitConversion<Ul>
{
    type Output = Qnty<Ul>;
    fn add(mut self, rhs: Qnty<Ur>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<Ul, Ur> AddAssign<Qnty<Ur>> for Qnty<Ul>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Ur: UnitConversion<Ul>
{
    fn add_assign(&mut self, rhs: Qnty<Ur>) {
        self.value += rhs.value * Conversion::<Ur, Ul>::FACTOR;
    }
}

impl<Ul, Ur> Sub<Qnty<Ur>> for Qnty<Ul>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Ur: UnitConversion<Ul>
{
    type Output = Qnty<Ul>;
    fn sub(mut self, rhs: Qnty<Ur>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<Ul, Ur> SubAssign<Qnty<Ur>> for Qnty<Ul>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Ur: UnitConversion<Ul>
{
    fn sub_assign(&mut self, rhs: Qnty<Ur>) {
        self.value -= rhs.value * Conversion::<Ur, Ul>::FACTOR;
    }
}

impl<S: UnitSystem, D: Dim, Ur> Mul<Qnty<Ur>> for Qnty<SystemUnit<S, D>>
where
    Ur: Unit,
    D: Mul<<Ur as Unit>::Dim>,
    Prod<D, <Ur as Unit>::Dim>: Dim,
    Conversion<Ur, SystemUnit<S, Ur::Dim>>: UnitConversion<SystemUnit<S, Ur::Dim>>
{
    type Output = Qnty<SystemUnit<S, Prod<D, Ur::Dim>>>;
    fn mul(self, rhs: Qnty<Ur>) -> Self::Output {
        Self::Output::new(
            self.value * 
            rhs.value * 
            <Conversion<Ur,SystemUnit<S, Ur::Dim>> as UnitConversion<SystemUnit<S, Ur::Dim>>>::FACTOR
        )
    }
}

impl<S: UnitSystem, D: Dim, Ur> Div<Qnty<Ur>> for Qnty<SystemUnit<S, D>>
where
    Ur: Unit,
    D: Div<<Ur as Unit>::Dim>,
    Quot<D, <Ur as Unit>::Dim>: Dim,
    Conversion<Ur, SystemUnit<S, Ur::Dim>>: UnitConversion<SystemUnit<S, Ur::Dim>>
{
    type Output = Qnty<SystemUnit<S, Quot<D, Ur::Dim>>>;
    fn div(self, rhs: Qnty<Ur>) -> Self::Output {
        Self::Output::new(
            self.value / 
            (
                rhs.value *
                <Conversion<Ur,SystemUnit<S, Ur::Dim>> as UnitConversion<SystemUnit<S, Ur::Dim>>>::FACTOR
            )
        )
    }
}

impl<U: UnitInfo> Debug for Qnty<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} {}", self.value, <U as UnitInfo>::abbr())
    }
}
