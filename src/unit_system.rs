use std::marker::PhantomData as PD;

use crate::base_unit::*;

pub trait UnitSystem {
    type Mass: BaseUnit;
    type Length: BaseUnit; //+BaseUnitInfo;
    type Time: BaseUnit; //+BaseUnitInfo;
}

#[derive(Debug, Copy, Clone)]
pub struct System<MB: BaseUnit, LB: BaseUnit, TB: BaseUnit> {
    mass_base: PD<MB>,
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<MB: BaseUnit, LB: BaseUnit, TB: BaseUnit> UnitSystem for System<MB, LB, TB> {
    type Mass = MB;
    type Length = LB;
    type Time = TB;
}
