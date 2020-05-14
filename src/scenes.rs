//! Some pre-made scenes for your use.
use crate::{
    camera::Camera,
    create_world,
    hittable::{
        aa_rect::{XYRect, XZRect, YZRect},
        block::Block,
        constant_medium::ConstantMedium,
        moving_sphere::MovingSphere,
        sphere::Sphere,
        world::World,
        Hittable,
    },
    material::Material,
    texture, vec3,
    vec3::{Axis::*, Vec3},
};
use rand::prelude::*;

/// The final scene from the book *Ray Tracing: The Next Week*.
pub fn tracer_the_next_week_final_scene() -> World {
    let mut rng = thread_rng();
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    // The ground is made up of boxes of randomly varying height:
    let ground_mat = Material::lambertian(vec3!(0.48, 0.83, 0.53).into());
    let ground_boxes_per_side = 20;
    let ground_box_width = 100.0;
    let mut ground: Vec<Box<dyn Hittable>> =
        Vec::with_capacity(ground_boxes_per_side * ground_boxes_per_side);

    for i in 0..ground_boxes_per_side {
        for j in 0..ground_boxes_per_side {
            let x0 = -1000.0 + (i as f32 * ground_box_width);
            let y0 = 0.0;
            let z0 = -1000.0 + (j as f32 * ground_box_width);
            let x1 = x0 + ground_box_width;
            let y1: f32 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + ground_box_width;

            ground.push(Box::new(Block::new(
                vec3!(x0, y0, z0),
                vec3!(x1, y1, z1),
                ground_mat.clone(),
            )));
        }
    }

    world.append(&mut ground);

    // Add a large light:
    let light = Material::diffuse_light(Vec3::from(7.0).into());
    world.push(Box::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 553.9, light,
    )));

    // Add a large, thin area fog:
    world.push(Box::new(ConstantMedium::new(
        Box::new(Sphere::new(vec3!(), 5000.0, Material::dielectric(1.5, 0.0))),
        0.0001,
        Vec3::from(1.0).into(),
    )));

    // Add a moving sphere:
    let center1 = vec3!(400.0, 400.0, 200.0);
    let center2 = center1 + vec3!(30.0);
    let moving_sphere_material = Material::lambertian(vec3!(0.7, 0.3, 0.1).into());
    world.push(Box::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    // Add a glass sphere:
    world.push(Box::new(Sphere::new(
        vec3!(260.0, 150.0, 45.0),
        50.0,
        Material::dielectric_with_albedo(vec3!(0.2, 0.9, 0.4), 1.5, 0.1),
    )));

    // Add a metal sphere:
    world.push(Box::new(Sphere::new(
        vec3!(0.0, 150.0, 145.0),
        50.0,
        Material::metal(vec3!(0.8, 0.8, 0.9), 10.0),
    )));

    // Add a blue subsurface reflection sphere:
    let ssr_boundary = Sphere::new(
        vec3!(360.0, 150.0, 145.0),
        70.0,
        Material::dielectric(1.5, 0.0),
    );
    world.push(ssr_boundary.box_clone());
    world.push(Box::new(ConstantMedium::new(
        Box::new(ssr_boundary),
        0.2,
        vec3!(0.2, 0.4, 0.9).into(),
    )));

    // Add the Earth:
    let earth_mat = Material::lambertian(texture::image("./images/Mercator-projection.jpg"));
    world.push(Box::new(Sphere::new(
        vec3!(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));

    // Add a marble sphere:
    let marble_mat = Material::lambertian(texture::simple_marble(0.1, X));
    world.push(Box::new(Sphere::new(
        vec3!(220.0, 280.0, 300.0),
        80.0,
        marble_mat,
    )));

    // Add a bunch of small white spheres in the shape of a cube:
    let white_mat = Material::lambertian(Vec3::from(0.73).into());
    let ns = 1000;
    let mut small_spheres: Vec<Box<dyn Hittable>> = Vec::with_capacity(ns);

    for _ in 0..ns {
        small_spheres.push(Box::new(Sphere::new(
            Vec3::random_range(&mut rng, 0.0, 165.0),
            10.0,
            white_mat.clone(),
        )));
    }

    world.push(
        World::new(small_spheres)
            .rotate(Y, 15.0)
            .translate(vec3!(-100.0, 270.0, 395.0))
            .box_clone(),
    );

    World::new(world)
}

/// A "Cornell Box" scene. Introduced in 1984, and is used to model the
/// interaction of light between diffuse surfaces.
pub fn cornell_box(aspect: f32) -> (World, Camera) {
    let red = Material::lambertian(vec3!(0.65, 0.05, 0.05).into());
    let white = Material::lambertian(vec3!(0.73, 0.73, 0.73).into());
    let green = Material::lambertian(vec3!(0.12, 0.45, 0.15).into());

    let light = Material::diffuse_light(Vec3::from(7.0).into());

    let world = create_world!(
        // Five walls:
        YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green), // left
        YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red),     // right
        XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()), // floor
        XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()), // ceiling
        XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()), // back
        // Light:
        XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light).flip_face(),
        // Blocks:
        Block::new(vec3!(), vec3!(165.0, 330.0, 165.0), white.clone())
            .rotate(Y, 15.0)
            .translate(vec3!(265.0, 0.0, 295.0)),
        Block::new(vec3!(), Vec3::from(165.0), white)
            .rotate(Y, -18.0)
            .translate(vec3!(130.0, 0.0, 65.0))
    );

    let lookfrom = vec3!(278.0, 278.0, -800.0);
    let lookat = vec3!(278.0, 278.0, 0.0);
    let vup = vec3!(0.0, 1.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let t0 = 0.0;
    let t1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect,
        aperture,
        dist_to_focus,
        t0,
        t1,
    );

    (world, cam)
}

/// A scene with a perlin turbulence sphere on a perlin turbulence ground, with
/// a white diffuse light formed by a axis-aligned rectangle (`XYRect`). Oh: and
/// a floating, glowing sphere.
pub fn simple_lit_two_perlin_spheres() -> World {
    let pertext = texture::simple_marble(4.0, Z);
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
    let texture = texture::simple_marble(3.0, Z);

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
            texture::simple_marble(40.0, Z),
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
                    Material::dielectric_with_albedo(albedo, 1.5, 0.5)
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
        Material::dielectric_with_albedo(vec3!(0.5, 0.5, 1.0), 1.5, 0.7),
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
