use typenum::*;

/// A value that can apply a conversion factor
pub trait Convertible: Sized {
    fn convert<C: UnsignedRational>(&self) -> Self;

}

macro_rules! impl_conv_float {
    ($T:ty) => {
        impl Convertible for $T {
            fn convert<C: UnsignedRational>(&self) -> Self {
                (*self as f64 * C::F64) as Self
            }
        }
    };
    ($T:ty, $($Ts:ty),+) => {
        impl_conv_float!{$T}
        impl_conv_float!{$($Ts),+}
    }
}

impl_conv_float!{f32, f64, u32, i32, u64, i64}

pub trait ConversionTo<T> {
    type Factor: UnsignedRational;
}

pub type Conversion<From, To> = <From as ConversionTo<To>>::Factor;
