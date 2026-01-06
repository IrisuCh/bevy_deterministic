use bevy::prelude::*;
use fixed::types::I32F32;

use crate::{
    input::prelude::{ActionType, InputRegistry, Key},
    keys,
};

#[derive(Clone, Copy)]
pub enum Action {
    Click,
    Jump,
    Fire,
    Left,
    Right,
}

pub struct SimulationInputPlugin;

impl Plugin for SimulationInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_world_mouse_position);

        let mut registry = app.world_mut().resource_mut::<InputRegistry<Action>>();
        registry.add_action("Jump", ActionType::JustPressed, keys![KeyZ], || {
            Action::Jump
        });
        registry.add_action("Fire", ActionType::JustPressed, keys![KeyX], || {
            Action::Fire
        });

        registry.add_action("Left", ActionType::Pressed, keys![Slash], || Action::Left);
        registry.add_action("Right", ActionType::Pressed, keys![Comma], || Action::Right);

        registry.add_action(
            "Click",
            ActionType::JustPressed,
            vec![Key::from_mouse_index(0)],
            || Action::Click,
        );
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct WorldMousePosition {
    x: I32F32,
    y: I32F32,
}

impl WorldMousePosition {
    pub const fn x(&self) -> I32F32 {
        self.x
    }
    pub const fn y(&self) -> I32F32 {
        self.y
    }
}

fn update_world_mouse_position(
    mut res: ResMut<WorldMousePosition>,
    window: Single<&Window>,
    mut last_cursor_pos: Local<Option<Vec2>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();

    let cursor_pos = window
        .cursor_position()
        .unwrap_or_else(|| (*last_cursor_pos).unwrap_or_default());
    *last_cursor_pos = Some(cursor_pos);

    let world_pos = camera
        .viewport_to_world_2d(camera_transform, cursor_pos)
        .unwrap_or_default();

    res.x = I32F32::from_num(world_pos.x);
    res.y = I32F32::from_num(world_pos.y);
}
