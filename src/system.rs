use crate::{base_unit::*, dimension::*, tmap, unit::*};

use typenum::{consts::*, Shleft, UFrac};

pub type Mass<S> = SystemUnit<S, MassDimension>;
pub type Length<S> = SystemUnit<S, LengthDimension>;
pub type Area<S> = SystemUnit<S, AreaDimension>;
pub type Time<S> = SystemUnit<S, TimeDimension>;
pub type Velocity<S> = SystemUnit<S, VelocityDimension>;

pub mod si {
    use super::*;

    pub type System = tmap! {
        MassBase: mass::KilogramBaseUnit,
        LengthBase: length::MeterBaseUnit,
        TimeBase: time::SecondBaseUnit,
    };

    pub type Meters = Length<System>;
    pub type Centimeters = ScaledUnit<Meters, UFrac<U1, U1000>>;
    pub type Kilometers = ScaledUnit<Meters, UFrac<U1000>>;

    pub type Seconds = Time<System>;
    pub type Minutes = ScaledUnit<Seconds, UFrac<U60>>;
    pub type Hours = ScaledUnit<Minutes, UFrac<U60>>;

    #[test]
    fn conversions() {
        use crate::conversion::*;
        use typenum::UnsignedRational;
        
        assert_eq!(Conversion::<Centimeters, Meters>::F32, 1.0/1_000.0);
        assert_eq!(Conversion::<Meters, Centimeters>::F32, 1_000.0);

        assert_eq!(Conversion::<Meters, Kilometers>::F32, 1.0/1_000.0);

        assert_eq!(Conversion::<Hours, Seconds>::F32, 3_600.0);
    }
}

pub mod imperial {
    use super::*;

    pub type System = tmap! {
        MassBase: mass::SlugBaseUnit,
        LengthBase: length::FootBaseUnit,
        TimeBase: time::SecondBaseUnit,
    };

    pub type Feet = Length<System>;
    pub type Yards = ScaledUnit<Feet, UFrac<U3>>;
    // 5280 = 1010010100000 <= 660 << 3
    pub type Miles = ScaledUnit<Feet, UFrac<Shleft<U660, U3>>>;

    pub type Seconds = Time<System>;
    pub type Minutes = ScaledUnit<Seconds, UFrac<U60>>;
    pub type Hours = ScaledUnit<Minutes, UFrac<U60>>;

    #[test]
    fn conversions() {
        use crate::conversion::*;
        use typenum::UnsignedRational;

        assert_eq!(Conversion::<Feet, Yards>::F32, 1.0/3.0);
        assert_eq!(Conversion::<Miles, Feet>::F32, 5_280.0);

        assert_eq!(Conversion::<Hours, Seconds>::F32, 3_600.0);
    }
}
