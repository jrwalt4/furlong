use std::marker::PhantomData as PD;

use crate::base_unit::*;

pub trait UnitSystem {
    type Length: BaseUnit; //+BaseUnitInfo;
    type Time: BaseUnit; //+BaseUnitInfo;
}

#[derive(Debug, Copy, Clone)]
pub struct System<LB: BaseUnit, TB: BaseUnit> {
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<LB: BaseUnit, TB: BaseUnit> UnitSystem for System<LB, TB> {
    type Length = LB;
    type Time = TB;
}
