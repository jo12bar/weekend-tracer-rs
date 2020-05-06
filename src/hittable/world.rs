//! The world to be rendered.

use crate::aabb::AABB;
use crate::hittable::{moving_sphere::MovingSphere, sphere::Sphere, HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::texture;
use crate::vec3;
use crate::vec3::Vec3;
use rand::Rng;

/// The world that needs to be rendered, with all of its objects. Every object
/// needs to implement `Hittable`. Coincidentally, this struct *also* implements
/// `Hittable`.
#[derive(Default, Debug, Clone)]
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    /// Create a new `World`, filled with the passed-in `objects`.
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        World { objects }
    }

    /// Create a random scene for funsies!
    pub fn random_scene<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::default();

        // Ground:
        objects.push(Box::new(Sphere::new(
            vec3!(0.0, -1000.0, 0.0),
            1000.0,
            Material::lambertian(texture::checkerboard(
                vec3!(0.2, 0.3, 0.1).into(),
                vec3!(0.9, 0.9, 0.9).into(),
            )),
        )));

        // Random small spheres:
        for a in -11..11 {
            for b in -11..11 {
                let center = vec3!(
                    (a as f32) + 0.9 * rng.gen::<f32>(),
                    0.2,
                    (b as f32) + 0.9 * rng.gen::<f32>(),
                );

                if (center - vec3!(4.0, 0.2)).length() > 0.9 {
                    let choose_mat: f32 = rng.gen();

                    let material = if choose_mat < 0.8 {
                        // Diffuse
                        let albedo = Vec3::random(rng) * Vec3::random(rng);
                        Material::lambertian(albedo.into())
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Vec3::random_range(rng, 0.5, 1.0);
                        let fuzz: f32 = rng.gen();
                        Material::metal(albedo, fuzz)
                    } else {
                        // Glass
                        let albedo = Vec3::random_range(rng, 0.5, 1.0);
                        Material::dielectric_with_albedo(albedo, 1.5)
                    };

                    if choose_mat < 0.8 {
                        // Diffuse material. Randombly translate y coordinate
                        // during capture.
                        objects.push(Box::new(MovingSphere::new(
                            center,
                            center + vec3!(0.0, rng.gen_range(0.0, 0.5), 0.0),
                            0.0,
                            1.0,
                            0.2,
                            material,
                        )));
                    } else {
                        // Either a metal or a dielectric. Doesn't move.
                        objects.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                }
            }
        }

        // Large glass ball:
        objects.push(Box::new(Sphere::new(
            vec3!(0.0, 1.0),
            1.0,
            Material::dielectric_with_albedo(vec3!(0.5, 0.5, 1.0), 1.5),
        )));

        // Large diffuse ball:
        objects.push(Box::new(Sphere::new(
            vec3!(-4.0, 1.0),
            1.0,
            Material::lambertian(vec3!(0.4, 0.2, 0.1).into()),
        )));

        // Large metal ball:
        objects.push(Box::new(Sphere::new(
            vec3!(4.0, 1.0),
            1.0,
            Material::metal(vec3!(0.7, 0.6, 0.5), 0.0),
        )));

        World { objects }
    }

    /// Add an object to the `World`.
    pub fn add(&mut self, object: Box<dyn Hittable>) -> &mut Self {
        self.objects.push(object);
        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // We want to keep track of the closest-hit object. So, we intialize the
        // closest value for `t` to `t_max`.
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(obj_hit_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = obj_hit_rec.t;
                rec = Some(obj_hit_rec);
            }
        }

        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut temp_box: Option<AABB>;
        let mut output_box = AABB::new(vec3!(), vec3!());

        for object in &self.objects {
            temp_box = object.bounding_box(t0, t1);
            if let Some(temp_box) = temp_box {
                output_box = if first_box {
                    temp_box
                } else {
                    AABB::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

/// A convenience macro for more easily building `World`'s.
#[macro_export]
macro_rules! create_world {
    ($($object:expr),* $(,)?) => {
        World::new(vec![
            $(Box::new($object)),*
        ])
    };
}
