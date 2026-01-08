use crate::{
    Fx,
    physics::collision::aabb::{CollisionInfo, Obb},
    transform::{FVec3, FixedTransform},
};

const STEPS: Fx = Fx::const_from_int(16);

pub struct SubstepIterator {
    transform: FixedTransform,
    step: FVec3,
    steps: Fx,
    completed: Fx,
}

impl SubstepIterator {
    pub fn new(transform: FixedTransform, velocity_x: Fx, velocity_y: Fx, velocity_z: Fx) -> Self {
        let step = FVec3 {
            x: velocity_x / STEPS,
            y: velocity_y / STEPS,
            z: velocity_z / STEPS,
        };
        Self {
            transform,
            step,
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
