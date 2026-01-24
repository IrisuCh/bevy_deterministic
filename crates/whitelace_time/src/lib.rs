//TODO Time/Tick manager
#![allow(clippy::needless_pass_by_value)]
#![no_std]

use bevy::prelude::*;
use whitelace_math::{Fx, fx};
use whitelace_sync::{MultiworldApp, WorldLabel, WorldRes, WorldResMut};

#[derive(Resource, Default)]
pub struct Time {
    delta_time: Fx,
}

impl Time {
    #[must_use]
    #[inline]
    pub const fn delta_time(&self) -> Fx {
        self.delta_time
    }
}

pub struct TimePlugin<W: WorldLabel> {
    _phantom: core::marker::PhantomData<W>,
}

impl<W: WorldLabel> Default for TimePlugin<W> {
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<W: WorldLabel + Default> Plugin for TimePlugin<W> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Time>();
        app.add_sync_system(sync_time::<W>);
    }
}

#[allow(clippy::disallowed_types)]
fn sync_time<W: WorldLabel>(
    from: WorldRes<bevy::prelude::Time<Fixed>>,
    mut to: WorldResMut<Time, W>,
) {
    to.delta_time = fx!(from.delta_secs_f64());
}
