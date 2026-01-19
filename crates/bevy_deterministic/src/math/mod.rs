mod direction;
mod fquat;
mod fvec3;

use std::f32;

pub use direction::FDir3;
pub use fquat::FQuat;
pub use fvec3::FVec3;
pub use fx::{Fx, IntoFx, const_fx, fx};

#[inline]
pub fn fx_epsilon() -> Fx {
    fx!(f32::EPSILON)
}
