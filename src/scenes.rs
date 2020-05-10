//! Some pre-made scenes for your use.
use crate::{
    create_world,
    hittable::{
        aa_rect::{XYRect, XZRect, YZRect},
        block::Block,
        moving_sphere::MovingSphere,
        sphere::Sphere,
        world::World,
        Hittable,
    },
    material::Material,
    texture, vec3,
    vec3::Vec3,
};
use rand::prelude::*;

/// A "Cornell Box" scene. Introduced in 1984, and is used to model the
/// interaction of light between diffuse surfaces.
pub fn cornell_box() -> World {
    let red = Material::lambertian(vec3!(0.65, 0.05, 0.05).into());
    let white = Material::lambertian(vec3!(0.73, 0.73, 0.73).into());
    let green = Material::lambertian(vec3!(0.12, 0.45, 0.15).into());

    let light = Material::diffuse_light(Vec3::from(15.0).into());

    create_world!(
        // Five walls:
        YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green), // left
        YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red),     // right
        XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()), // floor
        XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()), // ceiling
        XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()), // back
        // Light:
        XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
        // Blocks:
        Block::new(
            vec3!(130.0, 0.0, 65.0),
            vec3!(295.0, 165.0, 230.0),
            white.clone()
        ),
        Block::new(vec3!(265.0, 0.0, 295.0), vec3!(430.0, 330.0, 460.0), white),
    )
}

/// A scene with a perlin turbulence sphere on a perlin turbulence ground, with
/// a white diffuse light formed by a axis-aligned rectangle (`XYRect`). Oh: and
/// a floating, glowing sphere.
pub fn simple_lit_two_perlin_spheres() -> World {
    let pertext = texture::simple_marble(4.0);
    let difflight = Material::diffuse_light(vec3!(4.0, 4.0, 4.0).into());

    create_world!(
        // Ground:
        Sphere::new(
            vec3!(0.0, -1000.0),
            1000.0,
            Material::lambertian(pertext.clone())
        ),
        // Sphere:
        Sphere::new(vec3!(0.0, 2.0), 2.0, Material::lambertian(pertext)),
        // Floating glowing sphere:
        Sphere::new(vec3!(0.0, 7.0), 2.0, difflight.clone()),
        // Glowing rectangle:
        XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight),
    )
}

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
