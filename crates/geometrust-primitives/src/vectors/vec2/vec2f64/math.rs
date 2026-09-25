//! Math related functions on 2D vectors.
use super::definition::Vec2F64;

impl Vec2F64 {
    /// Dot product of two vectors.
    #[inline]
    pub const fn dot_product(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Find the length or magnitude of the given vector.
    #[inline]
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Find the squared length or magnitude of the given vector.
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}
