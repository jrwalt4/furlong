use crate::types::{Real, Info};

pub trait BaseUnit {
    const CONV: Real;
}

pub trait BaseUnitInfo {
    const NAME: Info;
    const SYMBOL: Info;
}
