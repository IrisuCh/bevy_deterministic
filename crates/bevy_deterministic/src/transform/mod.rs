mod fquat;
mod fvec3;

use bevy::prelude::*;

use crate::sync::SyncTarget;
pub use crate::transform::{fquat::FQuat, fvec3::FVec3};

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(FixedGlobalTransform)]
pub struct FixedTransform {
    pub position: FVec3,
    pub rotation: FQuat,
    pub size: FVec3,
}

impl Default for FixedTransform {
    fn default() -> Self {
        Self {
            position: FVec3::ZERO,
            rotation: FQuat::IDENTITY,
            size: FVec3::ONE,
        }
    }
}

impl FixedTransform {
    #[must_use]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: FVec3::new_f32(x, y, z),
            size: FVec3::new_f32(1.0, 1.0, 1.0),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn from_position_vec3(position: Vec3) -> Self {
        Self {
            position: FVec3::from_vec3(position),
            size: FVec3::new_f32(1.0, 1.0, 1.0),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_scale(mut self, scale: FVec3) -> Self {
        self.size = scale;
        self
    }

    #[must_use]
    pub fn with_scale_xyz(mut self, x: f32, y: f32, z: f32) -> Self {
        self.size = FVec3::new_f32(x, y, z);
        self
    }

    #[must_use]
    pub fn with_scale_vec3(mut self, scale: Vec3) -> Self {
        self.size = FVec3::from_vec3(scale);
        self
    }

    #[must_use]
    pub fn with_rotation(mut self, quat: FQuat) -> Self {
        self.rotation = quat;
        self
    }

    #[inline]
    pub fn rotate(&mut self, rotation: FQuat) {
        self.rotation = rotation * self.rotation;
    }

    #[inline]
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotate(FQuat::from_rotation_x(angle));
    }

    #[inline]
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotate(FQuat::from_rotation_y(angle));
    }

    #[inline]
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotate(FQuat::from_rotation_z(angle));
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedGlobalTransform {
    position: FVec3,
    rotation: FQuat,
    size: FVec3,
}

impl FixedGlobalTransform {
    #[must_use]
    pub const fn transform(&self) -> FixedTransform {
        FixedTransform {
            position: self.position,
            rotation: self.rotation,
            size: self.size,
        }
    }

    #[must_use]
    pub const fn position(&self) -> FVec3 {
        self.position
    }

    #[must_use]
    pub const fn size(&self) -> FVec3 {
        self.size
    }

    #[must_use]
    pub const fn rotation(&self) -> FQuat {
        self.rotation
    }
}

pub(crate) fn sync_fixed_global_transforms(
    query: Query<(&mut FixedGlobalTransform, &FixedTransform), Changed<FixedTransform>>,
) {
    for (mut global, local) in query {
        global.position = local.position;
        global.size = local.size;
        global.rotation = local.rotation;
    }
}

pub(crate) fn sync_fixed_transforms(
    query: Query<(Entity, &Children), Changed<FixedTransform>>,
    mut globals: Query<(&mut FixedGlobalTransform, &FixedTransform)>,
) {
    for (parent, children) in query {
        let (global_pos, global_size, global_rotation) = {
            let global = globals.get(parent).unwrap().0;
            (global.position, global.size, global.rotation)
        };
        for entity in children {
            if let Ok((mut child_global, child_local)) = globals.get_mut(*entity) {
                child_global.position = global_pos + child_local.position;
                child_global.size = global_size * child_local.size;
                child_global.rotation = (global_rotation * child_local.rotation).normalize();
            }
        }
    }
}

pub(crate) fn sync_transform(
    from: Query<&FixedTransform>,
    mut to: Query<(&mut Transform, &SyncTarget)>,
) {
    for (mut transform, sync) in &mut to {
        let fixed_transform = from.get(sync.0).unwrap();

        // Сдвигаем позицию к центру
        transform.translation =
            fixed_transform.position.as_vec3() + fixed_transform.size.as_vec3() / 2.0;
        transform.scale = fixed_transform.size.as_vec3();
        transform.rotation = fixed_transform.rotation.as_quat();
    }
}
