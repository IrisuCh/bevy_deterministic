use std::collections::HashMap;

use crate::collision::{Collider, CollisionSide};
use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};
use strum::EnumCount;

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
    pub(crate) fn clamp_negative_x(&mut self) {
        self.x = self.x.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// MIN..0
    pub(crate) fn clamp_negative_y(&mut self) {
        self.y = self.y.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// MIN..0
    pub(crate) fn clamp_negative_z(&mut self) {
        self.z = self.z.clamp(I32F32::MIN, I32F32::ZERO);
    }

    /// 0..MAX
    pub(crate) fn clamp_positive_x(&mut self) {
        self.x = self.x.clamp(I32F32::ZERO, I32F32::MAX);
    }

    /// 0..MAX
    pub(crate) fn clamp_positive_y(&mut self) {
        self.y = self.y.clamp(I32F32::ZERO, I32F32::MAX);
    }

    /// 0..MAX
    pub(crate) fn clamp_positive_z(&mut self) {
        self.z = self.z.clamp(I32F32::ZERO, I32F32::MAX);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct KinematicRigidBody {
    pub velocity: Velocity,
    pub freeze: bool,

    #[reflect(ignore)]
    history: EntityList,
    #[reflect(ignore)]
    offset_state: usize,
}

impl KinematicRigidBody {
    pub(crate) fn insert_other(&mut self, other: Entity, side: CollisionSide) {
        let count = self.history.count[side.index()];
        if count == 0 {
            self.apply_offset(side);
        }

        self.history.insert_other(other, side);
    }

    pub(crate) fn remove_other(&mut self, other: Entity) -> bool {
        if let Some((count, side)) = self.history.remove_other(other) {
            let is_last = count == 0;
            if is_last {
                self.remove_offset(side);
            }
            is_last
        } else {
            false
        }
    }

    pub(crate) fn is_history_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub(crate) fn has_other(&self, other: Entity) -> bool {
        self.history.has_other(other)
    }

    const fn remove_offset(&mut self, side: CollisionSide) {
        self.offset_state &= !(side as usize);
    }

    const fn apply_offset(&mut self, side: CollisionSide) {
        self.offset_state |= side as usize;
    }

    pub(crate) const fn is_offset_applied(&self, side: CollisionSide) -> bool {
        self.offset_state & (side as usize) != 0
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct StaticRigidBody;

#[derive(Reflect, Default, Debug)]
struct EntityList {
    map: HashMap<Entity, CollisionSide>,
    count: [usize; CollisionSide::COUNT],
}

impl EntityList {
    fn insert_other(&mut self, other: Entity, side: CollisionSide) {
        self.map.insert(other, side);
        self.count[side.index()] += 1;
    }

    fn remove_other(&mut self, other: Entity) -> Option<(usize, CollisionSide)> {
        if let Some(side) = self.map.remove(&other) {
            let index = side.index();
            self.count[index] -= 1;
            Some((self.count[index], side))
        } else {
            None
        }
    }

    fn change_side(&mut self, other: Entity, new: CollisionSide) {
        if let Some(side) = self.map.get_mut(&other) {
            self.count[side.index()] -= 1;
            *side = new;
            self.count[side.index()] += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn has_other(&self, other: Entity) -> bool {
        self.map.contains_key(&other)
    }
}
