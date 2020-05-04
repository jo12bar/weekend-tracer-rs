//! **A**xis-**A**ligned **B**ounding **B**oxes.
//!
//! These have the ability to very quickly and effeciently tell you if a `Ray`
//! intersects a certain area. The "slab" method is used, which allows us to
//! just compare intervals.

use crate::{
    ray::Ray,
    vec3,
    vec3::{Axis, Axis::*, Vec3},
};

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
            box0.min[X].min(box1.min[X]),
            box0.min[Y].min(box1.min[Y]),
            box0.min[Z].min(box1.min[Z]),
        );
        let large = vec3!(
            box0.max[X].max(box1.max[X]),
            box0.max[Y].max(box1.max[Y]),
            box0.max[Z].max(box1.max[Z]),
        );
        AABB::new(small, large)
    }

    /// Test if a ray hits the bounding box at some point.
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let mut tmin = tmin;
        let mut tmax = tmax;

        // Loop through each of the three components.
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

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

    /// Returns the longest axis in the box.
    pub fn longest_axis(&self) -> Axis {
        let mut ranges = [
            (X, self.axis_range(X)),
            (Y, self.axis_range(Y)),
            (Z, self.axis_range(Z)),
        ];
        // Note reversed comparison function, to sort from greatest to least:
        ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        ranges[0].0
    }

    /// Returns the range of an axis in the box.
    pub fn axis_range(&self, axis: Axis) -> f32 {
        let min = self.min[axis].min(self.max[axis]);
        let max = self.min[axis].max(self.max[axis]);

        max - min
    }

    /// Returns the surface area of a box.
    pub fn area(&self) -> f32 {
        let x = self.axis_range(X);
        let y = self.axis_range(Y);
        let z = self.axis_range(Z);
        2.0 * (x * y + x * z + y * z)
    }
}
