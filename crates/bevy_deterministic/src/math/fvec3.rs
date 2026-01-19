use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use bevy::prelude::*;
use fx::IntoFx;
use serde::{Deserialize, Serialize};

use crate::math::Fx;

#[derive(
    Reflect,
    Serialize,
    Deserialize,
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct FVec3 {
    pub x: Fx,
    pub y: Fx,
    pub z: Fx,
}

impl FVec3 {
    /// All zeroes.
    pub const ZERO: Self = Self {
        x: Fx::ZERO,
        y: Fx::ZERO,
        z: Fx::ZERO,
    };

    /// All ones.
    pub const ONE: Self = Self {
        x: Fx::const_from_int(1),
        y: Fx::const_from_int(1),
        z: Fx::const_from_int(1),
    };

    /// All negative ones.
    pub const NEG_ONE: Self = Self {
        x: Fx::const_from_int(-1),
        y: Fx::const_from_int(-1),
        z: Fx::const_from_int(-1),
    };

    /// All `Fx::MIN`.
    pub const MIN: Self = Self {
        x: Fx::MIN,
        y: Fx::MIN,
        z: Fx::MIN,
    };

    /// All `Fx::MAX`.
    pub const MAX: Self = Self {
        x: Fx::MAX,
        y: Fx::MAX,
        z: Fx::MAX,
    };

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: Fx::const_from_int(1),
        y: Fx::const_from_int(0),
        z: Fx::const_from_int(0),
    };

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: Fx::const_from_int(0),
        y: Fx::const_from_int(1),
        z: Fx::const_from_int(0),
    };

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: Fx::const_from_int(0),
        y: Fx::const_from_int(0),
        z: Fx::const_from_int(1),
    };

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: Fx::const_from_int(-1),
        y: Fx::const_from_int(0),
        z: Fx::const_from_int(0),
    };

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: Fx::const_from_int(0),
        y: Fx::const_from_int(-1),
        z: Fx::const_from_int(0),
    };

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self {
        x: Fx::const_from_int(0),
        y: Fx::const_from_int(0),
        z: Fx::const_from_int(-1),
    };

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    #[must_use]
    pub fn new(x: impl IntoFx, y: impl IntoFx, z: impl IntoFx) -> Self {
        Self {
            x: x.into_fx(),
            y: y.into_fx(),
            z: z.into_fx(),
        }
    }

    #[must_use]
    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            x: Fx::from_num(vec.x),
            y: Fx::from_num(vec.y),
            z: Fx::from_num(vec.z),
        }
    }

    #[must_use]
    pub fn x_f32(&self) -> f32 {
        self.x.to_num::<f32>()
    }

    #[must_use]
    pub fn y_f32(&self) -> f32 {
        self.y.to_num::<f32>()
    }

    #[must_use]
    pub fn z_f32(&self) -> f32 {
        self.z.to_num::<f32>()
    }

    #[must_use]
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x.to_num::<f32>(),
            self.y.to_num::<f32>(),
            self.z.to_num::<f32>(),
        )
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> Fx {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self.mul(self.length_recip())
    }

    #[inline]
    #[must_use]
    pub fn length_recip(self) -> Fx {
        self.length().recip()
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> Fx {
        self.dot(self).sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalize_or_zero(self) -> Self {
        let len_sq = self.length_squared();
        if len_sq == Fx::ZERO {
            Self::ZERO
        } else {
            self.mul(len_sq.sqrt().recip())
        }
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> Fx {
        self.dot(self)
    }
}

impl Add for FVec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for FVec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for FVec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Sub for FVec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for FVec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Fx> for FVec3 {
    type Output = Self;

    fn mul(self, rhs: Fx) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div for FVec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<Fx> for FVec3 {
    type Output = Self;

    fn div(self, rhs: Fx) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for FVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<Vec3> for FVec3 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl MulAssign for FVec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<Fx> for FVec3 {
    fn mul_assign(&mut self, rhs: Fx) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
