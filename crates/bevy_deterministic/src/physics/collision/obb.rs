use crate::math::{FQuat, FVec3, Fx, fx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Obb {
    pub center: FVec3,       // центр OBB
    pub half_extents: FVec3, // половины размеров (width/2, height/2, depth/2)
    pub rotation: FQuat,
}

impl Obb {
    pub fn from_transform(position: FVec3, size: FVec3, rotation: FQuat) -> Self {
        let half_extents = size * fx!(0.5);
        Self {
            center: position + half_extents, // ← конвертируем в центр
            half_extents,
            rotation,
        }
    }

    pub fn intersects(&self, other: &Self) -> Option<CollisionInfo> {
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

        let mut min_overlap = Fx::MAX;
        let mut collision_normal = FVec3::ZERO;

        for axis in &axes {
            if axis.length_squared() == Fx::ZERO {
                continue;
            }
            let (min1, max1) = self.min_max(*axis);
            let (min2, max2) = other.min_max(*axis);

            if max1 < min2 || max2 < min1 {
                return None;
            }

            let overlap = max1.min(max2) - min1.max(min2);

            if overlap < min_overlap {
                min_overlap = overlap;
                collision_normal = *axis;
            }
        }

        let direction = other.center - self.center;

        if collision_normal.dot(direction) < Fx::ZERO {
            collision_normal = -collision_normal;
        }

        Some(CollisionInfo {
            normal: collision_normal,
            depth: min_overlap,
        })
    }

    pub fn axes(&self) -> [FVec3; 3] {
        [
            self.rotation.rotate_vec3(FVec3::new(1, 0, 0)), // X
            self.rotation.rotate_vec3(FVec3::new(0, 1, 0)), // Y
            self.rotation.rotate_vec3(FVec3::new(0, 0, 1)), // Z
        ]
    }

    pub fn min_max(&self, axis: FVec3) -> (Fx, Fx) {
        let corners = self.vertices();
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

    pub fn vertices(&self) -> [FVec3; 8] {
        // Локальные оси после вращения
        let axis_x = self.rotation.rotate_vec3(FVec3::new(1, 0, 0));
        let axis_y = self.rotation.rotate_vec3(FVec3::new(0, 1, 0));
        let axis_z = self.rotation.rotate_vec3(FVec3::new(0, 0, 1));

        // Смещения по осям
        let dx = axis_x * self.half_extents.x;
        let dy = axis_y * self.half_extents.y;
        let dz = axis_z * self.half_extents.z;

        // Все комбинации ±dx ±dy ±dz
        [
            self.center - dx - dy - dz,
            self.center + dx - dy - dz,
            self.center - dx + dy - dz,
            self.center + dx + dy - dz,
            self.center - dx - dy + dz,
            self.center + dx - dy + dz,
            self.center - dx + dy + dz,
            self.center + dx + dy + dz,
        ]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CollisionInfo {
    pub normal: FVec3, // Направление выталкивания
    pub depth: Fx,     // На сколько нужно сдвинуть
}
