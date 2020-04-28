//! Structs and methods for working with rays.
//!
//! All rays can be thought of as 3D lines:
//!
//! > **p&#x20D7;**(t) = **a** + t**b&#x20D7;**
//!
//! Where:
//!
//! - **p&#x20D7;** is the 3D position along the line (a vector).
//! - **a** is the ray's origin.
//! - **b&#x20D7;** is the direction and speed (i.e. the "gradient") of the ray
//!   (also a vector).
//! - t is a real number that moves you to different positions on the ray,
//!   linearly.

use crate::vec3::Vec3;

/// A ray in 3D, with some origin and direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// The ray's origin.
    pub origin: Vec3,
    // The ray's direction.
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new `Ray` at origin `origin` with direction `direction`.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Get the position of the ray at parameter `t`.
    ///
    /// ```
    /// use weekend_tracer_rs::ray::Ray;
    /// use weekend_tracer_rs::vec3;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let r = Ray::new(vec3!(), vec3!(1.0, 2.0, -3.0));
    ///
    /// assert_eq!(r.at(0.0), vec3!());
    /// assert_eq!(r.at(1.0), vec3!(1.0, 2.0, -3.0));
    /// assert_eq!(r.at(-2.0), vec3!(-2.0, -4.0, 6.0));
    /// ```
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
