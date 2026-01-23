pub mod input;
pub mod schedule;

use core::ops::{Deref, DerefMut};

use bevy::{
    ecs::{
        schedule::{IntoScheduleConfigs, ScheduleLabel, Schedules},
        system::ScheduleSystem,
        world::World,
    },
    platform::prelude::vec::Vec,
};

use crate::main::{
    input::{FrameInput, UserInput},
    schedule::{FixedSchedule, SchedulePlugin},
};

pub trait DPlugin<I: UserInput> {
    fn build(&self, world: &mut Subworld<I>);
}

pub struct Subworld<I: UserInput = ()> {
    world: World,
    _phantom: core::marker::PhantomData<I>,
}

impl<I: UserInput> Default for Subworld<I> {
    fn default() -> Self {
        let world = World::new();
        let mut instance = Self {
            world,
            _phantom: core::marker::PhantomData,
        };

        instance.init_resource::<FrameInput<I>>();
        instance.add_plugin(SchedulePlugin);

        instance
    }
}

impl<I: UserInput> Subworld<I> {
    pub fn add_plugin(&mut self, plugin: impl DPlugin<I>) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, systems);

        self
    }

    pub fn sync(&mut self, rhs: &mut World, mut f: impl FnMut(&mut World, &mut World)) {
        f(&mut self.world, rhs);
    }

    pub fn tick(&mut self, input: Vec<I>) {
        self.world
            .get_resource_mut::<FrameInput<I>>()
            .unwrap()
            .set(input);

        self.world.run_schedule(FixedSchedule);
    }
}

impl<I: UserInput> Deref for Subworld<I> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl<I: UserInput> DerefMut for Subworld<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}
