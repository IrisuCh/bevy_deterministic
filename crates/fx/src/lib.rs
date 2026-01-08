#![allow(clippy::inline_always)]

pub type Fx = fixed::types::I32F32;

#[macro_export]
macro_rules! fx {
    ($n:expr) => {
        $crate::Fx::from_num($n)
    };
}

#[macro_export]
macro_rules! const_fx {
    ($n:expr) => {
        $crate::Fx::const_from_int($n)
    };
}

pub trait IntoFx {
    fn into_fx(self) -> Fx;
}

impl IntoFx for i32 {
    fn into_fx(self) -> Fx {
        Fx::from_num(self)
    }
}

impl IntoFx for f32 {
    #[inline(always)]
    fn into_fx(self) -> Fx {
        Fx::from_num(self)
    }
}

impl IntoFx for Fx {
    #[inline(always)]
    fn into_fx(self) -> Fx {
        self
    }
}

impl IntoFx for f64 {
    #[inline(always)]
    fn into_fx(self) -> Fx {
        Fx::from_num(self)
    }
}
