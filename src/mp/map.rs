//! A type level map of key-value pairs.

use super::*;

use std::ops::{Add, Neg, Sub};

use typenum::{
    operator_aliases::{Compare, Eq, Negate, Sum},
    type_operators::{Cmp, IsEqual}
};

//-----------------------------------------------------------------------------
// TMap, TEnd, and TEntry

/// A collection of [`TypeMapEntry`]'s
pub trait TypeMap {}

/// A type container similar to [`TArr`](typenum::TArr), 
/// but for (key, value) pairs of type [`TEntry`].
#[derive(Copy, Clone, Default, Debug)]
pub struct TMap<Entry, Map> {
    _entry: Entry,
    _rest: Map
}

impl<E: TypeMapEntry, M: TypeMap> TypeMap for TMap<E, M> {}

/// MList terminator
#[derive(Copy, Clone, Default, Debug)]
pub struct TEnd;

impl TypeMap for TEnd {}

/// An Entry in a TypeList
pub trait TypeMapEntry {
    type Key;
    type Value;
}

/// The [`Key`](TypeMapEntry::Key) of a [`TypeMapEntry`];
pub type TKey<E> = <E as TypeMapEntry>::Key;

/// The [`Value`](TypeMapEntry::Value) of a [`TypeMapEntry`];
pub type TVal<E> = <E as TypeMapEntry>::Value;

/// An item in a [`TMap`] with a Key and Value.
#[derive(Copy, Clone, Default, Debug)]
pub struct TEntry<K, V>(K, V);

impl<K, V> TypeMapEntry for TEntry<K, V> {
    type Key = K;
    type Value = V;
}

#[macro_export]
/// Macro to build a TMap
/// # Example
/// ```
/// # use furlong::{tmap, mp::map::{TMap, TEntry, TEnd}};
/// # use typenum::*;
/// struct A;
/// struct B;
/// assert_type_eq!(tmap!{A: U1, B: U2}, TMap<TEntry<A, U1>, TMap<TEntry<B, U2>, TEnd>>);
/// ```
macro_rules! tmap {
    () => { $crate::mp::map::TEnd };
    ($K:ty: $V:ty,) => { $crate::mp::map::TMap<$crate::mp::map::TEntry<$K, $V>, $crate::mp::map::TEnd> };
    ($K:ty: $V:ty) => { $crate::mp::map::TMap<$crate::mp::map::TEntry<$K, $V>, $crate::mp::map::TEnd> };
    ($K:ty: $V:ty, $($K2:ty: $V2:ty),+) => { $crate::mp::map::TMap<$crate::mp::map::TEntry<$K, $V>, tmap!{$($K2: $V2),+}> };
    ($K:ty: $V:ty, $($K2:ty: $V2:ty),+,) => { tmap!{$K: $V, $($K2: $V2),+} };
}

//-----------------------------------------------------------------------------
// Get

/// Type operator to get entry with key `K` from a TypeMap. 
/// 
/// # Examples
/// ```
/// # use furlong::{tmap, mp::map::*};
/// # use typenum::*;
/// assert_type_eq!(
///     MapGet<
///         tmap!{U1: P1, U2: P2},
///         U1
///     >, P1);
/// ```
/// 
/// Fails if key does not exist
/// ```compile_fail
/// # use furlong::{tmap, mp::map::*};
/// # use typenum::*;
/// assert_type_eq!(
///     MapGet<
///         tmap!{U1: P1, U2: P2},
///         U3 // Error: Type has no item with key U3
///     >, P1);
/// ```
/// 
pub trait TypeMapFind<K> {
    type Output;
}

impl<K, V> TypeMapFind<K> for TEntry<K, V> {
    type Output = V;
}

impl<KFind, E, L> TypeMapFind<KFind> for TMap<E, L>
where
    E: TypeMapEntry,
    TKey<E>: IsEqual<KFind>,
    MatchIf<Eq<TKey<E>, KFind>, E, L>: Match,
    Switch<MatchIf<Eq<TKey<E>, KFind>, E, L>>: TypeMapFind<KFind>
{
    type Output = MapGet<Switch<MatchIf<Eq<TKey<E>, KFind>, E, L>>, KFind>;
}

pub type MapGet<L, K> = <L as TypeMapFind<K>>::Output;

//-----------------------------------------------------------------------------
// Add

/// ```
/// # use typenum::*;
/// # use furlong::tmap;
/// # type A = U1;
/// # type B = U2;
/// # type C = U3;
/// typenum::assert_type_eq!(
///     Sum<
///         tmap!{A: U1,        C: U3},
///         tmap!{       B: U2, C: U4}
///     >,
///         tmap!{A: U1, B: U2, C: U7}
/// );
/// ```
impl<El: TypeMapEntry, Ml: TypeMap, Er: TypeMapEntry, Mr: TypeMap> Add<TMap<Er, Mr>> for TMap<El, Ml>
where
    TKey<El>: Cmp<TKey<Er>>,
    El: TEntryAdd<Ml, Er, Mr, Compare<TKey<El>, TKey<Er>>>,
{
    type Output = TEntrySum<El, Ml, Er, Mr>;
    fn add(self, _rhs: TMap<Er, Mr>) -> Self::Output {
        unimplemented!()
    }
}

pub trait TEntryAdd<Ml, Er, Mr, C> {
    type Output;
}

pub type TEntrySum<El, Ml, Er, Mr> = <El as TEntryAdd<Ml, Er, Mr, Compare<TKey<El>, TKey<Er>>>>::Output;

impl<Kl, Vl, Ml, Er, Mr> TEntryAdd<Ml, Er, Mr, Greater> for TEntry<Kl, Vl>
where
    TMap<Self, Ml>: Add<Mr>,
{
    type Output = TMap<Er, Sum<TMap<Self, Ml>, Mr>>;
}

impl<Kl, Vl, Ml, Er, Mr> TEntryAdd<Ml, Er, Mr, Less> for TEntry<Kl, Vl>
where
    Ml: Add<TMap<Er, Mr>>
{
    type Output = TMap<Self, Sum<Ml, TMap<Er, Mr>>>;
}

impl<Kl, Vl, Ml, Er: TypeMapEntry, Mr> TEntryAdd<Ml, Er, Mr, Equal> for TEntry<Kl, Vl>
where
    Ml: Add<Mr>,
    Vl: Add<TVal<Er>>
{
    type Output = TMap<TEntry<Kl, Sum<Vl, TVal<Er>>>, Sum<Ml, Mr>>;
}

impl<Er, Mr> Add<TMap<Er, Mr>> for TEnd {
    type Output = TMap<Er, Mr>;
    fn add(self, _rhs: TMap<Er, Mr>) -> Self::Output {
        unimplemented!()
    }
}

impl<El, Ml> Add<TEnd> for TMap<El, Ml> {
    type Output = TMap<El, Ml>;
    fn add(self, _rhs: TEnd) -> Self::Output {
        unimplemented!()
    }
}

impl Add<TEnd> for TEnd {
    type Output = TEnd;
    fn add(self, _rhs: TEnd) -> Self::Output {
        unimplemented!()
    }
}

impl<KeyL, ValL: Add<ValR>, KeyR, ValR> Add<TEntry<KeyR, ValR>> for TEntry<KeyL, ValL> 
{
    type Output = TEntry<KeyL, Sum<ValL, ValR>>;
    fn add(self, _rhs: TEntry<KeyR, ValR>) -> Self::Output {
        unimplemented!()
    }
}

#[test]
fn tmap_add() {
    use typenum::*;
    type M1 = tmap!{U1: P2};
    type M2 = tmap!{U2: P3};
    type M3 = Sum<M1, M2>;
    assert_type_eq!(M3, tmap!{U1: P2, U2: P3});
    assert_eq!(<MapGet<M3, U2> as Integer>::I32, 3);
}
//-----------------------------------------------------------------------------
// Sub

impl<El, Ml, Er, Mr> Sub<TMap<Er, Mr>> for TMap<El, Ml>
where
    TMap<Er, Mr>: Neg,
    Self: Add<Negate<TMap<Er, Mr>>>
{
    type Output = Sum<Self, Negate<TMap<Er, Mr>>>;
    fn sub(self, _rhs: TMap<Er, Mr>) -> Self::Output {
        unimplemented!()
    }
}

#[test]
fn tmap_sub() {
    use typenum::*;
    type M1 = tmap!{U1: P2};
    type M2 = tmap!{U2: P3};
    type M3 = Diff<M1, M2>;
    assert_type_eq!(M3, tmap!{U1: P2, U2: N3});
    assert_eq!(<MapGet<M3, U2> as Integer>::I32, -3);

    type M4 = tmap!{U1: P2, U2: P2};
    type M5 = tmap!{U1: P2, U2: P2};
    type M6 = Diff<M4, M5>;
    assert_type_eq!(M6, tmap!{U1: Z0, U2: Z0});
    assert_eq!(<MapGet<M6, U2> as Integer>::I32, 0);
}

//-----------------------------------------------------------------------------
// Neg

impl<E: Neg, M: Neg> Neg for TMap<E, M> {
    type Output = TMap<Negate<E>, Negate<M>>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl Neg for TEnd {
    type Output = TEnd;
    fn neg(self) -> Self::Output {
        TEnd
    }
}

impl<K, V: Neg> Neg for TEntry<K, V> {
    type Output = TEntry<K, Negate<V>>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}
