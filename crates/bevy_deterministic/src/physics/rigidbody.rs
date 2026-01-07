use bevy::prelude::*;
use strum::EnumCount;

use crate::{
    DetMap,
    physics::collision::{Collider, CollisionSide},
    transform::FVec3,
};

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct KinematicRigidBody {
    pub velocity: FVec3,
    pub freeze: bool,

    #[reflect(ignore)]
    history: EntityList,
}

impl KinematicRigidBody {
    #[inline]
    pub(crate) fn insert_other(&mut self, other: Entity, side: CollisionSide) {
        self.history.insert_other(other, side);
    }

    #[inline]
    pub(crate) fn remove_other(&mut self, other: Entity) -> bool {
        if let Some(count) = self.history.remove_other(other) {
            count == 0
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn has_other(&self, other: Entity) -> bool {
        self.history.has_other(other)
    }
}

#[derive(Default, Debug)]
struct EntityList {
    map: DetMap<Entity, CollisionSide>,
    count: [usize; CollisionSide::COUNT],
}

impl EntityList {
    fn insert_other(&mut self, other: Entity, side: CollisionSide) {
        self.map.insert(other, side);
        self.count[side.index()] += 1;
    }

    fn remove_other(&mut self, other: Entity) -> Option<usize> {
        if let Some(side) = self.map.shift_remove(&other) {
            let index = side.index();
            self.count[index] -= 1;
            Some(self.count[index])
        } else {
            None
        }
    }

    fn has_other(&self, other: Entity) -> bool {
        self.map.contains_key(&other)
    }
}
