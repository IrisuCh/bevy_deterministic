use bevy::prelude::*;

pub struct InputBuffer<I: UserInput> {
    ptr: *mut Vec<I>,
}

impl<I: UserInput> SystemInput for InputBuffer<I> {
    type Param<'i> = InputBuffer<I>;
    type Inner<'i> = InputBuffer<I>;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
        this
    }
}

pub trait UserInput: Send + Sync + 'static {}

impl UserInput for () {}

#[derive(Resource)]
pub struct FrameInput<I: UserInput> {
    history: Vec<I>,
    frame: Vec<I>,
}

impl<I: UserInput> Default for FrameInput<I> {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            frame: Vec::new(),
        }
    }
}

impl<I: UserInput> FrameInput<I> {
    pub(crate) fn set(&mut self, input: Vec<I>) {
        let temp = core::mem::take(&mut self.frame);
        self.history.extend(temp);
        self.frame = input;
    }

    pub(crate) fn as_buffer(&mut self) -> InputBuffer<I> {
        InputBuffer {
            ptr: &raw mut self.frame,
        }
    }
}
