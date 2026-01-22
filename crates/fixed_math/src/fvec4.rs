use crate::{Fx, fx};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FVec4 {
    pub x: Fx,
    pub y: Fx,
    pub z: Fx,
    pub w: Fx,
}

impl FVec4 {
    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> Fx {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> Fx {
        self.dot(self)
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        Fx::abs(self.length_squared() - Fx::ONE) <= fx!(2e-4)
    }
}
