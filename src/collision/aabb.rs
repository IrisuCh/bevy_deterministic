use crate::determinism::transform::{GlobalPosition, Position, Size};
use fixed::types::I32F32;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    // В AABB достаточно двух точек.
    // Все остальные 6 вычисляются математически при необходимости.
    pub min: Position,
    pub max: Position,
}

impl Aabb {
    // Создание из позиции и размера
    pub fn from_pos_size(pos: Position, size: &Size) -> Self {
        Self {
            min: pos,
            max: Position {
                x: pos.x + size.x,
                y: pos.y + size.y,
                z: pos.z + size.z,
            },
        }
    }

    pub fn new(x: I32F32, y: I32F32, z: I32F32, w: I32F32, h: I32F32, d: I32F32) -> Self {
        Self {
            min: Position::new_fixed(x, y, z),
            max: Position::new_fixed(x + w, y + h, z + d),
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
    pub fn contains_point(&self, p: &Position) -> bool {
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
    pub fn corners(&self) -> [Position; 8] {
        [
            Position::new_fixed(self.min.x, self.min.y, self.min.z),
            Position::new_fixed(self.max.x, self.min.y, self.min.z),
            Position::new_fixed(self.min.x, self.max.y, self.min.z),
            Position::new_fixed(self.max.x, self.max.y, self.min.z),
            Position::new_fixed(self.min.x, self.min.y, self.max.z),
            Position::new_fixed(self.max.x, self.min.y, self.max.z),
            Position::new_fixed(self.min.x, self.max.y, self.max.z),
            Position::new_fixed(self.max.x, self.max.y, self.max.z),
        ]
    }
}
