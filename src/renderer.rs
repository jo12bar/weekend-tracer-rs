use crate::bvh::BVH;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Scatter, ScatterType};
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::util::clamp;
use crate::vec3;
use crate::vec3::{Channel::*, Vec3};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

/// A pixel. Components are ordered `R`, `G`, `B`. Each component should range
/// from 0-255.
pub type Pixel = (u32, u32, u32);

fn ray_color<R: Rng + ?Sized>(
    rng: &mut R,
    ray: &Ray,
    background_color: &Vec3,
    bvh: &BVH,
    lights: Arc<dyn Hittable>,
    reflection_depth: usize,
) -> Vec3 {
    if reflection_depth == 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        vec3!()
    } else if let Some(hit_record) = bvh.hit(ray, 0.001, f32::INFINITY) {
        //                                        ^^^^^
        //                                          |
        // This `0.001` is so that we don't get weird "shadow acne" due to
        // floating-point errors.
        //
        // We hit something! Scatter the ray based on material type. If it
        // successfully scattered, reflect the ray according by the material
        // type, and recurse. If it was absorbed, just return black.
        //
        // We also add on some emitted light if the ray hit some emitting material.

        let emitted =
            hit_record
                .material
                .emitted(&hit_record, hit_record.uv, &hit_record.hit_point);

        if let Some(Scatter {
            attenuation,
            scattered,
        }) = hit_record.material.scatter(rng, ray, &hit_record)
        {
            match scattered {
                ScatterType::Specular(specular_ray) => {
                    attenuation
                        * ray_color(
                            rng,
                            &specular_ray,
                            background_color,
                            bvh,
                            lights,
                            reflection_depth - 1,
                        )
                }

                ScatterType::PDF(scatter_pdf) => {
                    let light_pdf = PDF::hittable(lights.clone(), hit_record.hit_point);
                    let mixture_pdf = PDF::mixture(&light_pdf, &scatter_pdf);

                    let scattered =
                        Ray::new(hit_record.hit_point, mixture_pdf.generate(rng), ray.time);
                    let pdf_val = mixture_pdf.value(&scattered.direction);

                    emitted
                        + attenuation
                            * hit_record
                                .material
                                .scattering_pdf(rng, ray, &hit_record, &scattered)
                            * ray_color(
                                rng,
                                &scattered,
                                background_color,
                                bvh,
                                lights,
                                reflection_depth - 1,
                            )
                            / pdf_val
                }
            }
        } else {
            emitted
        }
    } else {
        // Didn't hit anything! Just render the background colour.
        *background_color
    }
}

/// Render the scene. Outputs a vector of (r, g, b) integer triples, one for
/// each pixel, which can range from 0 to 255.
#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_arguments)]
pub fn render(
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
    bvh: BVH,
    lights: Arc<dyn Hittable>,
    camera: Camera,
    background_color: Vec3,
) -> Vec<Pixel> {
    let pb_style = ProgressStyle::default_bar()
        .template("{spinner} {msg} [{elapsed_precise}] [{bar:30.yellow/blue}] {pos}/{len}")
        .progress_chars("=>-");

    let pb = ProgressBar::new((width * height) as u64);
    pb.set_style(pb_style);

    (0..(width * height))
        .into_par_iter()
        .progress_with(pb)
        .map_init(thread_rng, |rng, screen_pos| {
            let j = height - 1 - screen_pos / width;
            let i = screen_pos % width;

            // Take a whole bunch of samples within a pixel, and average out the
            // pixel's colour.
            let mut color = vec3!();
            for _ in 0..samples_per_pixel {
                // Each sample is offset by a small, random amount.
                let u = ((i as f32) + rng.gen::<f32>()) / (width as f32);
                let v = ((j as f32) + rng.gen::<f32>()) / (height as f32);

                let ray = camera.get_ray(rng, u, v);
                color += ray_color(
                    rng,
                    &ray,
                    &background_color,
                    &bvh,
                    lights.clone(),
                    max_reflection_depth,
                );
            }

            // Replace NaN components with zero.
            let mut r = if color[R].is_nan() { 0.0 } else { color[R] };
            let mut g = if color[G].is_nan() { 0.0 } else { color[G] };
            let mut b = if color[B].is_nan() { 0.0 } else { color[B] };

            // Divide the color total by the number of samples and gamma-correct
            // for a gamma value of 2.0.
            let scale = 1.0 / (samples_per_pixel as f32);
            r = (scale * r).sqrt();
            g = (scale * g).sqrt();
            b = (scale * b).sqrt();

            let ir = (256.0 * clamp(r, 0.0, 0.999)) as u32;
            let ig = (256.0 * clamp(g, 0.0, 0.999)) as u32;
            let ib = (256.0 * clamp(b, 0.0, 0.999)) as u32;

            (ir, ig, ib)
        })
        .collect()
}

/// Convert a rendered scene into a iterator over
/// [ARGB](https://en.wikipedia.org/wiki/RGBA_color_model#ARGB_(word-order))
/// 32-bit unsigned colour integers:
/// - The upper 8 bits is for the alpha channel.
/// - The next 8 bits is for the red channel.
/// - The next 8 bits is for the blue channel.
/// - The lowest 8 bits is for the green channel.
pub fn convert_to_argb<I>(rendered_scene: I) -> impl Iterator<Item = u32>
where
    I: IntoIterator<Item = Pixel>,
{
    rendered_scene.into_iter().map(|pixel| {
        let (r, g, b) = pixel;
        (255 << 24) | (r << 16) | (g << 8) | b
    })
}
