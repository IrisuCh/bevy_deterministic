use bevy::prelude::*;
use fixed_math::{Fx, fx};

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

#[allow(clippy::disallowed_types)]
pub fn sync_time(det: &mut World, rhs: &mut World) {
    let time = rhs.get_resource::<bevy::prelude::Time<Fixed>>().unwrap();

    let mut fixed_time = det.get_resource_mut::<Time>().unwrap();
    fixed_time.delta_time = fx!(time.delta_secs());
}
