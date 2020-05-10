//! A box (or block, because of Rust naming conventions).

use crate::{
    aabb::AABB,
    create_world,
    hittable::{
        aa_rect::{XYRect, XZRect, YZRect},
        world::World,
        HitRecord, Hittable,
    },
    material::Material,
    ray::Ray,
    vec3::{Axis::*, Vec3},
};

/// A axis-aligned block, made from 6 rectangles.
#[derive(Clone, Debug)]
pub struct Block {
    /// Minimum corner for the block.
    pub block_min: Vec3,
    /// Maximum corner for the block.
    pub block_max: Vec3,
    /// The sides of the block.
    sides: World,
}

impl Block {
    /// Create a new block.
    pub fn new(p0: Vec3, p1: Vec3, material: Material) -> Self {
        Self {
            block_min: p0,
            block_max: p1,
            sides: create_world!(
                // Front:
                XYRect::new(p0[X], p1[X], p0[Y], p1[Y], p1[Z], material.clone()),
                // Back:
                XYRect::new(p0[X], p1[X], p0[Y], p1[Y], p0[Z], material.clone()).flip_face(),
                // Top:
                XZRect::new(p0[X], p1[X], p0[Z], p1[Z], p1[Y], material.clone()),
                // Bottom:
                XZRect::new(p0[X], p1[X], p0[Z], p1[Z], p0[Y], material.clone()).flip_face(),
                // Right:
                YZRect::new(p0[Y], p1[Y], p0[Z], p1[Z], p1[X], material.clone()),
                // Left:
                YZRect::new(p0[Y], p1[Y], p0[Z], p1[Z], p0[X], material).flip_face(),
            ),
        }
    }
}

impl Hittable for Block {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.block_min, self.block_max))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
