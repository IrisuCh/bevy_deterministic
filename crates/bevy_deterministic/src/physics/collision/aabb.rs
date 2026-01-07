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
    #[must_use]
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

    #[must_use]
    pub fn new(x: Fx, y: Fx, z: Fx, w: Fx, h: Fx, d: Fx) -> Self {
        Self {
            min: FVec3::new_fixed(x, y, z),
            max: FVec3::new_fixed(x + w, y + h, z + d),
        }
    }

    #[must_use]
    pub const fn x(&self) -> Fx {
        self.min.x
    }

    #[must_use]
    pub const fn y(&self) -> Fx {
        self.min.y
    }

    #[must_use]
    pub const fn z(&self) -> Fx {
        self.min.z
    }

    #[must_use]
    pub fn w(&self) -> Fx {
        self.max.x - self.min.x
    }

    #[must_use]
    pub fn h(&self) -> Fx {
        self.max.y - self.min.y
    }

    #[must_use]
    pub fn d(&self) -> Fx {
        self.max.z - self.min.z
    }

    #[must_use]
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
    #[must_use]
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    #[must_use]
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

#[derive(Debug, Clone)]
pub struct Obb {
    pub center: FVec3,       // центр OBB
    pub half_extents: FVec3, // половины размеров (width/2, height/2, depth/2)
    pub rotation: FQuat,
}

impl Obb {
    pub fn from_transform(position: FVec3, size: FVec3, rotation: FQuat) -> Self {
        let half_extents = size * Fx::from_num(0.5);
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
            self.rotation.rotate_vec3(FVec3::new_f32(1.0, 0.0, 0.0)), // X
            self.rotation.rotate_vec3(FVec3::new_f32(0.0, 1.0, 0.0)), // Y
            self.rotation.rotate_vec3(FVec3::new_f32(0.0, 0.0, 1.0)), // Z
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
        let axis_x = self.rotation.rotate_vec3(FVec3::new_f32(1.0, 0.0, 0.0));
        let axis_y = self.rotation.rotate_vec3(FVec3::new_f32(0.0, 1.0, 0.0));
        let axis_z = self.rotation.rotate_vec3(FVec3::new_f32(0.0, 0.0, 1.0));

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

    //pub fn w(&self) -> I32F32 {
    //    self.max.x - self.min.x
    //}
    //pub fn h(&self) -> I32F32 {
    //    self.max.y - self.min.y
    //}
    //pub fn d(&self) -> I32F32 {
    //    self.max.z - self.min.z
    //}
}

#[derive(Debug)]
pub struct CollisionInfo {
    pub normal: FVec3, // Направление выталкивания
    pub depth: Fx,     // На сколько нужно сдвинуть
}
