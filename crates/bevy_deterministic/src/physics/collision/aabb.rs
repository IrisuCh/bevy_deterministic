use fx::IntoFx;

use crate::{
    math::{FQuat, FVec3, Fx},
    physics::collision::obb::Obb,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn new(
        x: impl IntoFx,
        y: impl IntoFx,
        z: impl IntoFx,
        w: impl IntoFx,
        h: impl IntoFx,
        d: impl IntoFx,
    ) -> Self {
        let x = x.into_fx();
        let y = y.into_fx();
        let z = z.into_fx();
        let w = w.into_fx();
        let h = h.into_fx();
        let d = d.into_fx();

        Self {
            min: FVec3::new(x, y, z),
            max: FVec3::new(x + w, y + h, z + d),
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
            FVec3::new(self.min.x, self.min.y, self.min.z),
            FVec3::new(self.max.x, self.min.y, self.min.z),
            FVec3::new(self.min.x, self.max.y, self.min.z),
            FVec3::new(self.max.x, self.max.y, self.min.z),
            FVec3::new(self.min.x, self.min.y, self.max.z),
            FVec3::new(self.max.x, self.min.y, self.max.z),
            FVec3::new(self.min.x, self.max.y, self.max.z),
            FVec3::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    #[must_use]
    pub fn as_obb(&self, rotation: FQuat) -> Obb {
        let size = self.max - self.min;
        Obb::from_transform(self.min, size, rotation)
    }
}
