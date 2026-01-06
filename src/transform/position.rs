use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
#[require(GlobalPosition)]
pub struct Position {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Position {
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
    pub fn new_vec3(vec: Vec3) -> Self {
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
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct GlobalPosition {
    pub(super) x: I32F32,
    pub(super) y: I32F32,
    pub(super) z: I32F32,
}

impl GlobalPosition {
    #[must_use]
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x.to_num::<f32>(),
            self.y.to_num::<f32>(),
            self.z.to_num::<f32>(),
        )
    }

    #[must_use]
    pub const fn x(&self) -> I32F32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> I32F32 {
        self.y
    }

    #[must_use]
    pub const fn z(&self) -> I32F32 {
        self.z
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
    pub fn as_position(&self) -> Position {
        Position::new_fixed(self.x, self.y, self.z)
    }
}
