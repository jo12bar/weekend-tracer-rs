//! For structs and functions that rotate `Hittable` objects.
//! Depending on which axis we are rotating about, we have different
//! formulas for finding the rotated components of the ray.
//!
//! For rotating counterclockwise (CCW) about the Z axis, we have:
//!
//!      x' = x⋅cos(θ) - y⋅sin(θ)
//!      y' = x⋅sin(θ) + y⋅cos(θ)
//!
//! For rotating CCW about the Y axis:
//!
//!      x' =  x⋅cos(θ) + z⋅sin(θ)
//!      z' = -x⋅sin(θ) + z⋅cos(θ)
//!
//! For rotating CCW about the X axis:
//!
//!      y' = y⋅cos(θ) - z⋅sin(θ)
//!      z' = y⋅sin(θ) + z⋅cos(θ)

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3,
    vec3::{Axis, Axis::*, Vec3},
};

/// Represents some rotation about either the X, Y, or Z axis.
#[derive(Debug, Clone)]
pub enum Rotate {
    X(RotateX),
    Y(RotateY),
    Z(RotateZ),
}

impl Rotate {
    /// Create a new rotation instance for rotating some `Hittable` object about
    /// either the X, Y, or Z axis by θ degrees.
    pub fn new(obj: Box<dyn Hittable>, axis: Axis, angle: f32) -> Self {
        match axis {
            X => Self::X(RotateX::new(obj, angle)),
            Y => Self::Y(RotateY::new(obj, angle)),
            Z => Self::Z(RotateZ::new(obj, angle)),
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match &self {
            Self::X(x) => x.hit(ray, t_min, t_max),
            Self::Y(y) => y.hit(ray, t_min, t_max),
            Self::Z(z) => z.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match &self {
            Self::X(x) => x.bounding_box(t0, t1),
            Self::Y(y) => y.bounding_box(t0, t1),
            Self::Z(z) => z.bounding_box(t0, t1),
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

/// A rotation instance. Holds a `Hittable` object, and rotates it about the X
/// axis by some amount of degrees.
#[derive(Debug, Clone)]
pub struct RotateX {
    /// sin(θ), where θ is the angle to rotate by in radians.
    sin_theta: f32,
    /// cos(θ), where θ is the angle to rotate by in radians.
    cos_theta: f32,
    /// The object itself.
    obj: Box<dyn Hittable>,
}

impl RotateX {
    /// Creates a new rotation instance for some `Hittable` object. The object
    /// is rotated about the x axis by θ degrees.
    pub fn new(obj: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = crate::util::deg_to_rad(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Self {
            sin_theta,
            cos_theta,
            obj,
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[Y] = ray.origin[Y] * self.cos_theta + ray.origin[Z] * self.sin_theta;
        origin[Z] = -ray.origin[Y] * self.sin_theta + ray.origin[Z] * self.cos_theta;

        direction[Y] = ray.direction[Y] * self.cos_theta + ray.direction[Z] * self.sin_theta;
        direction[Z] = -ray.direction[Y] * self.sin_theta + ray.direction[Z] * self.cos_theta;

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(rec) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut hit_point = rec.hit_point;
            let mut normal = rec.normal;

            hit_point[Y] = rec.hit_point[Y] * self.cos_theta - rec.hit_point[Z] * self.sin_theta;
            hit_point[Z] = rec.hit_point[Y] * self.sin_theta + rec.hit_point[Z] * self.cos_theta;

            normal[Y] = rec.normal[Y] * self.cos_theta - rec.normal[Z] * self.sin_theta;
            normal[Z] = rec.normal[Y] * self.sin_theta + rec.normal[Z] * self.cos_theta;

            Some(HitRecord::new(
                &rotated_ray,
                rec.t,
                hit_point,
                normal,
                rec.material,
                rec.uv,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.obj.bounding_box(t0, t1) {
            let mut min = Vec3::from(std::f32::MAX);
            let mut max = Vec3::from(std::f32::MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max[X] + (1 - i) as f32 * bbox.min[X];
                        let y = j as f32 * bbox.max[Y] + (1 - j) as f32 * bbox.min[Y];
                        let z = k as f32 * bbox.max[Z] + (1 - k) as f32 * bbox.min[Z];

                        let new_y = y * self.cos_theta - z * self.sin_theta;
                        let new_z = y * self.sin_theta + z * self.cos_theta;

                        let tester = vec3!(x, new_y, new_z);

                        for component in 0..3 {
                            min[component] = min[component].min(tester[component]);
                            max[component] = max[component].max(tester[component]);
                        }
                    }
                }
            }

            Some(AABB::new(min, max))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

/// A rotation instance. Holds a `Hittable` object, and rotates it about the Y
/// axis by some amount of degrees.
#[derive(Debug, Clone)]
pub struct RotateY {
    /// sin(θ), where θ is the angle to rotate by in radians.
    sin_theta: f32,
    /// cos(θ), where θ is the angle to rotate by in radians.
    cos_theta: f32,
    /// The object itself.
    obj: Box<dyn Hittable>,
}

impl RotateY {
    /// Creates a new rotation instance for some `Hittable` object. The object
    /// is rotated about the y axis by θ degrees.
    pub fn new(obj: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = crate::util::deg_to_rad(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Self {
            sin_theta,
            cos_theta,
            obj,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[X] = ray.origin[X] * self.cos_theta - ray.origin[Z] * self.sin_theta;
        origin[Z] = ray.origin[X] * self.sin_theta + ray.origin[Z] * self.cos_theta;

        direction[X] = ray.direction[X] * self.cos_theta - ray.direction[Z] * self.sin_theta;
        direction[Z] = ray.direction[X] * self.sin_theta + ray.direction[Z] * self.cos_theta;

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(rec) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut hit_point = rec.hit_point;
            let mut normal = rec.normal;

            hit_point[X] = rec.hit_point[X] * self.cos_theta + rec.hit_point[Z] * self.sin_theta;
            hit_point[Z] = -rec.hit_point[X] * self.sin_theta + rec.hit_point[Z] * self.cos_theta;

            normal[X] = rec.normal[X] * self.cos_theta + rec.normal[Z] * self.sin_theta;
            normal[Z] = -rec.normal[X] * self.sin_theta + rec.normal[Z] * self.cos_theta;

            Some(HitRecord::new(
                &rotated_ray,
                rec.t,
                hit_point,
                normal,
                rec.material,
                rec.uv,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.obj.bounding_box(t0, t1) {
            let mut min = Vec3::from(std::f32::MAX);
            let mut max = Vec3::from(std::f32::MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max[X] + (1 - i) as f32 * bbox.min[X];
                        let y = j as f32 * bbox.max[Y] + (1 - j) as f32 * bbox.min[Y];
                        let z = k as f32 * bbox.max[Z] + (1 - k) as f32 * bbox.min[Z];

                        let new_x = x * self.cos_theta + z * self.sin_theta;
                        let new_z = -x * self.sin_theta + z * self.cos_theta;

                        let tester = vec3!(new_x, y, new_z);

                        for component in 0..3 {
                            min[component] = min[component].min(tester[component]);
                            max[component] = max[component].max(tester[component]);
                        }
                    }
                }
            }

            Some(AABB::new(min, max))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

/// A rotation instance. Holds a `Hittable` object, and rotates it about the Z
/// axis by some amount of degrees.
#[derive(Debug, Clone)]
pub struct RotateZ {
    /// sin(θ), where θ is the angle to rotate by in radians.
    sin_theta: f32,
    /// cos(θ), where θ is the angle to rotate by in radians.
    cos_theta: f32,
    /// The object itself.
    obj: Box<dyn Hittable>,
}

impl RotateZ {
    /// Creates a new rotation instance for some `Hittable` object. The object
    /// is rotated about the y axis by θ degrees.
    pub fn new(obj: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = crate::util::deg_to_rad(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Self {
            sin_theta,
            cos_theta,
            obj,
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[X] = ray.origin[X] * self.cos_theta + ray.origin[Y] * self.sin_theta;
        origin[Y] = -ray.origin[X] * self.sin_theta + ray.origin[Y] * self.cos_theta;

        direction[X] = ray.direction[X] * self.cos_theta + ray.direction[Y] * self.sin_theta;
        direction[Y] = -ray.direction[X] * self.sin_theta + ray.direction[Y] * self.cos_theta;

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(rec) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut hit_point = rec.hit_point;
            let mut normal = rec.normal;

            hit_point[X] = rec.hit_point[X] * self.cos_theta - rec.hit_point[Y] * self.sin_theta;
            hit_point[Y] = rec.hit_point[X] * self.sin_theta + rec.hit_point[Y] * self.cos_theta;

            normal[X] = rec.normal[X] * self.cos_theta - rec.normal[Y] * self.sin_theta;
            normal[Y] = rec.normal[X] * self.sin_theta + rec.normal[Y] * self.cos_theta;

            Some(HitRecord::new(
                &rotated_ray,
                rec.t,
                hit_point,
                normal,
                rec.material,
                rec.uv,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.obj.bounding_box(t0, t1) {
            let mut min = Vec3::from(std::f32::MAX);
            let mut max = Vec3::from(std::f32::MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max[X] + (1 - i) as f32 * bbox.min[X];
                        let y = j as f32 * bbox.max[Y] + (1 - j) as f32 * bbox.min[Y];
                        let z = k as f32 * bbox.max[Z] + (1 - k) as f32 * bbox.min[Z];

                        let new_x = x * self.cos_theta - y * self.sin_theta;
                        let new_y = x * self.sin_theta + y * self.cos_theta;

                        let tester = vec3!(new_x, new_y, z);

                        for component in 0..3 {
                            min[component] = min[component].min(tester[component]);
                            max[component] = max[component].max(tester[component]);
                        }
                    }
                }
            }

            Some(AABB::new(min, max))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
