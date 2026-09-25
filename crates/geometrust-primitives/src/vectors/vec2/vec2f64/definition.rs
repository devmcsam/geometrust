//! Second dimension vector and functions that have to do with creation.

/// A second-dimension vector specialized for 64 bit floating point numbers.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2F64 {
    /// The x component of the vector.
    pub(crate) x: f64,
    /// The y component of the vector.
    pub(crate) y: f64,
}

impl Vec2F64 {
    /// A `Vec2F64` with both components being 0.0
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    /// A `Vec2F64` with both components being 1.0
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// Create vec2 new 2D vector.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create vec2 new 2D vector with the same value for both components.
    #[inline]
    pub const fn splat(value: f64) -> Self {
        Self::new(value, value)
    }

    /// Create vec2 new vector with both components being 0.0
    #[inline]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Create vec2 new vector with both components being 1.0
    #[inline]
    pub const fn one() -> Self {
        Self::ONE
    }

    #[inline]
    pub const fn from_tuple(tup: (f64, f64)) -> Self {
        Self::new(tup.0, tup.1)
    }

    #[inline]
    pub const fn from_array(arr: [f64; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    /// Create vec2 vector from an angle in radians.
    #[inline]
    pub fn from_angle(theta: f64) -> Self {
        Self::new(theta.cos(), theta.sin())
    }

    /// Create vec2 vector from polar coordinates.
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }
}
