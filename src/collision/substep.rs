use fixed::types::I32F32;
use crate::collision::aabb::AABB;
use crate::determinism::transform::{GlobalPosition, Size};

const STEPS: I32F32 = I32F32::const_from_int(16);

pub struct SubstepIterator<'a> {
    position: &'a GlobalPosition,
    size: &'a Size,
    step: (I32F32, I32F32),
    steps: I32F32,
    completed: I32F32,
}

impl<'a> SubstepIterator<'a> {
    pub fn new(
        position: &'a GlobalPosition,
        size: &'a Size,
        velocity_x: I32F32,
        velocity_y: I32F32,
    ) -> Self {
        let x_step = velocity_x / STEPS;
        let y_step = velocity_y / STEPS;
        let step = (x_step, y_step);
        Self { position, size, step, steps: STEPS, completed: I32F32::ZERO }
    }

    pub fn next_overlap(&mut self, other: &AABB) -> Option<AABB> {
        self.reset();
        self.find(|rect| rect.is_overlap_rect(other))
    }

    const fn reset(&mut self) {
        self.completed = I32F32::ZERO;
    }
}

impl<'a> Iterator for SubstepIterator<'a> {
    type Item = AABB;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed >= self.steps {
            return None;
        }

        let mul = I32F32::from_num(self.completed);
        let x = self.position.x() + self.step.0 * mul;
        let y = self.position.y() + self.step.1 * mul;
        self.completed += I32F32::const_from_int(1);
        Some(AABB::new(x, y, self.size.x, self.size.y))
    }
}
