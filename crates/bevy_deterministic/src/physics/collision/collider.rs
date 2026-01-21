use bevy::prelude::*;
use strum::EnumCount;

use crate::{DetMap, math::FVec3, physics::prelude::CollisionSide, transform::FixedTransform};

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[require(FixedTransform)]
pub struct Collider {
    pub trigger: bool,
    pub disabled: bool,
    pub fixed: bool,
    pub center: FVec3,
    pub size: FVec3,

    history: EntityList,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            trigger: false,
            disabled: false,
            fixed: false,
            center: FVec3::ZERO,
            size: FVec3::ONE,
            history: EntityList::default(),
        }
    }
}

impl Collider {
    #[must_use]
    pub fn trigger() -> Self {
        Self {
            trigger: true,
            disabled: false,
            ..default()
        }
    }

    #[must_use]
    pub fn disabled() -> Self {
        Self {
            trigger: false,
            disabled: true,
            ..default()
        }
    }

    #[must_use]
    pub fn fixed() -> Self {
        Self {
            trigger: false,
            disabled: false,
            fixed: true,
            ..default()
        }
    }

    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn with_trigger(mut self, trigger: bool) -> Self {
        self.trigger = trigger;
        self
    }

    #[must_use]
    pub fn with_fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    #[must_use]
    pub fn with_size(mut self, size: FVec3) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn with_center(mut self, center: FVec3) -> Self {
        self.center = center;
        self
    }

    #[must_use]
    pub fn transform(&self, transform: &FixedTransform) -> FixedTransform {
        let mut transform = transform.clone();
        transform.position += self.center;
        transform.size *= self.size;
        transform
    }

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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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
