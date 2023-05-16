use std::fmt::{Display, Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, AddAssign, Mul, Div, SubAssign, Sub};

use typenum::{Prod, Quot};
use num_traits::{Zero, One, AsPrimitive};

use crate::dimension::*;
use crate::unit::*;
use crate::unit_system::UnitSystem;

#[repr(transparent)]
pub struct Qnty<U, T = f64> {
    value: T,
    unit: PD<U>,
}

impl<U, T: Clone> Clone for Qnty<U, T> {
    fn clone(&self) -> Self {
        Qnty {
            value: self.value.clone(),
            unit: PD
        }
    }
}

impl<U, T: Copy> Copy for Qnty<U, T> {}

impl<U, T> Qnty<U, T> {
    /// Create a [`Qnty`] with the provided raw value. This 'raw_value' is the value
    /// of the base unit in the [`System`](crate::unit_system::UnitSystem) associated with
    /// this [`Unit`]. 
    /// ```
    /// # use furlong::{Qnty, system::si};
    /// let length = Qnty::<si::Length>::from_raw_value(1.0);
    /// assert_eq!(length.raw_value(), &1.0);
    /// ```
    pub fn from_raw_value(value: T) -> Qnty<U, T> {
        Qnty { value, unit: PD }
    }

    /// Returns a reference to the raw value of this [`Qnty`].
    pub fn raw_value(&self) -> &T {
        &self.value
    }

    pub fn into_type<T2>(self) -> Qnty<U, T2>
    where
        T: Into<T2>,
    {
        Qnty::from_raw_value(self.value.into())
    }

    pub fn into_unit<U2>(self) -> Qnty<U2, T> 
    where
        Conversion<U, U2>: UnitConversion,
        T: Convertible
    {
        Qnty::from_raw_value(self.value.convert::<Conversion<U, U2>>())
    }

    pub fn as_type<T2: 'static + Copy>(self) -> Qnty<U, T2>
    where 
        T: AsPrimitive<T2>
    {
        Qnty::from_raw_value(self.value.as_())
    }

    pub fn as_unit<U2>(self) -> Qnty<U2, T>
    where
        U: Unit,
        U2: Unit<System = U::System>
    {
        // same system, so no need to convert the value
        Qnty::from_raw_value(self.value)
    }

}

impl<S: UnitSystem, D: Dim, T> From<T> for Qnty<SystemUnit<S, D>, T> {
    fn from(value: T) -> Self {
        Qnty::from_raw_value(value)
    }
}

impl<Ul, Tl, Ur, Tr> PartialEq<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tl: UnitEq<Tr>,
{
    fn eq(&self, other: &Qnty<Ur, Tr>) -> bool {
        self.value.eq::<Conversion::<Ur, Ul>>(other.raw_value())
    }
}

impl<Ul, Tl, Ur, Tr> Add<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: SameDimension<<Ur as Unit>::Dim>,
    Tl: Add<Tr>
{
    type Output = Qnty<Ul, <Tl as Add<Tr>>::Output>;
    fn add(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Qnty::from_raw_value(self.value + rhs.value)
    }
}

impl<Ul, Tl, Ur, Tr> AddAssign<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: SameDimension<<Ur as Unit>::Dim>,
    Tl: AddAssign<Tr>
{
    fn add_assign(&mut self, rhs: Qnty<Ur, Tr>) {
        self.value += rhs.value;
    }
}

impl<Ul: Unit, Tl, Ur: Unit, Tr> Sub<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: SameDimension<<Ur as Unit>::Dim>,
    Tl: Sub<Tr>
{
    type Output = Qnty<Ul, <Tl as Sub<Tr>>::Output>;
    fn sub(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Qnty::from_raw_value(self.value - rhs.value)
    }
}

impl<Ul, T, Ur> SubAssign<Qnty<Ur, T>> for Qnty<Ul, T>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: SameDimension<<Ur as Unit>::Dim>,
    T: SubAssign
{
    fn sub_assign(&mut self, rhs: Qnty<Ur, T>) {
        self.value -= rhs.value;
    }
}

impl<Ul, Tl, Ur, Tr> Mul<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: Mul<<Ur as Unit>::Dim>,
    Prod<<Ul as Unit>::Dim, <Ur as Unit>::Dim>: Dim,
    Tl: Mul<Tr>
{
    type Output = Qnty<SystemUnit<<Ul as Unit>::System, Prod<<Ul as Unit>::Dim, Ur::Dim>>, <Tl as Mul<Tr>>::Output>;
    fn mul(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Self::Output::from_raw_value(
            self.value * 
            rhs.value
        )
    }
}

impl<Ul, Tl, Ur, Tr> Div<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: Div<<Ur as Unit>::Dim>,
    Quot<<Ul as Unit>::Dim, <Ur as Unit>::Dim>: Dim,
    Tl: Div<Tr>
{
    type Output = Qnty<SystemUnit<Ul::System, Quot<Ul::Dim, Ur::Dim>>, <Tl as Div<Tr>>::Output>;
    fn div(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Self::Output::from_raw_value( self.value / rhs.value )
    }
}

impl<U: UnitInfo, T: Display> Display for Qnty<U, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{0:.1$} {2}", self.value, f.precision().unwrap_or(2), <U as UnitInfo>::abbr())
    }
}

impl<U: UnitInfo, T: Debug> Debug for Qnty<U, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Qnty")
            .field("value", &self.value)
            .field("unit", &U::abbr())
            .finish()
    }
}

/// Only if it's a [`SystemUnit`] does 1 have a raw_value == 1
impl<S: UnitSystem, D: Dim, T: One> One for Qnty<SystemUnit<S, D>, T>
where Self: Mul<Output = Self> {
    fn one() -> Self {
        Qnty::from_raw_value(T::one())
    }
}

impl<U, T: Zero> Zero for Qnty<U, T> 
where Self: Add<Output = Self> {
    fn zero() -> Self {
        Self::from_raw_value(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
