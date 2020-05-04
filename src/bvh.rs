//! Bounding Volume Heirarchies.
//!
//! These allow us to more effeciently compute what a ray might be able to hit
//! without having to construct a bunch of `HitRecord` structs and do a bunch of
//! calculations up front.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

/// A bounding volume heirarchy.
///
/// Mainly based off of [this example](https://github.com/cbiffle/rtiow-rust/blob/master/src/bvh.rs).
#[derive(Debug, Clone)]
pub struct BVH {
    pub bbox: AABB,
    pub size: usize,
    pub contents: BVHContents,
}

/// A node in a bounding volume heirarchy. Can hold references to left and right
/// children. The children should implement `Hittable`, which means that they
/// could either be objects or they could be more `BVHNodes`, fog clouds, etc...
#[derive(Debug, Clone)]
pub enum BVHContents {
    Node { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

impl BVH {
    /// Create a new `BVH`.
    ///
    /// Largely derived from Peter Shirley's implementation, but doesn't use
    /// random axis selection, avoiding some pathological cases.
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        // Find the bounding box that encompasses all objects
        let bbox: AABB = objects
            .iter()
            .skip(1)
            .fold(objects[0].bounding_box(time0, time1).unwrap(), |bb, obj| {
                AABB::surrounding_box(bb, obj.bounding_box(time0, time1).unwrap())
            });

        // Find the biggest axis for this set of objects
        let axis = bbox.longest_axis();

        // Sort objects along longest axis by 2*centroid.
        objects.sort_unstable_by(|a, b| {
            let a_bb = a.bounding_box(time0, time1).unwrap();
            let b_bb = b.bounding_box(time0, time1).unwrap();
            let a_bb_min: [f32; 3] = a_bb.min.into();
            let a_bb_max: [f32; 3] = a_bb.max.into();
            let b_bb_min: [f32; 3] = b_bb.min.into();
            let b_bb_max: [f32; 3] = b_bb.max.into();
            let a_2centroid = a_bb_min[axis] + a_bb_max[axis];
            let b_2centroid = b_bb_min[axis] + b_bb_max[axis];
            a_2centroid.partial_cmp(&b_2centroid).unwrap()
        });

        match objects.len() {
            0 => panic!("Can't create a BVH from zero objects!"),
            1 => Self {
                bbox,
                size: 1,
                contents: BVHContents::Leaf(objects.pop().unwrap()),
            },
            _ => {
                let right = Box::new(BVH::new(
                    objects.drain(objects.len() / 2..).collect(),
                    time0,
                    time1,
                ));
                let left = Box::new(BVH::new(objects, time0, time1));

                Self {
                    bbox: AABB::surrounding_box(left.bbox, right.bbox),
                    size: left.size + right.size,
                    contents: BVHContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            match &self.contents {
                BVHContents::Leaf(obj) => obj.hit(ray, t_min, t_max),

                BVHContents::Node { left, right } => {
                    let hit_left = left.hit(ray, t_min, t_max);

                    // Don't bother searching past the left hit in the right BVH:
                    let right_t_max = if let Some(rec) = &hit_left {
                        rec.t
                    } else {
                        t_max
                    };

                    let hit_right = right.hit(ray, t_min, right_t_max);

                    match (hit_left, hit_right) {
                        (h, None) | (None, h) => h,
                        (Some(hl), Some(hr)) => {
                            if hl.t < hr.t {
                                Some(hl)
                            } else {
                                Some(hr)
                            }
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
