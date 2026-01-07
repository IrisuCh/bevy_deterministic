use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct Key {
    pub keyboard: Vec<KeyCode>,
    pub mouse: Option<MouseButton>,
    //.. TODO Gamepad,
    //modifier: Option<KeyCode>,
}

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        Self {
            keyboard: vec![code],
            mouse: None,
        }
    }
}

impl From<MouseButton> for Key {
    fn from(btn: MouseButton) -> Self {
        Self {
            keyboard: vec![],
            mouse: Some(btn),
        }
    }
}

impl Key {
    #[inline]
    #[must_use]
    pub const fn from_mouse_index(index: u16) -> Self {
        let mouse = match index {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Right),
            2 => Some(MouseButton::Middle),
            3 => Some(MouseButton::Back),
            4 => Some(MouseButton::Forward),
            n => Some(MouseButton::Other(n)),
        };
        Self {
            keyboard: vec![],
            mouse,
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_kb(keys: Vec<KeyCode>) -> Self {
        Self {
            keyboard: keys,
            mouse: None,
        }
    }
}
