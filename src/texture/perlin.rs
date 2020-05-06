//! Perlin noise. Allows for fast random textures.

use crate::{
    texture::Texture,
    vec3,
    vec3::{Axis::*, Vec3},
};
use lazy_static::lazy_static;
use rand::prelude::*;
use std::sync::Arc;

/// The number of points to use for noise generation.
const POINT_COUNT: usize = 256;

lazy_static! {
    static ref RANVEC: [Vec3; POINT_COUNT] = generate_vectors(&mut thread_rng());
    static ref PERM_X: [u8; POINT_COUNT] = perlin_generate_perm(&mut thread_rng());
    static ref PERM_Y: [u8; POINT_COUNT] = perlin_generate_perm(&mut thread_rng());
    static ref PERM_Z: [u8; POINT_COUNT] = perlin_generate_perm(&mut thread_rng());
}

fn permutate<R: Rng + ?Sized>(rng: &mut R, p: &mut [u8; POINT_COUNT], n: usize) {
    for i in (1..n).rev() {
        let target: usize = rng.gen_range(0, i);
        p.swap(i, target);
    }
}

fn perlin_generate_perm<R: Rng + ?Sized>(rng: &mut R) -> [u8; POINT_COUNT] {
    let mut p = [0_u8; POINT_COUNT];

    for (i, element) in p.iter_mut().enumerate() {
        *element = i as u8;
    }

    permutate(rng, &mut p, POINT_COUNT);

    p
}

fn generate_vectors<R: Rng + ?Sized>(rng: &mut R) -> [Vec3; POINT_COUNT] {
    let mut ranvec = [vec3!(); POINT_COUNT];

    for element in ranvec.iter_mut() {
        *element = Vec3::random_range(rng, -1.0, 1.0).unit_vector();
    }

    ranvec
}

/// Linearly interpolate in 3 dimensions across some corners.
fn trilinear_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for (i, x) in c.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            for (k, z) in y.iter().enumerate() {
                let weight = vec3!(u - i as f32, v - j as f32, w - k as f32);
                accum += (((i as f32) * uu) + ((1 - i) as f32) * (1.0 - uu))
                    * (((j as f32) * vv) + ((1 - j) as f32) * (1.0 - vv))
                    * (((k as f32) * ww) + ((1 - k) as f32) * (1.0 - ww))
                    * z.dot(&weight);
            }
        }
    }

    accum
}

/// Generate some perlin noise at a point.
#[allow(clippy::many_single_char_names)]
fn noise(p: &Vec3) -> f32 {
    let i = p[X].floor();
    let j = p[Y].floor();
    let k = p[Z].floor();

    let u = p[X] - i;
    let v = p[Y] - j;
    let w = p[Z] - k;

    let mut c = [[[vec3!(); 2]; 2]; 2];

    for (di, x) in c.iter_mut().enumerate() {
        for (dj, y) in x.iter_mut().enumerate() {
            for (dk, z) in y.iter_mut().enumerate() {
                let ix = PERM_X[((i as i32 + di as i32) & 255) as usize];
                let iy = PERM_X[((j as i32 + dj as i32) & 255) as usize];
                let iz = PERM_X[((k as i32 + dk as i32) & 255) as usize];
                *z = RANVEC[(ix ^ iy ^ iz) as usize];
            }
        }
    }

    trilinear_interpolate(&c, u, v, w)
}

/// Creates a random texture made of perlin noise.
pub fn perlin_noise(scale: f32) -> Texture {
    Texture(Arc::new(move |_uv_coords, hit_point| {
        Vec3::from(1.0) * 0.5 * (1.0 + noise(&(hit_point * scale)))
    }))
}
