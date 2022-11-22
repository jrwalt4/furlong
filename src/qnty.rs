use std::fmt::{Display, Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, AddAssign, Mul, Div, SubAssign, Sub};

use approx::AbsDiffEq;
use num_traits::Float;
use typenum::{Prod, Quot};

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
    pub fn new(value: T) -> Qnty<U, T> {
        Qnty { value, unit: PD }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

pub trait IntoUnit<U, T> {
    fn into_unit(self) -> Qnty<U, T>;
}

impl<U1, T1, U2, T2> IntoUnit<U2, T2> for Qnty<U1, T1>
where
    Conversion<U1, U2>: UnitConversion,
    T1: Into<T2>,
    T2: Mul<f64, Output=T2>
{
    fn into_unit(self) -> Qnty<U2, T2>{
        Qnty::<U2, T2>::new(self.value.into() * Conversion::<U1, U2>::FACTOR)
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

impl<Ul, Tl, Ur, Tr> AbsDiffEq<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Into<Tl>,
    Tl: Mul<f64, Output=Tl>,
    Tl: PartialEq<Tr>,
    Tl: AbsDiffEq<Epsilon = Tl>,
    Tl: Float,
    Qnty<Ur, Tr>: Copy
{
    type Epsilon = Qnty<Ul, Tl>;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::new(<Tl as Float>::epsilon())
    }

    fn abs_diff_eq(&self, other: &Qnty<Ur, Tr>, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(other.into_unit().value(), epsilon.value)
    }
}

impl<Ul, Tl, Ur, Tr> PartialEq<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Into<Tl>,
    Tl: Mul<f64, Output=Tl>,
    Tl: PartialEq,
    Qnty<Ur, Tr>: Copy
{
    fn eq(&self, other: &Qnty<Ur, Tr>) -> bool {
        self.value() == other.into_unit().value()
    }
}

impl<Ul, Tl, Ur, Tr> Add<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Into<Tl>,
    Tl: Mul<f64, Output=Tl> + AddAssign<Tl>
{
    type Output = Qnty<Ul, Tl>;
    fn add(mut self, rhs: Qnty<Ur, Tr>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<Ul, Tl, Ur, Tr> AddAssign<Qnty<Ur, Tr>> for Qnty<Ul, Tl>
where
    Ur: Unit,
    Ul: Unit<Dim = <Ur as Unit>::Dim>,
    Conversion<Ur, Ul>: UnitConversion,
    Tr: Into<Tl>,
    Tl: Mul<f64, Output=Tl> + AddAssign<Tl>
{
    fn add_assign(&mut self, rhs: Qnty<Ur, Tr>) {
        self.value += rhs.into_unit().value;
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

impl<U: UnitInfo> Display for Qnty<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{0:.1$} {2}", self.value, f.precision().unwrap_or(2), <U as UnitInfo>::abbr())
    }
}

impl<U: UnitInfo> Debug for Qnty<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Qnty")
            .field("value", &self.value)
            .field("unit", &U::abbr())
            .finish()
    }
}
