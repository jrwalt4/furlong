use std::fmt::{Display, Debug, Formatter, Result};
use std::marker::PhantomData as PD;
use std::ops::{Add, AddAssign, Mul, Div, SubAssign, Sub};

use typenum::{Prod, Quot};
use num_traits::{Zero, One, AsPrimitive};

use crate::{
    conversion::*,
    dimension::*,
    unit::*
};

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
    Ul: Unit,
    Ur: Unit<System = <Ul as Unit>::System>,
    <Ul as Unit>::Dim: SameDimension<<Ur as Unit>::Dim>,
    Tl: PartialEq<Tr>,
{
    fn eq(&self, other: &Qnty<Ur, Tr>) -> bool {
        self.value == other.value
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

#[cfg(test)]
mod test {
    use std::fmt::Display;
    
    use crate::{
        qnty::Qnty,
        unit::UnitInfo,
        system::si::{self, Length as Meters},
        system::imperial::{self, Length as Feet},
    };

    #[test]
    fn scaled_units() {
        let one_km = si::Kilometer::new(1.0);
        assert_eq!(one_km.raw_value(), &1_000.0);

        let one_yd = imperial::Yard::new(1.0);
        assert_eq!(one_yd.raw_value(), &3.0);

        let one_km_i = si::Kilometer::new(1);
        assert_eq!(one_km_i.raw_value(), &1_000);

        let one_yd_i = imperial::Yard::new(1);
        assert_eq!(one_yd_i.raw_value(), &3);
    }

    #[test]
    fn add_same_unit() {
        let l1 = Qnty::<Meters>::from_raw_value(2.0);
        let l2 = Meters::new(1.5);
        let l3 = Qnty::<Meters>::from_raw_value(3.5);
        assert_eq!(l1 + l2, l3);
    }

    #[test]
    fn add_int_units() {
        let l1 = Meters::new(1.5f64);
        let l2 = Meters::new(2i32);
        let l3 = l1 + l2.into_type::<f64>();
        assert_eq!(l3, Meters::new(3.5));
    }

    #[test]
    fn add_different_units() {
        let l1 = Feet::new(2.0f32);
        let l2 = Meters::new(1.0);
        let l3 = Feet::new(2.0 + 3.0 / 0.9144);
        assert_eq!(l1 + l2.into_unit::<Feet>(), l3);
    }

    #[test]
    fn add_different_types() {
        let mut l_f64 = Feet::new(2.0);
        let l_i32 = Feet::new::<i32>(1);
        l_f64 += l_i32.as_type::<f64>();
        assert_eq!(l_f64, Feet::new(3.0));
    }

    #[test]
    fn with_vectors() {
        use std::ops::{Mul, Add};

        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Vec3<T>(T, T, T);

        impl<T: Mul<f64, Output = T>> Mul<f64> for Vec3<T> {
            type Output = Vec3<T>;
            fn mul(self, rhs: f64) -> Self::Output {
                Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
            }
        }

        impl<T: Add<T2>, T2> Add<Vec3<T2>> for Vec3<T> {
            type Output = Vec3<<T as Add<T2>>::Output>;
            fn add(self, rhs: Vec3<T2>) -> Self::Output {
                Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
            }
        }

        let length_v = Feet::new(Vec3::<f64>(1.0, 2.0, 3.0));
        let width_v = length_v;
        let perimeter_v = length_v + width_v;
        assert_eq!(perimeter_v, Feet::new(Vec3::<f64>(2.0, 4.0, 6.0)));
    }

    #[test]
    fn subtract_units() {
        let l1 = Meters::new(3.0f32);
        let l2 = Feet::new(3.0f32);
        assert_eq!(l1 - l2.into_unit::<Meters>(), Meters::new(3.0-0.9144));
    }

    #[test]
    fn multiply_units() {
        let l1 = Meters::new(2.0f32);
        let l2 = Feet::new(3.0);
        let a1 = l1 * l2.into_unit::<Meters>();
        assert_eq!(a1, si::Area::new(2.0*0.9144));
    }

    #[test]
    fn divide_units() {
        let l = Meters::new(2.0f64);
        let t = Qnty::<si::Time>::from_raw_value(1.0);
        let v = l / t;
        assert_eq!(v.raw_value(), &2.0);
    }

    #[test]
    fn copy() {
        let l1 = Meters::new(1.0);
        let l2 = l1;
        assert_eq!(l1, l2);
    }

    #[test]
    fn unit_info() {
        type U = Meters;
        assert_eq!(<U as UnitInfo>::abbr(), "m");
        let length = Meters::new(3.0);
        assert_eq!(format!("{length:.3}"), "3.000 m");
    }

    #[test]
    fn generic_unit_info() {
        // test Display for i32
        let q = Qnty::<Feet, i32>::from_raw_value(2);
        assert_eq!(format!("{q}"), "2 ft");

        // test Display for custom type
        #[derive(Debug, Clone, Copy)]
        struct MyValue(f32);
        impl Display for MyValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
        let mv = MyValue(3.0);
        let q = Qnty::<Feet, MyValue>::from_raw_value(mv);
        assert_eq!(format!("{q}"), format!("{mv} ft"));
    }
}
