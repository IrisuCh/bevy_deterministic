use bevy::prelude::*;
use strum_macros::{Display, EnumCount};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, EnumCount, Display)]
pub enum CollisionSide {
    Left = 0b00_0001,   // -X
    Right = 0b00_0010,  // +X
    Bottom = 0b00_0100, // -Y
    Top = 0b00_1000,    // +Y
    Front = 0b01_0000,  // +Z
    Back = 0b10_0000,   // -Z
}

impl CollisionSide {
    #[must_use]
    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub const fn index(&self) -> usize {
        (*self as u32).trailing_zeros() as usize
    }
}
