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
    pub(crate) fn new(value: T) -> Qnty<U, T> {
        Qnty { value, unit: PD }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn as_type<T2>(self) -> Qnty<U, T2>
    where
        T2: 'static + Copy,
        T: 'static + Copy + AsPrimitive<T2>
    {
        Qnty::<U, T2>::new(self.value.as_())
    }

    pub fn as_unit<U2>(self) -> Qnty<U2, <T as Mul<f64>>::Output>
    where
        Conversion<U, U2>: UnitConversion,
        T: Mul<f64>
    {
        Qnty::<U2, <T as Mul<f64>>::Output>::new(self.value * Conversion::<U, U2>::SCALE)
    }
}

pub trait IntoType<T> {

}

pub trait IntoUnit<U, T> {
    fn into_unit(self) -> Qnty<U, T>;
}

impl<U1, T1, U2, T2> IntoUnit<U2, T2> for Qnty<U1, T1>
where
    Conversion<U1, U2>: UnitConversion,
    T1: Mul<f64>,
    <T1 as Mul<f64>>::Output: Into<T2>
{
    fn into_unit(self) -> Qnty<U2, T2>{
        Qnty::<U2, T2>::new((self.value * Conversion::<U1, U2>::SCALE).into())
    }
}

impl<U1, T1, U2, T2> Into<Qnty<U2, T2>> for &Qnty<U1, T1>
where
    Qnty<U1, T1>: IntoUnit<U2, T2>,
    Qnty<U1, T1>: Copy
{
    fn into(self) -> Qnty<U2, T2> {
        self.into_unit()
    }
}

impl<Ul, Tl, Ur, Tr> PartialEq<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Mul<f64> + Copy,
    Tl: PartialEq<<Tr as Mul<f64>>::Output>,
{
    fn eq(&self, other: &Qnty<Ur, Tr>) -> bool {
        self.value == *other.value() * Conversion::<Ur, Ul>::SCALE
    }
}

impl<Ul, Tl, Ur, Tr> Add<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Mul<f64>,
    Tl: Add<<Tr as Mul<f64>>::Output>
{
    type Output = Qnty<Ul, <Tl as Add<<Tr as Mul<f64>>::Output>>::Output>;
    fn add(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Self::Output::new(self.value + rhs.as_unit::<Ul>().value)
    }
}

impl<Ul, Tl, Ur, Tr> AddAssign<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Mul<f64>,
    Tl: AddAssign<<Tr as Mul<f64>>::Output>
{
    fn add_assign(&mut self, rhs: Qnty<Ur, Tr>) {
        self.value += rhs.as_unit().value;
    }
}

impl<Ul, Tl, Ur, Tr> Sub<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Qnty<Ur, Tr>: IntoUnit<Ul, Tl>,
    Tl: SubAssign
{
    type Output = Qnty<Ul, Tl>;
    fn sub(mut self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<Ul, Tl, Ur, Tr> SubAssign<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Qnty<Ur, Tr>: IntoUnit<Ul, Tl>,
    Tl: SubAssign
{
    fn sub_assign(&mut self, rhs: Qnty<Ur, Tr>) {
        self.value -= rhs.into_unit().value;
    }
}

impl<S: UnitSystem, D: Dim, Tl, Ur, Tr> Mul<Qnty<Ur, Tr>> for Qnty<SystemUnit<S, D>, Tl>
where
    Ur: Unit,
    D: Mul<<Ur as Unit>::Dim>,
    Prod<D, <Ur as Unit>::Dim>: Dim,
    Qnty<Ur, Tr>: IntoUnit<SystemUnit<S, Ur::Dim>, Tl>,
    Tl: Mul<Output = Tl>
{
    type Output = Qnty<SystemUnit<S, Prod<D, Ur::Dim>>, Tl>;
    fn mul(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Self::Output::new(
            self.value * 
            rhs.into_unit().value
        )
    }
}

impl<S: UnitSystem, D: Dim, Tl, Ur, Tr> Div<Qnty<Ur, Tr>> for Qnty<SystemUnit<S, D>, Tl>
where
    Ur: Unit,
    D: Div<<Ur as Unit>::Dim>,
    Quot<D, <Ur as Unit>::Dim>: Dim,
    Qnty<Ur, Tr>: IntoUnit<SystemUnit<S, Ur::Dim>, Tl>,
    Tl: Div<Output = Tl>
{
    type Output = Qnty<SystemUnit<S, Quot<D, Ur::Dim>>, Tl>;
    fn div(self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        Self::Output::new( self.value / rhs.into_unit().value )
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

impl<U, T: One> One for Qnty<U, T>
where Self: Mul<Output = Self> {
    fn one() -> Self {
        Qnty::new(T::one())
    }
}

impl<U, T: Zero> Zero for Qnty<U, T> 
where Self: Add<Output = Self> {
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
