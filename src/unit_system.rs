use std::marker::PhantomData as PD;

use crate::base_unit::BaseUnit;
use crate::base_dimension::{MassBaseDimension, LengthBaseDimension, TimeBaseDimension};

pub trait UnitSystem {
    type Mass: BaseUnit<Dimension = MassBaseDimension>;
    type Length: BaseUnit<Dimension = LengthBaseDimension>;
    type Time: BaseUnit<Dimension = TimeBaseDimension>;
}

#[derive(Debug)]
pub struct MakeSystem<MB, LB, TB> {
    mass_base: PD<MB>,
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<M, L, T> UnitSystem for MakeSystem<M, L, T>
where
    M: BaseUnit<Dimension = MassBaseDimension>,
    L: BaseUnit<Dimension = LengthBaseDimension>,
    T: BaseUnit<Dimension = TimeBaseDimension>,
{
    type Mass = M;
    type Length = L;
    type Time = T;
}
