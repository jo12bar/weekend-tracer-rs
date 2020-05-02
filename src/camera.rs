//! A camera for viewing our world.

use crate::{ray::Ray, util::deg_to_rad, vec3, vec3::Vec3};
use rand::Rng;

/// A simple axis-aligned camera.
#[derive(Debug, Copy, Clone)]
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

    /// Horizontal component of orthogonal basis.
    u: Vec3,
    /// Vertical component of orthogonal basis.
    v: Vec3,
    /// Depth-wise component of orthogonal basis.
    w: Vec3,

    /// The radius of the lens.
    lens_radius: f32,
}

impl Camera {
    /// Create a new camera.
    ///
    /// - `lookfrom` is the point where the camera is in the world.
    /// - `lookat` is the point that the camera is looking at.
    /// - `vup` is the camera's upwards vector, which can change things like the
    ///   angle the camera is rolled at.
    /// - `vfov` is the top-to-bottom field of view, in degrees.
    /// - `aspect` is the aspect ratio, width:height.
    /// - `aperture` is the camera's aperture.
    /// - `focus_distance` is the distance from the camera that is in focus.
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Find an orthonormal basis {u,v,w} to describe our camera's
        // orientation. Note that vup, v, and w are all in the same plane. Our
        // camera will face point lookat, which is in the -w direction.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        Self {
            lower_left_corner: lookfrom
                - half_width * focus_distance * u
                - half_height * focus_distance * v
                - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            origin: lookfrom,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    /// Returns a ray that starts at the camera's origin and passes through
    /// screen coordinate (s, t). Will change starting location based on
    /// aperture of the camera and focal length.
    pub fn get_ray<R: Rng + ?Sized>(&self, rng: &mut R, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            vec3!(0.0, 0.0, 0.0),
            vec3!(0.0, 0.0, -1.0),
            vec3!(0.0, 1.0, 0.0),
            90.0,
            2.0,
            1.0,
            1.0,
        )
    }
}
