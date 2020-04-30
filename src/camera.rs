//! A camera for viewing our world.

use crate::{ray::Ray, vec3, vec3::Vec3};

/// A simple axis-aligned camera.
pub struct Camera {
    /// The lower-left corner of our "screen", in relation the the camera's
    /// `origin`.
    pub lower_left_corner: Vec3,
    /// The horizontal width of our "screen".
    pub horizontal: Vec3,
    /// The vertical height of our "screen".
    pub vertical: Vec3,
    /// The location of our camera.
    pub origin: Vec3,
}

impl Camera {
    /// Returns a ray that starts at the camera's origin and passes through
    /// screen coordinate (u, v).
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            lower_left_corner: vec3!(-2.0, -1.0, -1.0),
            horizontal: vec3!(4.0, 0.0, 0.0),
            vertical: vec3!(0.0, 2.0, 0.0),
            origin: vec3!(0.0, 0.0, 0.0),
        }
    }
}
