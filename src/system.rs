use crate::{
    base_unit::*,
    dimension::*,
    unit::*,
};

pub type Mass<S> = SystemUnit<S, MassDimension>;
pub type Length<S> = SystemUnit<S, LengthDimension>;
pub type Area<S> = SystemUnit<S, AreaDimension>;
pub type Time<S> = SystemUnit<S, TimeDimension>;

pub mod si {
    use super::*;

    pub type System = MakeSystem<
        mass::KilogramBaseUnit,
        length::MeterBaseUnit,
        time::SecondBaseUnit
    >;

    pub type Meters = Length<System>;
    pub type Centimeters = ScaledUnit<Meters, 1, 1000>;
    pub type Kilometers = ScaledUnit<Meters, 1000>;

    pub type Seconds = Time<System>;
    pub type Minutes = ScaledUnit<Seconds, 60>;
    pub type Hours = ScaledUnit<Minutes, 60>;

    #[test]
    fn conversions() {
        use crate::conversion::*;
        assert_eq!(Conversion::<Centimeters, Meters>::SCALE, 1.0/1_000.0);
        assert_eq!(Conversion::<Meters, Centimeters>::SCALE, 1_000.0);

        assert_eq!(Conversion::<Meters, Kilometers>::SCALE, 1.0/1_000.0);

        assert_eq!(Conversion::<Hours, Seconds>::SCALE, 3_600.0);
    }
}

pub mod imperial {
    use super::*;
    
    pub type System = MakeSystem<
        mass::SlugBaseUnit,
        length::FootBaseUnit,
        time::SecondBaseUnit
    >;

    pub type Feet = Length<System>;
    pub type Yards = ScaledUnit<Feet, 3>;
    pub type Miles = ScaledUnit<Feet, 5_280>;

    pub type Seconds = Time<System>;
    pub type Minutes = ScaledUnit<Seconds, 60>;
    pub type Hours = ScaledUnit<Minutes, 60>;

    #[test]
    fn conversions() {
        use crate::conversion::*;
        assert_eq!(Conversion::<Feet, Yards>::SCALE, 1.0/3.0);
        assert_eq!(Conversion::<Miles, Feet>::SCALE, 5_280.0);

        assert_eq!(Conversion::<Hours, Seconds>::SCALE, 3_600.0);
    }
}
