use crate::base_unit::{mass, length, time};
use crate::unit_system::MakeSystem;
use crate::unit::*;
use crate::dimension::*;

macro_rules! export_base_units {
    ($SYS:ty) => {
        pub type Mass = SystemUnit<$SYS, MassDimension>;
        pub type Length = SystemUnit<$SYS, LengthDimension>;
        pub type Area = SystemUnit<$SYS, AreaDimension>;
        pub type Time = SystemUnit<$SYS, TimeDimension>;
    };
}
pub mod si {
    use super::*;

    pub type System = MakeSystem<
        mass::KilogramBaseUnit,
        length::MeterBaseUnit,
        time::SecondBaseUnit
    >;
    export_base_units!(System);

    pub type Meter = self::Length;
    pub type Centimeter = ScaledUnit<Length, 1, 1000>;
    pub type Kilometer = ScaledUnit<Length, 1000>;

    pub type Second = self::Time;
    pub type Minute = ScaledUnit<Time, 60>;
    pub type Hour = ScaledUnit<Time, 3600>;

    #[test]
    fn conversions() {
        assert_eq!(Conversion::<Centimeter, Meter>::SCALE, 1.0/1_000.0);
        assert_eq!(Conversion::<Meter, Centimeter>::SCALE, 1_000.0);

        assert_eq!(Conversion::<Meter, Kilometer>::SCALE, 1.0/1_000.0);

        assert_eq!(Conversion::<Hour, Second>::SCALE, 3_600.0);
    }
}

pub mod imperial {
    use super::*;
    
    pub type System = MakeSystem<
        mass::SlugBaseUnit,
        length::FootBaseUnit,
        time::SecondBaseUnit
    >;
    export_base_units!(System);

    pub type Feet = self::Length;
    pub type Yard = ScaledUnit<Feet, 3>;
    pub type Mile = ScaledUnit<Feet, 5_280>;

    pub type Second = self::Time;
    pub type Minute = ScaledUnit<Time, 60>;
    pub type Hour = ScaledUnit<Time, 3600>;

    #[test]
    fn conversions() {
        assert_eq!(Conversion::<Feet, Yard>::SCALE, 1.0/3.0);
        assert_eq!(Conversion::<Mile, Feet>::SCALE, 5_280.0);

        assert_eq!(Conversion::<Hour, Second>::SCALE, 3_600.0);
    }
}
