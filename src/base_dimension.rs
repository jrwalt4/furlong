
pub trait BaseDimension {}

pub struct MassBaseDimension;
impl BaseDimension for MassBaseDimension {}

pub struct LengthBaseDimension;
impl BaseDimension for LengthBaseDimension {}

pub struct TimeBaseDimension;
impl BaseDimension for TimeBaseDimension {}
