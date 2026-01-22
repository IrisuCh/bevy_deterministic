use bevy::{
    self,
    app::{App, MainScheduleOrder},
    ecs::schedule::{ExecutorKind, ScheduleLabel},
    prelude::*,
};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct Input;

#[derive(Resource)]
pub struct FrameInput {
    inner: Vec<()>,
}

pub struct InputBuffer {
    ptr: *mut Vec<()>,
}

impl SystemInput for InputBuffer {
    type Param<'i> = InputBuffer;
    type Inner<'i> = InputBuffer;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
        this
    }
}

pub trait AppInputHelper {
    fn add_input_provider<M, S>(self, system: S) -> Self
    where
        S: IntoSystem<InputBuffer, (), M>;
}

impl AppInputHelper for App {
    fn add_input_provider<M, S>(mut self, system: S) -> Self
    where
        S: IntoSystem<InputBuffer, (), M>,
    {
        self.add_systems(
            Input,
            (|mut res: ResMut<FrameInput>| InputBuffer {
                ptr: &raw mut res.inner,
            })
            .pipe(system),
        );

        self
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        let mut custom_update_schedule = Schedule::new(Input);
        custom_update_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.add_schedule(custom_update_schedule);

        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_before(FixedUpdate, Input);
    }
}
