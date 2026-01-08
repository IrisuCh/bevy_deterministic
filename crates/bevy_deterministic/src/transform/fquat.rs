use std::ops::Mul;

use bevy::prelude::*;
use fx::IntoFx;

use crate::{Fx, fx, transform::FVec3};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FQuat {
    pub x: Fx,
    pub y: Fx,
    pub z: Fx,
    pub w: Fx,
}

impl FQuat {
    pub const IDENTITY: Self = Self {
        x: Fx::ZERO,
        y: Fx::ZERO,
        z: Fx::ZERO,
        w: Fx::ONE,
    };

    #[must_use]
    pub fn from_xyzw(x: impl IntoFx, y: impl IntoFx, z: impl IntoFx, w: impl IntoFx) -> Self {
        Self {
            x: x.into_fx(),
            y: y.into_fx(),
            z: z.into_fx(),
            w: w.into_fx(),
        }
    }

    #[must_use]
    pub fn as_quat(&self) -> Quat {
        let x = self.x.to_num::<f32>();
        let y = self.y.to_num::<f32>();
        let z = self.z.to_num::<f32>();
        let w = self.w.to_num::<f32>();
        Quat::from_xyzw(x, y, z, w)
    }

    /// Dot product
    #[must_use]
    pub fn dot(&self, other: Self) -> Fx {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Length squared
    #[must_use]
    pub fn length_squared(&self) -> Fx {
        self.dot(*self)
    }

    /// Normalize the quaternion
    #[must_use]
    pub fn normalize(&self) -> Self {
        let length_squared = self.length_squared();
        if length_squared == Fx::ZERO {
            return Self::IDENTITY;
        }

        let inv_length = Fx::ONE / length_squared.sqrt();
        Self {
            x: self.x * inv_length,
            y: self.y * inv_length,
            z: self.z * inv_length,
            w: self.w * inv_length,
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: impl IntoFx) -> Self {
        let angle = angle.into_fx();
        let (s, c) = cordic::sin_cos(angle * fx!(0.5));
        Self::from_xyzw(s, Fx::ZERO, Fx::ZERO, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: impl IntoFx) -> Self {
        let angle = angle.into_fx();
        let (s, c) = cordic::sin_cos(angle * fx!(0.5));
        Self::from_xyzw(Fx::ZERO, s, Fx::ZERO, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: impl IntoFx) -> Self {
        let angle = angle.into_fx();
        let (s, c) = cordic::sin_cos(angle * fx!(0.5));
        Self::from_xyzw(Fx::ZERO, Fx::ZERO, s, c)
    }

    /// Сопряжённый кватернион (обратное вращение)
    #[inline]
    #[must_use]
    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x, // меняем знак у векторной части
            y: -self.y,
            z: -self.z,
            w: self.w, // скалярная часть без изменений
        }
    }

    #[must_use]
    pub fn rotate_vec3(&self, vec: FVec3) -> FVec3 {
        // q * v * q⁻¹
        // где v — чистый кватернион (w=0)

        // Конвертируем вектор в чистый кватернион
        let v = FQuat::from_xyzw(vec.x, vec.y, vec.z, Fx::ZERO);

        // Вычисляем: self * v * self.conjugate()
        let self_conj = self.conjugate();
        let result = (*self * v) * self_conj;

        // Возвращаем векторную часть
        FVec3::new(result.x, result.y, result.z)
    }
}

// Правильное умножение кватернионов (Hamilton product)
impl Mul for FQuat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}

// Умножение на скаляр
impl Mul<Fx> for FQuat {
    type Output = Self;

    fn mul(self, scalar: Fx) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<FVec3> for FQuat {
    type Output = FVec3;

    fn mul(self, vec: FVec3) -> Self::Output {
        self.rotate_vec3(vec)
    }
}
