use bevy::prelude::*;
use strum::EnumCount;
use whitelace_core::{map::Map, math::FVec3};
use whitelace_math::{Fx, fx};
use whitelace_transform::FixedTransform;

use crate::prelude::CollisionSide;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[require(FixedTransform)]
pub struct Collider {
    pub trigger: bool,
    pub disabled: bool,
    pub fixed: bool,
    pub center: FVec3,
    pub size: FVec3,
    pub material: ColliderMaterial,

    pub(crate) contacts: Contacts,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            trigger: false,
            disabled: false,
            fixed: false,
            center: FVec3::ZERO,
            size: FVec3::ONE,
            material: ColliderMaterial::default(),
            contacts: Contacts::default(),
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
    pub(crate) fn insert_other(&mut self, other: Entity, contact: SurfaceContact) {
        self.contacts.insert_other(other, contact);
    }

    #[inline]
    pub(crate) fn remove_other(&mut self, other: Entity) -> bool {
        if let Some(count) = self.contacts.remove_other(other) {
            count == 0
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn has_other(&self, other: Entity) -> bool {
        self.contacts.has_other(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceContact {
    pub entity: Entity,
    pub contact_point: FVec3,
    pub contact_normal: FVec3,
    pub penetration_depth: Fx,
    pub relative_velocity: FVec3,
    pub last_update_frame: u64,
    pub side: CollisionSide,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Contacts {
    pub(crate) map: Map<Entity, SurfaceContact>,
    count: [usize; CollisionSide::COUNT],
}

impl Contacts {
    fn insert_other(&mut self, other: Entity, contact: SurfaceContact) {
        let side = contact.side;
        self.map.insert(other, contact);
        self.count[side.index()] += 1;
    }

    fn remove_other(&mut self, other: Entity) -> Option<usize> {
        if let Some(contact) = self.map.shift_remove(&other) {
            let index = contact.side.index();
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColliderMaterial {
    /// Коэффициент трения (0-1)
    /// 0 = абсолютно гладкий (лед), 1 = очень шероховатый
    pub friction: Fx,

    /// Упругость/восстановление (0-1)
    /// 0 = абсолютно неупругий (мягкий), 1 = абсолютно упругий (резиновый мяч)
    pub restitution: Fx,

    /// Сопротивление качению (для сфер, цилиндров)
    pub rolling_resistance: Fx,

    /// Прилипание/адгезия (дополнительная сила прилипания к поверхностям)
    pub adhesion: Fx,
    // Флаги для специального поведения
    //pub flags: ColliderMaterialFlags,
}

impl Default for ColliderMaterial {
    fn default() -> Self {
        Self {
            friction: fx!(0.4),    // Среднее трение (как дерево)
            restitution: fx!(0.3), // Немного упругий
            rolling_resistance: fx!(0.01),
            adhesion: fx!(0.0), // Нет прилипания
                                //flags: ColliderMaterialFlags::empty(),
        }
    }
}
