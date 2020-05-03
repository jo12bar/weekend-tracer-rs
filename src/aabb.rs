//! **A**xis-**A**ligned **B**ounding **B**oxes.
//!
//! These have the ability to very quickly and effeciently tell you if a `Ray`
//! intersects a certain area. The "slab" method is used, which allows us to
//! just compare intervals.

use crate::{ray::Ray, vec3, vec3::Vec3};

/// An axis-aligned bounding box. The two corners are specified with the `min`
/// and `max` vectors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    /// The "minimum" corner of the bounding box. All components should be
    /// smaller than those of `max`.
    pub min: Vec3,
    /// The "maximum" corner of the bounding box. All components should be
    /// larger than those of `min`.
    pub max: Vec3,
}

impl AABB {
    /// Create a new axis-aligned bounding box. Preferably, all components of
    /// `min` should be smaller than all components of `max`.
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Computes the bounding box of two bounding boxes.
    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = vec3!(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let large = vec3!(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB::new(small, large)
    }

    /// Test if a ray hits the bounding box at some point.
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let direction: [f32; 3] = ray.direction.into();
        let origin: [f32; 3] = ray.origin.into();
        let min: [f32; 3] = self.min.into();
        let max: [f32; 3] = self.max.into();

        let mut tmin = tmin;
        let mut tmax = tmax;

        // Loop through each of the three components.
        for i in 0..3 {
            let inv_d = 1.0 / direction[i];
            let mut t0 = (min[i] - origin[i]) * inv_d;
            let mut t1 = (max[i] - origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}
