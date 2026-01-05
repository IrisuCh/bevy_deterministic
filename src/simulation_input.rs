use bevy::prelude::*;
use fixed::types::I32F32;

pub struct SimulationInputPlugin;

impl Plugin for SimulationInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (update_world_mouse_position, make_input_snapshot));
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct WorldMousePosition {
    x: I32F32,
    y: I32F32,
}

impl WorldMousePosition {
    pub const fn x(&self) -> I32F32 { self.x }
    pub const fn y(&self) -> I32F32 { self.y }
}

fn update_world_mouse_position(
    mut res: ResMut<WorldMousePosition>,
    window: Single<&Window>,
    mut last_cursor_pos: Local<Option<Vec2>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
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

#[derive(Clone, Copy)]
pub enum Action {
    Click,
    Jump,
    Fire,
    Left,
    Right,
    Idle,
}

#[derive(Resource, Default, Clone)]
pub struct InputSnapshot {
    inner: Vec<Action>,
}

impl InputSnapshot {
    pub const fn inner(&self) -> &[Action] { self.inner.as_slice() }
}

fn make_input_snapshot(
    mut snapshot: ResMut<InputSnapshot>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    snapshot.inner.clear();
    if keys.just_pressed(KeyCode::KeyZ) {
        snapshot.inner.push(Action::Jump);
    }

    if keys.just_pressed(KeyCode::KeyX) {
        snapshot.inner.push(Action::Fire);
    }

    let mut idle = true;
    if keys.pressed(KeyCode::Slash) {
        snapshot.inner.push(Action::Left);
        idle = false;
    }

    if keys.pressed(KeyCode::Comma) {
        snapshot.inner.push(Action::Right);
        idle = false;
    }

    if idle {
        snapshot.inner.push(Action::Idle);
    }

    if mouse.just_pressed(MouseButton::Left) {
        snapshot.inner.push(Action::Click);
    }
}