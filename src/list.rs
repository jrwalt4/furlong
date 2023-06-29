//! Utilities for grouping types into a list similar to [`TypeArray`](typenum::TypeArray).

use typenum::{
    Less, Equal, Greater,
    consts::{True, False}, 
    operator_aliases::Eq,
    type_operators::IsEqual,
};

pub trait TypeList {}

/// A TypeList similar to [`TArr`](typenum::TArr).
#[derive(Copy, Clone, Default, Debug)]
pub struct TList<T, L> {
    _item: T,
    _list: L
}

impl<T, L: TypeList> TypeList for TList<T, L> {}

/// TypeList terminator
pub struct LTerm;

impl TypeList for LTerm {}

/// An Entry in a TypeList
pub trait TypeListEntry {
    type Key;
    type Value;
}

pub type TKey<E> = <E as TypeListEntry>::Key;
pub type TVal<E> = <E as TypeListEntry>::Value;

/// An item in a [`TList`] with a Key and Value.
pub struct TEntry<K, V>(K, V);

impl<K, V> TypeListEntry for TEntry<K, V> {
    type Key = K;
    type Value = V;
}

/// Type Operator to find `K` in a TypeList. 
/// 
/// # Examples
/// Find an item with key
/// ```
/// # use furlong::list::*;
/// # use typenum::*;
/// assert_type_eq!(
///     Find<
///         TList<TEntry<U1, P1>, TList<TEntry<U2, P2>, LTerm>>,
///         U1
///     >, P1);
/// ```
/// 
/// Type fails if key does not exist
/// ```compile_fail
/// # use furlong::list::*;
/// # use typenum::*;
/// assert_type_eq!(
///     Find<
///         TList<TEntry<U1, P1>, TList<TEntry<U2, P2>, LTerm>>,
///         U3
///     >, P1); // Type has no item with key U3
/// ```
/// 
pub trait ListFind<K> {
    type Output;
}

impl<K, V> ListFind<K> for TEntry<K, V> {
    type Output = V;
}

impl<KFind, E, L> ListFind<KFind> for TList<E, L>
where
    E: TypeListEntry,
    TKey<E>: IsEqual<KFind>,
    MatchIf<Eq<TKey<E>, KFind>, E, L>: Match,
    <MatchIf<Eq<TKey<E>, KFind>, E, L> as Match>::Output: ListFind<KFind>
{
    type Output = Find<<MatchIf<Eq<TKey<E>, KFind>, E, L> as Match>::Output, KFind>;
}

pub type Find<L, K> = <L as ListFind<K>>::Output;

pub trait Match {
    type Output;
}

pub struct MatchIf<Pred, Yes, No>(Pred, Yes, No);

impl<Yes, No> Match for MatchIf<True, Yes, No> {
    type Output = Yes;
}

impl<Yes, No> Match for MatchIf<False, Yes, No> {
    type Output = No;
}

pub struct MatchCmp<Pred, L, E, G>(Pred, L, E, G);

impl<L, E, G> Match for MatchCmp<Less, L, E, G> {
    type Output = L;
}

impl<L, E, G> Match for MatchCmp<Equal, L, E, G> {
    type Output = E;
}

impl<L, E, G> Match for MatchCmp<Greater, L, E, G> {
    type Output = G;
}
