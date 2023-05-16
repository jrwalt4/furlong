use std::marker::PhantomData as PD;

use crate::{
    base_dimension::{
        MassBaseDimension, 
        LengthBaseDimension, 
        TimeBaseDimension, 
        BaseDimension
    },
    base_unit::BaseUnit
};

pub trait UnitSystemPart<D: BaseDimension> {
    type Base: BaseUnit;
}

pub type GetBase<S, D> = <S as UnitSystemPart<D>>::Base;

pub trait UnitSystem:
    UnitSystemPart<MassBaseDimension> +
    UnitSystemPart<LengthBaseDimension> +
    UnitSystemPart<TimeBaseDimension> {}

#[derive(Debug)]
pub struct MakeSystem<MB, LB, TB> {
    mass_base: PD<MB>,
    length_base: PD<LB>,
    time_base: PD<TB>,
}

impl<Mass: BaseUnit, Length, Time> UnitSystemPart<MassBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Mass;
}

impl<Mass, Length: BaseUnit, Time> UnitSystemPart<LengthBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Length;
}

impl<Mass, Length, Time: BaseUnit> UnitSystemPart<TimeBaseDimension> for MakeSystem<Mass, Length, Time> {
    type Base = Time;
}

impl<M: BaseUnit, L: BaseUnit, T: BaseUnit> UnitSystem for MakeSystem<M, L, T> {}
