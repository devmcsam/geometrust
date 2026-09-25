//! Second dimension vectors and functions on vectors.
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A second-dimension vector specialized for 64 bit floating point numbers.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Vec2F64 {
    /// The x component of the vector.
    x: f64,
    /// The y component of the vector.
    y: f64,
}

impl Vec2F64 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// Create a new 2D vector.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a new 2D vector with the same value for both components.
    #[inline]
    pub const fn splat(value: f64) -> Self {
        Self::new(value, value)
    }

    /// Create a new vector with both components being 0.0
    #[inline]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Create a new vector with both components being 1.0
    #[inline]
    pub const fn one() -> Self {
        Self::ONE
    }

    /// Add a value to the x component of the vector.
    #[inline]
    pub const fn add_x(&self, x: f64) -> Self {
        Self::new(self.x + x, self.y)
    }

    /// Add a value to the y component of the vector.
    #[inline]
    pub const fn add_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y + y)
    }

    /// Multiply the x component of the vector by a value.
    #[inline]
    pub const fn scale_x(&self, x: f64) -> Self {
        Self::new(self.x * x, self.y)
    }

    /// Multiply the y component of the vector by a value.
    #[inline]
    pub const fn scale_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y * y)
    }

    /// Subtract the x component of the vector by a value.
    #[inline]
    pub const fn sub_x(&self, x: f64) -> Self {
        Self::new(self.x - x, self.y)
    }

    /// Subtract the y component of the vector by a value.
    #[inline]
    pub const fn sub_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y - y)
    }

    /// Divide the x component of the vector by a value.
    #[inline]
    pub const fn div_x(&self, x: f64) -> Self {
        Self::new(self.x / x, self.y)
    }

    /// Divide the y component of the vector by a value.
    #[inline]
    pub const fn div_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y / y)
    }

    /// Dot product of two vectors.
    #[inline]
    pub const fn dot_product(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2F64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2F64 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2F64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2F64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for Vec2F64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vec2F64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vec2F64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Vec2F64 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
