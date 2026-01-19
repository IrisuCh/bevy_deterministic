use bevy::prelude::*;

use crate::math::{Fx, fx};

#[derive(Resource, Default)]
pub struct FixedTime {
    delta_time: Fx,
}

impl FixedTime {
    #[must_use]
    pub const fn delta_time(&self) -> Fx {
        self.delta_time
    }
}

#[allow(clippy::disallowed_types)]
pub(super) fn sync_time(res: Res<Time<Fixed>>, mut fixed_time: ResMut<FixedTime>) {
    fixed_time.delta_time = fx!(res.delta_secs());
}
