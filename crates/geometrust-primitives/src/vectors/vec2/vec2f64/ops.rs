//! Operations on 2D vectors.
use super::definition::Vec2F64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Vec2F64 {
    /// Add vec2 value to the x component of the vector.
    #[inline]
    pub const fn add_x(&self, x: f64) -> Self {
        Self::new(self.x + x, self.y)
    }

    /// Add vec2 value to the y component of the vector.
    #[inline]
    pub const fn add_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y + y)
    }

    /// Multiply the x component of the vector by vec2 value.
    #[inline]
    pub const fn scale_x(&self, x: f64) -> Self {
        Self::new(self.x * x, self.y)
    }

    /// Multiply the y component of the vector by vec2 value.
    #[inline]
    pub const fn scale_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y * y)
    }

    /// Subtract the x component of the vector by vec2 value.
    #[inline]
    pub const fn sub_x(&self, x: f64) -> Self {
        Self::new(self.x - x, self.y)
    }

    /// Subtract the y component of the vector by vec2 value.
    #[inline]
    pub const fn sub_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y - y)
    }

    /// Divide the x component of the vector by vec2 value.
    #[inline]
    pub const fn div_x(&self, x: f64) -> Self {
        Self::new(self.x / x, self.y)
    }

    /// Divide the y component of the vector by vec2 value.
    #[inline]
    pub const fn div_y(&self, y: f64) -> Self {
        Self::new(self.x, self.y / y)
    }

    /// Perform vec2 component-wise addition of two vectors.
    #[inline]
    pub const fn add_components(&self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Perform vec2 component-wise subtraction of two vectors.
    #[inline]
    pub const fn sub_components(&self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Perform vec2 component-wise multiplication of two vectors.
    #[inline]
    pub const fn mul_components(&self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Perform vec2 component-wise division of two vectors.
    #[inline]
    pub const fn div_components(&self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
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
