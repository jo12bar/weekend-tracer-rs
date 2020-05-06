//! Some pre-made scenes for your use.
use crate::{
    create_world,
    hittable::{moving_sphere::MovingSphere, sphere::Sphere, world::World, Hittable},
    material::Material,
    texture, vec3,
    vec3::Vec3,
};
use rand::prelude::*;

/// A scene with two checkerboarded spheres.
pub fn two_checkerboard_spheres() -> World {
    let texture = texture::checkerboard(vec3!(0.2, 0.3, 0.1).into(), vec3!(0.9, 0.9, 0.9).into());

    create_world!(
        Sphere::new(
            vec3!(0.0, -10.0),
            10.0,
            Material::lambertian(texture.clone())
        ),
        Sphere::new(vec3!(0.0, 10.0, 0.0), 10.0, Material::lambertian(texture)),
    )
}

/// A scene with two spheres with perlin noise textures.
pub fn two_perlin_spheres() -> World {
    let texture = texture::perlin_turbulence(3.0, None);

    create_world!(
        Sphere::new(
            vec3!(0.0, -1000.0, 0.0),
            1000.0,
            Material::lambertian(texture.clone())
        ),
        Sphere::new(vec3!(0.0, 2.0, 0.0), 2.0, Material::lambertian(texture)),
    )
}

/// A scene with two spheres that kind-of look like marble if you squint enough.
pub fn two_marble_ish_spheres() -> World {
    let texture = texture::simple_marble(3.0);

    create_world!(
        Sphere::new(
            vec3!(0.0, -1000.0, 0.0),
            1000.0,
            Material::lambertian(texture.clone())
        ),
        Sphere::new(vec3!(0.0, 2.0, 0.0), 2.0, Material::lambertian(texture)),
    )
}

/// A scene with the earth sitting on top of a checkerboard floor.
pub fn earth_on_checkerboard() -> World {
    // Image taken from wikimedia commons:
    let earth_tex = texture::image("./images/Mercator-projection.jpg");
    let checker_tex =
        texture::checkerboard(vec3!(0.2, 0.3, 0.1).into(), vec3!(0.9, 0.9, 0.9).into());

    create_world!(
        Sphere::new(
            vec3!(0.0, -1000.0, 0.0),
            1000.0,
            Material::lambertian(checker_tex)
        ),
        Sphere::new(vec3!(0.0, 2.0, 0.0), 2.0, Material::lambertian(earth_tex)),
    )
}

/// Create a random scene for funsies!
pub fn random_scene<R: Rng + ?Sized>(rng: &mut R) -> World {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::default();

    // Ground:
    objects.push(Box::new(Sphere::new(
        vec3!(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian(texture::checkerboard(
            vec3!(0.2, 0.3, 0.1).into(),
            texture::simple_marble(40.0),
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
        Material::lambertian(texture::image("./images/Mercator-projection.jpg")),
    )));

    // Large metal ball:
    objects.push(Box::new(Sphere::new(
        vec3!(4.0, 1.0),
        1.0,
        Material::metal(vec3!(0.7, 0.6, 0.5), 0.0),
    )));

    World { objects }
}
