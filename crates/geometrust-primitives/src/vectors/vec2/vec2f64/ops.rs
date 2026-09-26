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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_x_adds_to_x_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.add_x(4.0), Vec2F64::new(6.0, 3.0));
    }

    #[test]
    fn add_x_handles_negative_values() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.add_x(-5.0), Vec2F64::new(-3.0, 3.0));
    }

    #[test]
    fn add_y_adds_to_y_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.add_y(4.0), Vec2F64::new(2.0, 7.0));
    }

    #[test]
    fn add_y_handles_negative_values() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.add_y(-5.0), Vec2F64::new(2.0, -2.0));
    }

    #[test]
    fn scale_x_multiplies_x_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.scale_x(4.0), Vec2F64::new(8.0, 3.0));
    }

    #[test]
    fn scale_x_handles_zero() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.scale_x(0.0), Vec2F64::new(0.0, 3.0));
    }

    #[test]
    fn scale_y_multiplies_y_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.scale_y(4.0), Vec2F64::new(2.0, 12.0));
    }

    #[test]
    fn scale_y_handles_zero() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.scale_y(0.0), Vec2F64::new(2.0, 0.0));
    }

    #[test]
    fn sub_x_subtracts_from_x_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.sub_x(4.0), Vec2F64::new(-2.0, 3.0));
    }

    #[test]
    fn sub_x_handles_negative_values() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.sub_x(-4.0), Vec2F64::new(6.0, 3.0));
    }

    #[test]
    fn sub_y_subtracts_from_y_component_only() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.sub_y(4.0), Vec2F64::new(2.0, -1.0));
    }

    #[test]
    fn sub_y_handles_negative_values() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector.sub_y(-4.0), Vec2F64::new(2.0, 7.0));
    }

    #[test]
    fn div_x_divides_x_component_only() {
        let vector = Vec2F64::new(8.0, 3.0);

        assert_eq!(vector.div_x(4.0), Vec2F64::new(2.0, 3.0));
    }

    #[test]
    fn div_x_by_zero_follows_f64_semantics() {
        let vector = Vec2F64::new(8.0, 3.0);
        let result = vector.div_x(0.0);

        assert_eq!(result.x, f64::INFINITY);
        assert_eq!(result.y, 3.0);
    }

    #[test]
    fn div_y_divides_y_component_only() {
        let vector = Vec2F64::new(2.0, 8.0);

        assert_eq!(vector.div_y(4.0), Vec2F64::new(2.0, 2.0));
    }

    #[test]
    fn div_y_by_zero_follows_f64_semantics() {
        let vector = Vec2F64::new(2.0, -8.0);
        let result = vector.div_y(0.0);

        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, f64::NEG_INFINITY);
    }

    #[test]
    fn add_components_adds_each_component() {
        let a = Vec2F64::new(2.0, 3.0);
        let b = Vec2F64::new(4.0, 5.0);

        assert_eq!(a.add_components(b), Vec2F64::new(6.0, 8.0));
    }

    #[test]
    fn add_components_handles_negative_components() {
        let a = Vec2F64::new(-2.0, 3.0);
        let b = Vec2F64::new(4.0, -5.0);

        assert_eq!(a.add_components(b), Vec2F64::new(2.0, -2.0));
    }

    #[test]
    fn sub_components_subtracts_each_component() {
        let a = Vec2F64::new(2.0, 3.0);
        let b = Vec2F64::new(4.0, 5.0);

        assert_eq!(a.sub_components(b), Vec2F64::new(-2.0, -2.0));
    }

    #[test]
    fn sub_components_handles_negative_components() {
        let a = Vec2F64::new(-2.0, 3.0);
        let b = Vec2F64::new(4.0, -5.0);

        assert_eq!(a.sub_components(b), Vec2F64::new(-6.0, 8.0));
    }

    #[test]
    fn mul_components_multiplies_each_component() {
        let a = Vec2F64::new(2.0, 3.0);
        let b = Vec2F64::new(4.0, 5.0);

        assert_eq!(a.mul_components(b), Vec2F64::new(8.0, 15.0));
    }

    #[test]
    fn mul_components_handles_zero_and_negative_components() {
        let a = Vec2F64::new(-2.0, 0.0);
        let b = Vec2F64::new(4.0, -5.0);

        assert_eq!(a.mul_components(b), Vec2F64::new(-8.0, -0.0));
    }

    #[test]
    fn div_components_divides_each_component() {
        let a = Vec2F64::new(8.0, 15.0);
        let b = Vec2F64::new(4.0, 5.0);

        assert_eq!(a.div_components(b), Vec2F64::new(2.0, 3.0));
    }

    #[test]
    fn div_components_by_zero_follows_f64_semantics() {
        let a = Vec2F64::new(8.0, -15.0);
        let b = Vec2F64::new(0.0, 0.0);
        let result = a.div_components(b);

        assert_eq!(result.x, f64::INFINITY);
        assert_eq!(result.y, f64::NEG_INFINITY);
    }

    #[test]
    fn add_operator_adds_vectors() {
        let a = Vec2F64::new(1.0, 2.0);
        let b = Vec2F64::new(3.0, 4.0);

        assert_eq!(a + b, Vec2F64::new(4.0, 6.0));
    }

    #[test]
    fn add_operator_handles_negative_components() {
        let a = Vec2F64::new(-1.0, 2.0);
        let b = Vec2F64::new(3.0, -4.0);

        assert_eq!(a + b, Vec2F64::new(2.0, -2.0));
    }

    #[test]
    fn add_assign_operator_adds_vectors_in_place() {
        let mut vector = Vec2F64::new(1.0, 2.0);

        vector += Vec2F64::new(3.0, 4.0);

        assert_eq!(vector, Vec2F64::new(4.0, 6.0));
    }

    #[test]
    fn add_assign_operator_handles_zero() {
        let mut vector = Vec2F64::new(1.0, 2.0);

        vector += Vec2F64::zero();

        assert_eq!(vector, Vec2F64::new(1.0, 2.0));
    }

    #[test]
    fn sub_operator_subtracts_vectors() {
        let a = Vec2F64::new(1.0, 2.0);
        let b = Vec2F64::new(3.0, 4.0);

        assert_eq!(a - b, Vec2F64::new(-2.0, -2.0));
    }

    #[test]
    fn sub_operator_handles_negative_components() {
        let a = Vec2F64::new(-1.0, 2.0);
        let b = Vec2F64::new(3.0, -4.0);

        assert_eq!(a - b, Vec2F64::new(-4.0, 6.0));
    }

    #[test]
    fn sub_assign_operator_subtracts_vectors_in_place() {
        let mut vector = Vec2F64::new(1.0, 2.0);

        vector -= Vec2F64::new(3.0, 4.0);

        assert_eq!(vector, Vec2F64::new(-2.0, -2.0));
    }

    #[test]
    fn sub_assign_operator_handles_zero() {
        let mut vector = Vec2F64::new(1.0, 2.0);

        vector -= Vec2F64::zero();

        assert_eq!(vector, Vec2F64::new(1.0, 2.0));
    }

    #[test]
    fn mul_operator_multiplies_vector_by_scalar() {
        let vector = Vec2F64::new(2.0, 3.0);

        assert_eq!(vector * 4.0, Vec2F64::new(8.0, 12.0));
    }

    #[test]
    fn mul_operator_handles_zero_scalar() {
        let vector = Vec2F64::new(2.0, -3.0);

        assert_eq!(vector * 0.0, Vec2F64::new(0.0, -0.0));
    }

    #[test]
    fn mul_assign_operator_multiplies_vector_by_scalar_in_place() {
        let mut vector = Vec2F64::new(2.0, 3.0);

        vector *= 4.0;

        assert_eq!(vector, Vec2F64::new(8.0, 12.0));
    }

    #[test]
    fn mul_assign_operator_handles_negative_scalar() {
        let mut vector = Vec2F64::new(2.0, -3.0);

        vector *= -4.0;

        assert_eq!(vector, Vec2F64::new(-8.0, 12.0));
    }

    #[test]
    fn div_operator_divides_vector_by_scalar() {
        let vector = Vec2F64::new(8.0, 12.0);

        assert_eq!(vector / 4.0, Vec2F64::new(2.0, 3.0));
    }

    #[test]
    fn div_operator_by_zero_follows_f64_semantics() {
        let vector = Vec2F64::new(8.0, -12.0);
        let result = vector / 0.0;

        assert_eq!(result.x, f64::INFINITY);
        assert_eq!(result.y, f64::NEG_INFINITY);
    }

    #[test]
    fn div_assign_operator_divides_vector_by_scalar_in_place() {
        let mut vector = Vec2F64::new(8.0, 12.0);

        vector /= 4.0;

        assert_eq!(vector, Vec2F64::new(2.0, 3.0));
    }

    #[test]
    fn div_assign_operator_by_zero_follows_f64_semantics() {
        let mut vector = Vec2F64::new(8.0, -12.0);

        vector /= 0.0;

        assert_eq!(vector.x, f64::INFINITY);
        assert_eq!(vector.y, f64::NEG_INFINITY);
    }
}
