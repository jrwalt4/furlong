use std::marker::PhantomData as PD;

use crate::unit::BaseUnit;
use crate::dimension::{MassBaseDimension, LengthBaseDimension, TimeBaseDimension};

pub trait UnitSystem {
    type Mass: BaseUnit<Dimension = MassBaseDimension>;
    type Length: BaseUnit<Dimension = LengthBaseDimension>;
    type Time: BaseUnit<Dimension = TimeBaseDimension>;
}

#[derive(Debug, Copy, Clone)]
pub struct System<MB: BaseUnit, LB: BaseUnit, TB: BaseUnit> {
    mass_base: PD<MB>,
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<M, L, T> UnitSystem for System<M, L, T>
where
    M: BaseUnit<Dimension = MassBaseDimension>,
    L: BaseUnit<Dimension = LengthBaseDimension>,
    T: BaseUnit<Dimension = TimeBaseDimension>,
{
    type Mass = M;
    type Length = L;
    type Time = T;
}

trait UnitSystemFrom<U> {
    const MULTIPLY_BY: f32;
}
