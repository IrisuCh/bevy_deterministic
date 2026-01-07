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
