use fixed::types::I32F32;

use crate::{
    physics::collision::Aabb,
    transform::{FVec3, FixedTransform},
};

const STEPS: I32F32 = I32F32::const_from_int(16);

pub struct SubstepIterator {
    transform: FixedTransform,
    step: FVec3,
    steps: I32F32,
    completed: I32F32,
}

impl SubstepIterator {
    pub fn new(
        transform: FixedTransform,
        velocity_x: I32F32,
        velocity_y: I32F32,
        velocity_z: I32F32,
    ) -> Self {
        let x_step = velocity_x / STEPS;
        let y_step = velocity_y / STEPS;
        let z_step = velocity_z / STEPS;
        let step = FVec3 {
            x: x_step,
            y: y_step,
            z: z_step,
        };
        Self {
            transform,
            step,
            steps: STEPS,
            completed: I32F32::ZERO,
        }
    }

    pub fn next_overlap(&mut self, other: &Aabb) -> Option<Aabb> {
        self.reset();
        self.find(|rect| rect.intersects(other))
    }

    const fn reset(&mut self) {
        self.completed = I32F32::ZERO;
    }
}

impl Iterator for SubstepIterator {
    type Item = Aabb;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed >= self.steps {
            return None;
        }

        let mul = I32F32::from_num(self.completed);
        let x = self.transform.position.x + self.step.x * mul;
        let y = self.transform.position.y + self.step.y * mul;
        let z = self.transform.position.z + self.step.z * mul;
        self.completed += I32F32::const_from_int(1);
        Some(Aabb::new(
            x,
            y,
            z,
            self.transform.size.x,
            self.transform.size.y,
            self.transform.size.z,
        ))
    }
}
