pub mod input;
pub mod schedule;

use bevy::{
    ecs::{
        bundle::Bundle,
        event::Event,
        resource::Resource,
        schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, Schedules},
        system::{IntoObserverSystem, IntoSystem, ScheduleSystem},
        world::{EntityWorldMut, FromWorld, World},
    },
    platform::prelude::vec::Vec,
};

use crate::main::{
    input::{FrameInput, InputBuffer, UserInput},
    schedule::{FixedSchedule, SchedulePlugin},
};

pub trait DPlugin<I: UserInput> {
    fn build(&self, world: &mut DeterministicWorld<I>);
}

pub struct DeterministicWorld<I: UserInput> {
    world: World,
    _phantom: core::marker::PhantomData<I>,
}

impl<I: UserInput> Default for DeterministicWorld<I> {
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

impl<I: UserInput> DeterministicWorld<I> {
    pub fn add_plugin(&mut self, plugin: impl DPlugin<I>) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn add_schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.world.add_schedule(schedule);
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

    pub fn add_observer<E, B, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> EntityWorldMut<'_>
    where
        E: Event,
        B: Bundle,
    {
        self.world.add_observer(system)
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
