use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

use crate::Fx;

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
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl FVec3 {
    pub const ZERO: Self = Self {
        x: I32F32::ZERO,
        y: I32F32::ZERO,
        z: I32F32::ZERO,
    };

    pub const ONE: Self = Self {
        x: I32F32::const_from_int(1),
        y: I32F32::const_from_int(1),
        z: I32F32::const_from_int(1),
    };

    #[must_use]
    pub fn new_fixed(x: I32F32, y: I32F32, z: I32F32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn new_f32(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: I32F32::from_num(x),
            y: I32F32::from_num(y),
            z: I32F32::from_num(z),
        }
    }

    #[must_use]
    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            x: I32F32::from_num(vec.x),
            y: I32F32::from_num(vec.y),
            z: I32F32::from_num(vec.z),
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
