//TODO global size

use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct Size {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Size {
    pub fn new_f32(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: I32F32::from_num(x),
            y: I32F32::from_num(y),
            z: I32F32::from_num(z),
        }
    }

    pub const fn new_fixed(x: I32F32, y: I32F32, z: I32F32) -> Self {
        Self { x, y, z }
    }

    pub fn x_f32(&self) -> f32 {
        self.x.to_num::<f32>()
    }

    pub fn y_f32(&self) -> f32 {
        self.y.to_num::<f32>()
    }

    pub fn z_f32(&self) -> f32 {
        self.z.to_num::<f32>()
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x.to_num::<f32>(),
            self.y.to_num::<f32>(),
            self.z.to_num::<f32>(),
        )
    }
}
