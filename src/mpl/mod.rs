//! Metaprogramming lists.

mod map;

#[doc(inline)]
pub use map::*;

use typenum::{
    True, False,
    Less, Equal, Greater
};

pub trait Match {
    type Output;
}

pub type Switch<M> = <M as Match>::Output;

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

//-----------------------------------------------------------------------------
// Get

/// Type operator to get retrieve an entry with key `K`. 
/// 
/// # Examples
/// ```
/// # use furlong::{tmap, mpl::*};
/// # use typenum::*;
/// assert_type_eq!(
///     Entry<
///         tmap!{U1: P1, U2: P2},
///         U1
///     >, P1);
/// ```
/// 
/// Fails if key does not exist
/// ```compile_fail
/// # use furlong::{tmap, mpl::*};
/// # use typenum::*;
/// assert_type_eq!(
///     Entry<
///         tmap!{U1: P1, U2: P2},
///         U3 // Error: Type has no item with key U3
///     >, P1);
/// ```
/// 
pub trait Get<K> {
    type Output;
}

pub type Entry<L, K> = <L as Get<K>>::Output;

//-----------------------------------------------------------------------------
// GetOr

/// Type operator to get retrieve an entry with key `K` or a default
/// 
/// # Examples
/// ```
/// # use furlong::{tmap, mpl::*};
/// # use typenum::*;
/// assert_type_eq!(
///     EntryOr<
///         tmap!{U1: P1, U2: P2},
///         U1,
///         ()
///     >, P1);
/// ```
/// 
/// Returns default if key does not exist
/// ```
/// # use furlong::{tmap, mpl::*};
/// # use typenum::*;
/// assert_type_eq!(
///     EntryOr<
///         tmap!{U1: P1, U2: P2},
///         U3, // Type has no item with key U3
///         Z0
///     >, Z0);
/// ```
/// 
pub trait GetOr<K, Def> {
    type Output;
}

pub type EntryOr<L, K, Def> = <L as GetOr<K, Def>>::Output;

//-----------------------------------------------------------------------------
// Sort

/// Sort a container by key
/// 
/// ```
/// # use furlong::{*, mpl::*};
/// # use typenum::*;
/// assert_type_eq!(
///     Sorted<tmap!{U2: P1, U1: P1}>,
///     tmap!{U1: P1, U2: P1}
/// );
/// 
/// 
/// ```
pub trait Sort {
    type Output;
}

pub type Sorted<M> = <M as Sort>::Output;
