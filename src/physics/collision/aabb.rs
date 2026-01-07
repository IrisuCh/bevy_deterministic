use fixed::types::I32F32;

use crate::{
    Fx,
    transform::{FQuat, FVec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: FVec3,
    pub max: FVec3,
}

impl Aabb {
    pub fn from_pos_size(pos: FVec3, size: FVec3) -> Self {
        Self {
            min: pos,
            max: FVec3 {
                x: pos.x + size.x,
                y: pos.y + size.y,
                z: pos.z + size.z,
            },
        }
    }

    pub fn new(x: I32F32, y: I32F32, z: I32F32, w: I32F32, h: I32F32, d: I32F32) -> Self {
        Self {
            min: FVec3::new_fixed(x, y, z),
            max: FVec3::new_fixed(x + w, y + h, z + d),
        }
    }

    // Хелперы для быстрого доступа
    pub const fn x(&self) -> I32F32 {
        self.min.x
    }
    pub const fn y(&self) -> I32F32 {
        self.min.y
    }
    pub const fn z(&self) -> I32F32 {
        self.min.z
    }

    pub fn w(&self) -> I32F32 {
        self.max.x - self.min.x
    }
    pub fn h(&self) -> I32F32 {
        self.max.y - self.min.y
    }
    pub fn d(&self) -> I32F32 {
        self.max.z - self.min.z
    }

    // Проверка попадания точки в бокс (3D)
    pub fn contains_point(&self, p: &FVec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    // Оптимизированная проверка пересечения двух AABB (3D)
    // Использует закон исключения: если по любой оси есть разрыв, значит столкновения нет.
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    // Если вам всё же нужны углы для отрисовки, генерируйте их лениво
    pub fn corners(&self) -> [FVec3; 8] {
        [
            FVec3::new_fixed(self.min.x, self.min.y, self.min.z),
            FVec3::new_fixed(self.max.x, self.min.y, self.min.z),
            FVec3::new_fixed(self.min.x, self.max.y, self.min.z),
            FVec3::new_fixed(self.max.x, self.max.y, self.min.z),
            FVec3::new_fixed(self.min.x, self.min.y, self.max.z),
            FVec3::new_fixed(self.max.x, self.min.y, self.max.z),
            FVec3::new_fixed(self.min.x, self.max.y, self.max.z),
            FVec3::new_fixed(self.max.x, self.max.y, self.max.z),
        ]
    }
}

pub struct OBB {
    pub min: FVec3,
    pub max: FVec3,
    pub rotation: FQuat,
}

impl OBB {
    pub fn intersects(&self, other: &Self) -> bool {
        let self_axes = self.axes();
        let other_axes = other.axes();
        let mut axes = [FVec3::ZERO; 15];
        axes[0] = self_axes[0];
        axes[1] = self_axes[1];
        axes[2] = self_axes[2];
        axes[3] = other_axes[0];
        axes[4] = other_axes[1];
        axes[5] = other_axes[2];
        axes[6] = self_axes[0].cross(other_axes[0]).normalize_or_zero();
        axes[7] = self_axes[0].cross(other_axes[1]).normalize_or_zero();
        axes[8] = self_axes[0].cross(other_axes[2]).normalize_or_zero();
        axes[9] = self_axes[1].cross(other_axes[0]).normalize_or_zero();
        axes[10] = self_axes[1].cross(other_axes[1]).normalize_or_zero();
        axes[11] = self_axes[1].cross(other_axes[2]).normalize_or_zero();
        axes[12] = self_axes[2].cross(other_axes[0]).normalize_or_zero();
        axes[13] = self_axes[2].cross(other_axes[1]).normalize_or_zero();
        axes[14] = self_axes[2].cross(other_axes[2]).normalize_or_zero();

        for axis in &axes {
            if axis.length_squared() == Fx::ZERO {
                continue;
            }
            let (min1, max1) = self.min_max(*axis);
            let (min2, max2) = other.min_max(*axis);

            if max1 < min2 || max2 < min1 {
                return false;
            }
        }

        true
    }

    pub fn axes(&self) -> [FVec3; 3] {
        [
            self.rotation.rotate_vec3(FVec3::new_f32(1.0, 0.0, 0.0)), // X
            self.rotation.rotate_vec3(FVec3::new_f32(0.0, 1.0, 0.0)), // Y
            self.rotation.rotate_vec3(FVec3::new_f32(0.0, 0.0, 1.0)), // Z
        ]
    }

    pub fn min_max(&self, axis: FVec3) -> (Fx, Fx) {
        let corners = self.corners();
        // НАЧИНАЙ С ПЕРВОЙ ВЕРШИНЫ, а не с ZERO!
        let mut min = corners[0].dot(axis);
        let mut max = min; // то же значение

        for corner in &corners[1..] {
            let dot = corner.dot(axis);
            min = min.min(dot);
            max = max.max(dot);
        }

        (min, max)
    }

    pub fn corners(&self) -> [FVec3; 8] {
        [
            FVec3::new_fixed(self.min.x, self.min.y, self.min.z),
            FVec3::new_fixed(self.max.x, self.min.y, self.min.z),
            FVec3::new_fixed(self.min.x, self.max.y, self.min.z),
            FVec3::new_fixed(self.max.x, self.max.y, self.min.z),
            FVec3::new_fixed(self.min.x, self.min.y, self.max.z),
            FVec3::new_fixed(self.max.x, self.min.y, self.max.z),
            FVec3::new_fixed(self.min.x, self.max.y, self.max.z),
            FVec3::new_fixed(self.max.x, self.max.y, self.max.z),
        ]
    }
}

mod tests {
    use bevy::log::tracing::Instrument;

    use crate::{
        physics::collision::aabb::OBB,
        transform::{FQuat, FVec3},
    };

    #[test]
    fn test() {
        let test_1 = OBB {
            min: FVec3::ZERO,
            max: FVec3::new_f32(1.0, 1.0, 1.0),
            rotation: FQuat::from_rotation_y(0.5),
        };

        let test_2 = OBB {
            min: FVec3::new_f32(0.0, 0.0, 0.0),
            max: FVec3::new_f32(3.0, 3.0, 3.0),
            rotation: FQuat::IDENTITY,
        };

        let intersects = test_1.intersects(&test_2);
        panic!("Intersection: {}", intersects);
    }
}
