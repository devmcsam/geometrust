//! Second dimension vectors and functions on vectors.

/// A second-dimension vector specialized for 64 bit floating point numbers.
struct Vec2F64 {
    /// The x component of the vector.
    x: f64,
    /// The y component of the vector.
    y: f64,
}

impl Vec2F64 {
    /// Create a new 2D vector.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
