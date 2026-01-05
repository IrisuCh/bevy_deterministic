use crate::determinism::transform::{GlobalPosition, Point, Position, Size};
use fixed::types::I32F32;

#[derive(Debug)]
pub struct AABB {
    /*

    2---3
    |   |
    0---1

    */
    pub c0: Position,
    pub c1: Position,
    pub c2: Position,
    pub c3: Position,
}

impl AABB {
    pub const fn x(&self) -> I32F32 {
        self.c0.x
    }

    pub const fn y(&self) -> I32F32 {
        self.c0.y
    }

    pub fn w(&self) -> I32F32 {
        self.c3.x - self.c0.x
    }

    pub fn h(&self) -> I32F32 {
        self.c3.y - self.c0.y
    }

    pub fn max_x(&self) -> I32F32 {
        self.x() + self.w()
    }

    pub fn max_y(&self) -> I32F32 {
        self.y() + self.h()
    }

    pub fn max(&self) -> Point {
        Point {
            x: self.max_x(),
            y: self.max_y(),
        }
    }

    pub const fn min(&self) -> Point {
        Point {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn from_pos_size(pos: &GlobalPosition, size: &Size) -> Self {
        Self {
            c0: Position::new(pos.x(), pos.y()),          // нижний левый
            c1: Position::new(pos.x() + size.x, pos.y()), // нижний правый
            c2: Position::new(pos.x(), pos.y() + size.y), // верхний левый
            c3: Position::new(pos.x() + size.x, pos.y() + size.y), // верхний правый
        }
    }

    pub fn new(x: I32F32, y: I32F32, w: I32F32, h: I32F32) -> Self {
        Self {
            c0: Position::new(x, y),         // нижний левый
            c1: Position::new(x + w, y),     // нижний правый
            c2: Position::new(x, y + h),     // верхний левый
            c3: Position::new(x + w, y + h), // верхний правый
        }
    }

    pub fn is_overlap(&self, point: &Position) -> bool {
        point.x >= self.c0.x && point.x <= self.c3.x && point.y >= self.c0.y && point.y <= self.c3.y
    }

    pub fn is_overlap_rect(&self, other: &AABB) -> bool {
        // Проверяем, попадает ли хотя бы одна вершина other внутрь self

        if self.is_overlap(&other.c0) {
            return true;
        }
        if self.is_overlap(&other.c1) {
            return true;
        }
        if self.is_overlap(&other.c2) {
            return true;
        }
        if self.is_overlap(&other.c3) {
            return true;
        }

        // Дополнительно: проверка обратного случая (self внутри other)
        if other.is_overlap(&self.c0) {
            return true;
        }
        if other.is_overlap(&self.c1) {
            return true;
        }
        if other.is_overlap(&self.c2) {
            return true;
        }
        if other.is_overlap(&self.c3) {
            return true;
        }

        // Иначе пересечения нет
        false
    }
}
