use derive_more::Into;

use crate::FVec3;
#[cfg(debug_assertions)]
use crate::Fx;

/// Checks that a vector with the given squared length is normalized.
///
/// Warns for small error with a length threshold of approximately `1e-4`,
/// and panics for large error with a length threshold of approximately `1e-2`.
///
/// The format used for the logged warning is `"Warning: {warning} The length is {length}`,
/// and similarly for the error.
#[cfg(debug_assertions)]
fn assert_is_normalized(message: &str, length_squared: Fx) {
    use crate::const_fx;

    let length_error_squared = Fx::abs(length_squared - const_fx!(1));

    // Panic for large error and warn for slight error.
    if length_error_squared > 2e-2 {
        // Length error is approximately 1e-2 or more.
        panic!(
            "Error: {message} The length is {}.",
            Fx::sqrt(length_squared)
        );
    } else if length_error_squared > 2e-4 {
        // Length error is approximately 1e-4 or more.
        #[cfg(feature = "std")]
        #[expect(clippy::print_stderr, reason = "Allowed behind `std` feature gate.")]
        {
            eprintln!(
                "Warning: {message} The length is {}.",
                ops::sqrt(length_squared)
            );
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Into)]
pub struct FDir3(FVec3);

impl FDir3 {
    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self(FVec3::X);
    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self(FVec3::Y);
    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self(FVec3::Z);
    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self(FVec3::NEG_X);
    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self(FVec3::NEG_Y);
    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self(FVec3::NEG_Z);
    /// The directional axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Create a [`Dir3`] from a [`Vec3`] that is already normalized.
    ///
    /// # Warning
    ///
    /// `value` must be normalized, i.e its length must be `1.0`.
    #[must_use]
    pub fn new_unchecked(value: FVec3) -> Self {
        #[cfg(debug_assertions)]
        assert_is_normalized(
            "The vector given to `Dir3::new_unchecked` is not normalized.",
            value.length_squared(),
        );

        Self(value)
    }
}

impl core::ops::Neg for FDir3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl core::ops::Deref for FDir3 {
    type Target = FVec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
