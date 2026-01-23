use whitelace_core::math::{FVec3, Fx};
use whitelace_transform::FixedTransform;

use crate::collision::obb::{CollisionInfo, Obb};

const STEPS: Fx = Fx::const_from_int(16);

pub struct SubstepIterator {
    transform: FixedTransform,
    step: FVec3,
    steps: Fx,
    completed: Fx,
}

impl SubstepIterator {
    pub fn with_no_velocity(transform: FixedTransform) -> Self {
        Self {
            transform,
            step: FVec3::ZERO,
            steps: Fx::ONE,
            completed: Fx::ZERO,
        }
    }

    pub fn new(transform: FixedTransform, velocity: FVec3) -> Self {
        Self {
            transform,
            step: velocity / STEPS,
            steps: STEPS,
            completed: Fx::ZERO,
        }
    }

    pub fn next_overlap(&mut self, other: &Obb) -> Option<CollisionInfo> {
        self.reset();
        self.find_map(|rect| rect.intersects(other))
    }

    const fn reset(&mut self) {
        self.completed = Fx::ZERO;
    }
}

impl Iterator for SubstepIterator {
    type Item = Obb;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed >= self.steps {
            return None;
        }

        let mul = Fx::from_num(self.completed);
        let x = self.transform.position.x + self.step.x * mul;
        let y = self.transform.position.y + self.step.y * mul;
        let z = self.transform.position.z + self.step.z * mul;
        self.completed += Fx::const_from_int(1);
        Some(Obb::from_transform(
            FVec3::new(x, y, z),
            self.transform.size,
            self.transform.rotation,
        ))
    }
}
