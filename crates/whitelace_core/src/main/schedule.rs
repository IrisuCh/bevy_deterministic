use bevy::{
    ecs::schedule::{ExecutorKind, InternedScheduleLabel, Schedule, ScheduleLabel},
    prelude::*,
};

use crate::main::{DPlugin, Subworld, input::UserInput};

#[derive(Resource, Debug)]
struct ScheduleOrder {
    labels: Vec<InternedScheduleLabel>,
}

impl Default for ScheduleOrder {
    fn default() -> Self {
        Self {
            labels: vec![Physics.intern(), Logic.intern()],
        }
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct FixedSchedule;

impl FixedSchedule {
    pub fn run(world: &mut World) {
        world.resource_scope(|world, order: Mut<ScheduleOrder>| {
            for &label in &order.labels {
                let _ = world.try_run_schedule(label);
            }
        });
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Physics;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Logic;

pub struct SchedulePlugin;
impl<I: UserInput> DPlugin<I> for SchedulePlugin {
    fn build(&self, app: &mut Subworld<I>) {
        let mut fixed_schedule = Schedule::new(FixedSchedule);
        fixed_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.init_resource::<ScheduleOrder>();
        app.add_schedule(fixed_schedule);
        app.add_systems(FixedSchedule, FixedSchedule::run);
    }
}
