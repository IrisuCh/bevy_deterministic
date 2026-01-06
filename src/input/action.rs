use bevy::prelude::*;

use crate::input::{InputSnapshot, key::Key};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionType {
    #[default]
    JustPressed,
    Pressed,
    JustReleased,
}

pub(super) fn handle_just_pressed_type<O: Send + Sync + 'static>(
    snapshot: &mut InputSnapshot<O>,
    action: &Action<O>,
    key: &Key,
    mouse: &ButtonInput<MouseButton>,
    keyboard: &ButtonInput<KeyCode>,
) {
    if key
        .keyboard
        .iter()
        .all(|keycode| keyboard.just_pressed(*keycode))
        && !key.keyboard.is_empty()
    {
        snapshot.inner.push((action.output_factory)());
    }

    if let Some(button) = key.mouse
        && mouse.just_pressed(button)
    {
        snapshot.inner.push((action.output_factory)());
    }
}

pub(super) fn handle_pressed_type<O: Send + Sync + 'static>(
    snapshot: &mut InputSnapshot<O>,
    action: &Action<O>,
    key: &Key,
    mouse: &ButtonInput<MouseButton>,
    keyboard: &ButtonInput<KeyCode>,
) {
    if key
        .keyboard
        .iter()
        .all(|keycode| keyboard.pressed(*keycode))
        && !key.keyboard.is_empty()
    {
        snapshot.inner.push((action.output_factory)());
    }

    if let Some(button) = key.mouse
        && mouse.pressed(button)
    {
        snapshot.inner.push((action.output_factory)());
    }
}

pub(super) fn handle_just_released_type<O: Send + Sync + 'static>(
    snapshot: &mut InputSnapshot<O>,
    action: &Action<O>,
    key: &Key,
    mouse: &ButtonInput<MouseButton>,
    keyboard: &ButtonInput<KeyCode>,
) {
    if key
        .keyboard
        .iter()
        .all(|keycode| keyboard.just_released(*keycode))
        && !key.keyboard.is_empty()
    {
        snapshot.inner.push((action.output_factory)());
    }

    if let Some(button) = key.mouse
        && mouse.just_released(button)
    {
        snapshot.inner.push((action.output_factory)());
    }
}

pub struct Action<O> {
    pub(crate) name: String,
    pub(crate) kind: ActionType,
    pub(crate) keys: Vec<Key>,
    pub(crate) output_factory: Box<dyn Fn() -> O + Send + Sync + 'static>,
}

impl<O> Action<O> {
    pub(crate) fn new(
        name: impl Into<String>,
        kind: ActionType,
        keys: Vec<Key>,
        output_factory: Box<dyn Fn() -> O + Send + Sync + 'static>,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            keys,
            output_factory,
        }
    }

    pub const fn name(&self) -> &str {
        self.name.as_str()
    }
}
