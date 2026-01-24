#![allow(clippy::needless_pass_by_value)]
#![no_std]

use bevy::prelude::*;
use whitelace_core::main::schedule::PreFixedUpdate;
use whitelace_math::{FDir3, FQuat, FVec3, IntoFx};
use whitelace_sync::{MultiworldApp, SyncTarget, WorldLabel, WorldQuery};

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
    pub fn from_xyz(x: impl IntoFx, y: impl IntoFx, z: impl IntoFx) -> Self {
        Self {
            position: FVec3::new(x, y, z),
            size: FVec3::new(1, 1, 1),
            ..Self::default()
        }
    }

    pub fn from_vec3(position: impl Into<FVec3>) -> Self {
        Self {
            position: position.into(),
            size: FVec3::new(1, 1, 1),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_scale(mut self, scale: impl Into<FVec3>) -> Self {
        self.size = scale.into();
        self
    }

    #[must_use]
    pub fn with_scale_xyz(mut self, x: impl IntoFx, y: impl IntoFx, z: impl IntoFx) -> Self {
        self.size = FVec3::new(x, y, z);
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
    pub fn rotate_x(&mut self, angle: impl IntoFx) {
        self.rotate(FQuat::from_rotation_x(angle));
    }

    #[inline]
    pub fn rotate_y(&mut self, angle: impl IntoFx) {
        self.rotate(FQuat::from_rotation_y(angle));
    }

    #[inline]
    pub fn rotate_z(&mut self, angle: impl IntoFx) {
        self.rotate(FQuat::from_rotation_z(angle));
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    #[must_use]
    pub fn local_x(&self) -> FDir3 {
        // Quat * unit vector is length 1
        FDir3::new_unchecked(self.rotation * FVec3::X)
    }

    /// Equivalent to [`-local_x()`][Transform::local_x()]
    #[inline]
    #[must_use]
    pub fn left(&self) -> FDir3 {
        -self.local_x()
    }

    /// Equivalent to [`local_x()`][Transform::local_x()]
    #[inline]
    #[must_use]
    pub fn right(&self) -> FDir3 {
        self.local_x()
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    #[must_use]
    pub fn local_y(&self) -> FDir3 {
        // Quat * unit vector is length 1
        FDir3::new_unchecked(self.rotation * FVec3::Y)
    }

    /// Equivalent to [`local_y()`][Transform::local_y]
    #[inline]
    #[must_use]
    pub fn up(&self) -> FDir3 {
        self.local_y()
    }

    /// Equivalent to [`-local_y()`][Transform::local_y]
    #[inline]
    #[must_use]
    pub fn down(&self) -> FDir3 {
        -self.local_y()
    }

    /// Get the unit vector in the local `Z` direction.
    #[inline]
    #[must_use]
    pub fn local_z(&self) -> FDir3 {
        // Quat * unit vector is length 1
        FDir3::new_unchecked(self.rotation * FVec3::Z)
    }

    /// Equivalent to [`-local_z()`][Transform::local_z]
    #[inline]
    #[must_use]
    pub fn forward(&self) -> FDir3 {
        -self.local_z()
    }

    /// Equivalent to [`local_z()`][Transform::local_z]
    #[inline]
    #[must_use]
    pub fn back(&self) -> FDir3 {
        self.local_z()
    }
}

impl From<(FVec3, FQuat, FVec3)> for FixedTransform {
    fn from((position, rotation, size): (FVec3, FQuat, FVec3)) -> Self {
        Self {
            position,
            rotation,
            size,
        }
    }
}

impl From<FixedTransform> for (FVec3, FQuat, FVec3) {
    fn from(val: FixedTransform) -> Self {
        (val.position, val.rotation, val.size)
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
    pub const fn as_local(&self) -> FixedTransform {
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

pub struct TransformPlugin<W: WorldLabel> {
    _marker: core::marker::PhantomData<W>,
}

impl<W: WorldLabel> Default for TransformPlugin<W> {
    fn default() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

impl<W: WorldLabel + Default> Plugin for TransformPlugin<W> {
    fn build(&self, app: &mut App) {
        app.add_world_systems(
            W::default(),
            PreFixedUpdate,
            (sync_fixed_global_transforms, sync_fixed_transforms),
        );

        app.add_sync_system(sync_transform::<W>);
    }
}

fn sync_transform<W: WorldLabel>(
    from: WorldQuery<&FixedTransform, (), W>,
    mut to: WorldQuery<(&mut Transform, &SyncTarget)>,
) {
    for (mut transform, sync) in &mut to.iter_mut() {
        let fixed_transform = from.get(sync.0).unwrap();

        transform.translation =
            fixed_transform.position.as_vec3() + fixed_transform.size.as_vec3() / 2.0;
        transform.scale = fixed_transform.size.as_vec3();
        transform.rotation = fixed_transform.rotation.as_quat();
    }
}
