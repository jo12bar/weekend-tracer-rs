//! Structs and methods related to operating on 3D vectors.

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3D vector. Could be utilized for points, colours, actual vectors, etc...
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Create a new 3D vector.
    ///
    /// For convenience, the `vec3!` macro is also provided. Use it like this:
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    /// use weekend_tracer_rs::vec3;
    ///
    /// assert_eq!(vec3!(), Vec3::new(0.0, 0.0, 0.0));
    /// assert_eq!(vec3!(1.0), Vec3::new(1.0, 0.0, 0.0));
    /// assert_eq!(vec3!(1.0, -3.0), Vec3::new(1.0, -3.0, 0.0));
    /// assert_eq!(vec3!(1.0, -3.0, 4.3), Vec3::new(1.0, -3.0, 4.3));
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Returns the length of the vector, squared.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -1.0, 1.0);
    /// assert_eq!(a.length_squared(), 3.0);
    /// ```
    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Returns the length of the vector.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(5.0, 10.0, -10.0);
    /// assert_eq!(a.length(), 15.0);
    /// ```
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Computes the [dot product](https://en.wikipedia.org/wiki/Dot_product) of
    /// two vectors.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -2.0, 3.0);
    /// let b = Vec3::new(-5.0, 9.0, 0.1);
    ///
    /// assert_eq!(a.dot(&b), -22.7);
    /// assert_eq!(b.dot(&a), -22.7);
    /// ```
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Computes the [cross product](https://en.wikipedia.org/wiki/Cross_product)
    /// of two vectors.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -2.0, 3.0);
    /// let b = Vec3::new(-5.0, 9.0, 0.1);
    ///
    /// assert_eq!(a.cross(&b), Vec3::new(-27.2, -15.1, -1.0));
    /// assert_eq!(b.cross(&a), Vec3::new(27.2, 15.1, 1.0));
    /// ```
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: -((self.x * rhs.z) - (self.z * rhs.x)),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// Returns the "unit vector version" of the original vector, which:
    ///
    /// - Is in the same direction as the original vector, and
    /// - Has length 1.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(5.0, 10.0, -10.0);
    /// let ua = a.unit_vector();
    ///
    /// // Check lengths:
    /// assert_eq!(a.length(), 15.0);
    /// assert_eq!(ua.length(), 1.0);
    ///
    /// // Check directionality by checking that multiplying the unit vector by
    /// // the length (magnitude) of the original vector gives back the original
    /// // vector:
    /// assert_eq!(ua * a.length(), a);
    /// assert_eq!(ua * (-a.length()), -a);
    /// assert_eq!(ua * (-42.0 * a.length()), -42.0 * a);
    /// ```
    pub fn unit_vector(&self) -> Self {
        let inverse_length = 1.0 / self.length();
        Vec3 {
            x: self.x * inverse_length,
            y: self.y * inverse_length,
            z: self.z * inverse_length,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

/// A convenience macro for more easily building `Vec3`'s. Use it like this:
///
/// ```
/// use weekend_tracer_rs::vec3::Vec3;
/// use weekend_tracer_rs::vec3;
///
/// assert_eq!(vec3!(), Vec3::new(0.0, 0.0, 0.0));
/// assert_eq!(vec3!(1.0), Vec3::new(1.0, 0.0, 0.0));
/// assert_eq!(vec3!(1.0, -3.0), Vec3::new(1.0, -3.0, 0.0));
/// assert_eq!(vec3!(1.0, -3.0, 4.3), Vec3::new(1.0, -3.0, 4.3));
/// ```
#[macro_export]
macro_rules! vec3 {
    () => {
        Vec3::new(0.0, 0.0, 0.0)
    };
    ($x:expr $(,)?) => {
        Vec3::new($x, 0.0, 0.0)
    };
    ($x:expr, $y:expr $(,)?) => {
        Vec3::new($x, $y, 0.0)
    };
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        Vec3::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_invocation() {
        assert_eq!(vec3!(4.0, 2.0, 1.0), Vec3::new(4.0, 2.0, 1.0));
        assert_eq!(
            vec3!(3.5, 64.2, -13.0),
            Vec3 {
                x: 3.5,
                y: 64.2,
                z: -13.0
            }
        );
        assert_eq!(vec3!(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(vec3!(-1.1), Vec3::new(-1.1, 0.0, 0.0));
        assert_eq!(vec3!(20.3, -5.6), Vec3::new(20.3, -5.6, 0.0));
    }

    #[test]
    fn add() {
        assert_eq!(
            vec3!(1.0, 2.0, 0.0) + vec3!(-1.0, 2.1, -4.2),
            vec3!(0.0, 4.1, -4.2)
        );
    }

    #[test]
    fn add_assign() {
        let mut a = vec3!();
        let b = vec3!(1.0, -8.8, 3.8);
        a += b;
        assert_eq!(
            a,
            Vec3 {
                x: 1.0,
                y: -8.8,
                z: 3.8
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            vec3!(1.0, -1.9, 3.45) - vec3!(0.0, 8.5, -5.4),
            vec3!(1.0, -10.4, 8.85)
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = vec3!();
        let b = vec3!(1.0, -8.8, 3.8);
        a -= b;
        assert_eq!(
            a,
            Vec3 {
                x: -1.0,
                y: 8.8,
                z: -3.8
            }
        );
    }

    #[test]
    fn mul() {
        assert_eq!(3.0 * vec3!(1.0, 2.0, -3.0), vec3!(3.0, 6.0, -9.0));
        assert_eq!(vec3!(1.0, 2.0, -3.0) * 3.0, vec3!(3.0, 6.0, -9.0));
        assert_eq!(
            vec3!(1.0, 2.0, 3.0) * vec3!(-1.0, 3.1),
            vec3!(-1.0, 6.2, 0.0),
        )
    }

    #[test]
    fn mul_assign() {
        let mut a = vec3!(1.0, 2.0, 3.0);
        let b = vec3!(-1.0, 0.0, 2.0);
        a *= 5.0;
        assert_eq!(a, vec3!(5.0, 10.0, 15.0));
        a *= b;
        assert_eq!(a, vec3!(-5.0, 0.0, 30.0));
    }

    #[test]
    fn div() {
        assert_eq!(vec3!(3.0, 6.0, -9.0) / 3.0, vec3!(1.0, 2.0, -3.0));
    }

    #[test]
    fn div_assign() {
        let mut a = vec3!(5.0, -15.0, 30.0);
        a /= 5.0;
        assert_eq!(a, vec3!(1.0, -3.0, 6.0));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn neg() {
        assert_eq!(-vec3!(6.0, -5.5, 3.14159), vec3!(-6.0, 5.5, -3.14159));
    }

    #[test]
    fn display() {
        let a = vec3!(0.0, -6.0, 8.659_834);
        assert_eq!(format!("a = {}", a), "a = <0, -6, 8.659834>");
    }
}
