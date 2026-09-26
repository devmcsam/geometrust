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

    /// Find the distance between two vectors.
    #[inline]
    pub fn distance(&self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }

    /// Find the distance squared between two vectors.
    #[inline]
    pub fn distance_squared(&self, other: Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    /// Find the absolute value of each component of the vector and return a new vector with each component.
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn assert_approx_eq(left: f64, right: f64) {
        assert!(
            (left - right).abs() <= EPSILON,
            "expected {left} to approximately equal {right}"
        );
    }

    #[test]
    fn dot_product_returns_component_wise_products_sum() {
        let a = Vec2F64::new(2.0, 3.0);
        let b = Vec2F64::new(4.0, 5.0);

        assert_eq!(a.dot_product(b), 23.0);
    }

    #[test]
    fn dot_product_handles_negative_components() {
        let a = Vec2F64::new(-2.0, 3.0);
        let b = Vec2F64::new(4.0, -5.0);

        assert_eq!(a.dot_product(b), -23.0);
    }

    #[test]
    fn dot_product_with_zero_vector_is_zero() {
        let vector = Vec2F64::new(3.5, -8.25);

        assert_eq!(vector.dot_product(Vec2F64::zero()), 0.0);
        assert_eq!(Vec2F64::zero().dot_product(vector), 0.0);
    }

    #[test]
    fn length_returns_magnitude() {
        let vector = Vec2F64::new(3.0, 4.0);

        assert_eq!(vector.length(), 5.0);
    }

    #[test]
    fn length_of_zero_vector_is_zero() {
        assert_eq!(Vec2F64::zero().length(), 0.0);
    }

    #[test]
    fn length_handles_negative_components() {
        let vector = Vec2F64::new(-3.0, -4.0);

        assert_eq!(vector.length(), 5.0);
    }

    #[test]
    fn length_squared_returns_squared_magnitude() {
        let vector = Vec2F64::new(3.0, 4.0);

        assert_eq!(vector.length_squared(), 25.0);
    }

    #[test]
    fn length_squared_of_zero_vector_is_zero() {
        assert_eq!(Vec2F64::zero().length_squared(), 0.0);
    }

    #[test]
    fn length_squared_handles_negative_components() {
        let vector = Vec2F64::new(-3.0, -4.0);

        assert_eq!(vector.length_squared(), 25.0);
    }

    #[test]
    fn distance_returns_distance_between_vectors() {
        let a = Vec2F64::new(1.0, 2.0);
        let b = Vec2F64::new(4.0, 6.0);

        assert_eq!(a.distance(b), 5.0);
    }

    #[test]
    fn distance_is_symmetric() {
        let a = Vec2F64::new(-1.5, 2.25);
        let b = Vec2F64::new(4.75, -6.5);

        assert_approx_eq(a.distance(b), b.distance(a));
    }

    #[test]
    fn distance_to_self_is_zero() {
        let vector = Vec2F64::new(-7.0, 11.0);

        assert_eq!(vector.distance(vector), 0.0);
    }

    #[test]
    fn distance_squared_returns_squared_distance_between_vectors() {
        let a = Vec2F64::new(1.0, 2.0);
        let b = Vec2F64::new(4.0, 6.0);

        assert_eq!(a.distance_squared(b), 25.0);
    }

    #[test]
    fn distance_squared_is_symmetric() {
        let a = Vec2F64::new(-1.5, 2.25);
        let b = Vec2F64::new(4.75, -6.5);

        assert_approx_eq(a.distance_squared(b), b.distance_squared(a));
    }

    #[test]
    fn distance_squared_to_self_is_zero() {
        let vector = Vec2F64::new(-7.0, 11.0);

        assert_eq!(vector.distance_squared(vector), 0.0);
    }

    #[test]
    fn abs_returns_absolute_value_of_each_component() {
        let vector = Vec2F64::new(-3.5, 4.25);

        assert_eq!(vector.abs(), Vec2F64::new(3.5, 4.25));
    }

    #[test]
    fn abs_handles_zero_and_negative_zero() {
        let vector = Vec2F64::new(-0.0, 0.0);
        let result = vector.abs();

        assert_eq!(result, Vec2F64::new(0.0, 0.0));
        assert!(!result.x.is_sign_negative());
        assert!(!result.y.is_sign_negative());
    }

    #[test]
    fn abs_preserves_infinity() {
        let vector = Vec2F64::new(f64::NEG_INFINITY, f64::INFINITY);

        assert_eq!(vector.abs(), Vec2F64::new(f64::INFINITY, f64::INFINITY));
    }
}
