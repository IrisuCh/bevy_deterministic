use bevy::prelude::*;

#[derive(Component, Debug, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SyncTarget(pub Entity);
