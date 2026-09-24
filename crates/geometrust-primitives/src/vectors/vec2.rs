//! Second dimension vectors and functions on vectors.

/// A second-dimension vector specialized for 64 bit floating point numbers.
#[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Add a value to the x component of the vector.
    #[inline]
    pub const fn add_x(self, x: f64) -> Self {
        Self::new(self.x + x, self.y)
    }

    /// Add a value to the y component of the vector.
    #[inline]
    pub const fn add_y(self, y: f64) -> Self {
        Self::new(self.x, self.y + y)
    }

    /// Multiply the x component of the vector by a value.
    #[inline]
    pub const fn scale_x(self, x: f64) -> Self {
        Self::new(self.x * x, self.y)
    }

    /// Multiply the y component of the vector by a value.
    #[inline]
    pub const fn scale_y(self, y: f64) -> Self {
        Self::new(self.x, self.y * y)
    }

    /// Subtract the x component of the vector by a value.
    #[inline]
    pub const fn sub_x(self, x: f64) -> Self {
        Self::new(self.x - x, self.y)
    }

    /// Subtract the y component of the vector by a value.
    #[inline]
    pub const fn sub_y(self, y: f64) -> Self {
        Self::new(self.x, self.y - y)
    }

    /// Divide the x component of the vector by a value.
    #[inline]
    pub const fn div_x(self, x: f64) -> Self {
        Self::new(self.x / x, self.y)
    }

    /// Divide the y component of the vector by a value.
    #[inline]
    pub const fn div_y(self, y: f64) -> Self {
        Self::new(self.x, self.y / y)
    }
}
