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
use rand::Rng;

/// A bounding volume heirarchy.
///
/// Mainly based off of [this example](https://github.com/cbiffle/rtiow-rust/blob/master/src/bvh.rs).
#[derive(Debug)]
pub struct BVH {
    pub bbox: AABB,
    pub size: usize,
    pub contents: BVHContents,
}

/// A node in a bounding volume heirarchy. Can hold references to left and right
/// children. The children should implement `Hittable`, which means that they
/// could either be objects or they could be more `BVHNodes`, fog clouds, etc...
#[derive(Debug)]
pub enum BVHContents {
    Node { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

impl BVH {
    /// Create a new `BVHNode`.
    pub fn new<R: Rng + ?Sized>(
        rng: &mut R,
        mut objects: Vec<Box<dyn Hittable>>,
        time0: f32,
        time1: f32,
    ) -> Self {
        // Randomly choose an axis:
        let axis: usize = rng.gen_range(0, 2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        match objects.len() {
            0 => panic!("Can't create a BVH from zero objects."),
            1 => Self {
                bbox: objects[0].bounding_box(time0, time1).unwrap(),
                size: 1,
                contents: BVHContents::Leaf(objects.pop().unwrap()),
            },
            _ => {
                objects.sort_unstable_by(|a, b| comparator(a.as_ref(), b.as_ref()));

                // Divide objects in two:
                let right = Box::new(BVH::new(
                    rng,
                    objects.drain(objects.len() / 2..).collect(),
                    time0,
                    time1,
                ));
                let left = Box::new(BVH::new(rng, objects, time0, time1));

                BVH {
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
}

/// Compares an axis of two bounding boxes.
fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> std::cmp::Ordering {
    let opt_box_a = a.bounding_box(0.0, 0.0);
    let opt_box_b = b.bounding_box(0.0, 0.0);

    match (opt_box_a, opt_box_b) {
        (Some(box_a), Some(box_b)) => {
            let a_min: [f32; 3] = box_a.min.into();
            let b_min: [f32; 3] = box_b.min.into();

            a_min[axis].partial_cmp(&b_min[axis]).unwrap()
        }
        (None, _) | (_, None) => {
            eprintln!(
                "Couldn't compute a bounding_box in BVH constructor. \
                 Continuing on like nothing happened."
            );
            std::cmp::Ordering::Equal
        }
    }
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
