use crate::collision::Collider;
use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct Velocity {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Velocity {
    /// MIN..0
    pub fn clamp_negative_x(&mut self) {
        self.x = self.x.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// MIN..0
    pub fn clamp_negative_y(&mut self) {
        self.y = self.y.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// MIN..0
    pub fn clamp_negative_z(&mut self) {
        self.z = self.z.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// 0..MAX
    pub fn clamp_positive_x(&mut self) {
        self.x = self.x.clamp(I32F32::ZERO, I32F32::MAX);
    }

    /// 0..MAX
    pub fn clamp_positive_y(&mut self) {
        self.y = self.y.clamp(I32F32::ZERO, I32F32::MAX);
    }

    /// 0..MAX
    pub fn clamp_positive_z(&mut self) {
        self.z = self.z.clamp(I32F32::ZERO, I32F32::MAX);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct KinematicRigidBody {
    pub velocity: Velocity,
    pub freeze: bool,
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct StaticRigidBody;
