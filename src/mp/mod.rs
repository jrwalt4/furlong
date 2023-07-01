//! Metaprogramming utilities.

pub mod map;

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
