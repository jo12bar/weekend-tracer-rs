//! Bounding Volume Heirarchies.
//!
//! These allow us to more effeciently compute what a ray might be able to hit
//! without having to construct a bunch of `HitRecord` structs and do a bunch of
//! calculations up front.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3,
    vec3::Vec3,
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
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn inner(mut objects: Vec<Box<dyn Hittable>>, n: usize, time0: f32, time1: f32) -> BVH {
            let mut boxes: Vec<AABB> = vec![AABB::new(vec3!(), vec3!()); n];
            let mut left_area: Vec<f32> = vec![0.0; n];
            let mut right_area: Vec<f32> = vec![0.0; n];

            let mut main_box: AABB = objects[0].bounding_box(time0, time1).unwrap();

            // Get the AABB that encompasses all objects:
            for obj in objects.iter().skip(1) {
                let new_box = obj.bounding_box(time0, time1).unwrap();
                main_box = AABB::surrounding_box(new_box, main_box);
            }

            // Get longest axis:
            // 0 == x, 1 == y, 2 == z.
            let axis = main_box.longest_axis();

            // Sort objects by bounds on longest axis:
            match axis {
                0 => objects.sort_unstable_by(|a, b| box_x_compare(a.as_ref(), b.as_ref())),
                1 => objects.sort_unstable_by(|a, b| box_y_compare(a.as_ref(), b.as_ref())),
                _ => objects.sort_unstable_by(|a, b| box_z_compare(a.as_ref(), b.as_ref())),
            }

            // Get all the bounding boxes:
            for i in 0..n {
                boxes[i] = objects[i].bounding_box(time0, time1).unwrap();
            }

            // Get culmultative areas starting with left-most box:
            left_area[0] = boxes[0].area();
            let mut left_box: AABB = boxes[0];

            for i in 1..(n - 1) {
                left_box = AABB::surrounding_box(left_box, boxes[i]);
                left_area[i] = left_box.area();
            }

            // Get culmultative area starting with right-most box:
            right_area[n - 1] = boxes[n - 1].area();
            let mut right_box = boxes[n - 1];

            for i in (1..=(n - 2)).rev() {
                right_box = AABB::surrounding_box(right_box, boxes[i]);
                right_area[i] = right_box.area();
            }

            // Compute minimum surface-area-hueristic (SAH):
            let mut min_sah = f32::MAX;
            let mut min_sah_idx = usize::MAX;
            for i in 0..(n - 1) {
                let sah = (i as f32) * left_area[i] + ((n - i - 1) as f32) * right_area[i + 1];
                if sah < min_sah {
                    min_sah_idx = i;
                    min_sah = sah;
                }
            }

            let left = if min_sah_idx == 0 {
                BVH::new(vec![objects[0].clone()], time0, time1)
            } else {
                inner(objects.clone(), min_sah_idx + 1, time0, time1)
            };

            let right = if min_sah_idx == n - 2 {
                BVH::new(vec![objects[min_sah_idx + 1].clone()], time0, time1)
            } else {
                inner(
                    objects.clone().drain((min_sah_idx + 1)..).collect(),
                    n - min_sah_idx - 1,
                    time0,
                    time1,
                )
            };

            BVH {
                bbox: main_box,
                size: left.size + right.size,
                contents: BVHContents::Node {
                    left: Box::new(left),
                    right: Box::new(right),
                },
            }
        }

        match objects.len() {
            0 => panic!("Can't create a BVH out of zero objects!"),
            1 => Self {
                bbox: objects[0].bounding_box(time0, time1).unwrap(),
                size: 1,
                contents: BVHContents::Leaf(objects.pop().unwrap()),
            },
            n => inner(objects, n, time0, time1),
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
