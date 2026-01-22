mod action;
mod key;
mod macros;
mod xd;

use core::fmt::Debug;

use bevy::prelude::*;

use crate::input::{
    action::{
        Action, ActionType, handle_just_pressed_type, handle_just_released_type,
        handle_pressed_type,
    },
    key::Key,
};

pub mod prelude {
    pub use crate::input::{
        InputRegistry, InputSnapshot,
        action::{Action, ActionType},
        key::Key,
    };
}

pub struct InputPlugin<O: Send + Sync + 'static> {
    _phantom: core::marker::PhantomData<O>,
}

impl<O: Send + Sync + 'static> Default for InputPlugin<O> {
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<O: Send + Sync + 'static> Plugin for InputPlugin<O> {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<InputSnapshot<O>>();
        app.init_resource::<InputRegistry<O>>();
        app.add_systems(PreUpdate, keyboard_mouse_input::<O>);
    }
}

#[derive(Resource, Reflect)]
pub struct InputSnapshot<O> {
    inner: Vec<O>,
}

impl<O: Send + Sync + 'static> Default for InputSnapshot<O> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<O: Send + Sync + 'static> InputSnapshot<O> {
    #[must_use]
    pub const fn inner(&self) -> &[O] {
        self.inner.as_slice()
    }
}

#[derive(Resource)]
pub struct InputRegistry<O> {
    kind: InputActionType,
    actions: Vec<Action<O>>,
}

impl<O: Send + Sync + 'static> Default for InputRegistry<O> {
    fn default() -> Self {
        Self {
            kind: InputActionType::KeyboardMouse,
            actions: Vec::new(),
        }
    }
}

impl<O: Send + Sync + 'static> InputRegistry<O> {
    pub fn add_action<F>(
        &mut self,
        name: impl Into<String>,
        kind: ActionType,
        keys: Vec<Key>,
        output_factory: F,
    ) where
        F: Fn() -> O + Send + Sync + 'static,
    {
        self.actions
            .push(Action::new(name, kind, keys, Box::new(output_factory)));
    }
}

#[derive(Resource, Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputActionType {
    #[default]
    KeyboardMouse,
    Gamepad, //TODO
}

fn keyboard_mouse_input<O: Send + Sync + 'static>(
    mut snapshot: ResMut<InputSnapshot<O>>,
    registry: Res<InputRegistry<O>>,

    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    snapshot.inner.clear();

    if registry.kind != InputActionType::KeyboardMouse {
        return;
    }

    for action in &registry.actions {
        for key in &action.keys {
            match action.kind {
                ActionType::JustPressed => {
                    handle_just_pressed_type(&mut snapshot, action, key, &mouse, &keyboard);
                }
                ActionType::Pressed => {
                    handle_pressed_type(&mut snapshot, action, key, &mouse, &keyboard);
                }
                ActionType::JustReleased => {
                    handle_just_released_type(&mut snapshot, action, key, &mouse, &keyboard);
                }
            }
        }
    }
}

/*
 * |----------|-------|---------|-----|
 * | Keyboard | Mouse | Gamepad | ... |
 * |----------|-------|---------|-----|
 * |          |       |         |     |
 * |          |       |         |     |
 *
 *
 *  Keyboard input
 *  Forward - W
 *  Backward - S
 *  Left - A
 *  Right - D
 *  Dash - Z + C + Arrow Up
 *  Dash - Mouse 4
 *  Jump - Space
 */
