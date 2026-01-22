use std::{
    f32,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Fx, IntoFx, fx};

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
    pub const fn const_new(x: Fx, y: Fx, z: Fx) -> Self {
        Self { x, y, z }
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

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: Fx::abs(self.x),
            y: Fx::abs(self.y),
            z: Fx::abs(self.z),
        }
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

    /// Returns a vector with a length no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `max` is negative when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length_max(self, max: impl IntoFx) -> Self {
        let max = max.into_fx();
        assert!(Fx::ZERO <= max);
        //glam_assert!(0.0 <= max);
        let length_sq = self.length_squared();
        if length_sq > max * max {
            (self / Fx::sqrt(length_sq)) * max
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn is_near_zero(&self) -> bool {
        let epsilon = fx!(f32::EPSILON);
        self.length_squared() < epsilon * epsilon
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

impl Index<usize> for FVec3 {
    type Output = Fx;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for FVec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
