//! Perlin noise. Allows for fast random textures.

use crate::{
    texture::Texture,
    vec3::{Axis::*, Vec3},
};
use lazy_static::lazy_static;
use rand::prelude::*;
use std::sync::Arc;

/// The number of points to use for noise generation.
const POINT_COUNT: usize = 256;

lazy_static! {
    static ref RANFLOAT: [f32; POINT_COUNT] = generate_floats(&mut thread_rng());
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

fn generate_floats<R: Rng + ?Sized>(rng: &mut R) -> [f32; POINT_COUNT] {
    let mut ranfloat = [0.0_f32; POINT_COUNT];

    for element in ranfloat.iter_mut() {
        *element = rng.gen();
    }

    ranfloat
}

/// Linearly interpolate in 3 dimensions across some corners.
fn trilinear_interpolate(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0_f32;

    for (i, x) in c.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            for (k, z) in y.iter().enumerate() {
                accum += (((i as f32) * u) + ((1 - i) as f32) * (1.0 - u))
                    * (((j as f32) * v) + ((1 - j) as f32) * (1.0 - v))
                    * (((k as f32) * w) + ((1 - k) as f32) * (1.0 - w))
                    * z;
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

    let mut u = p[X] - i;
    let mut v = p[Y] - j;
    let mut w = p[Z] - k;

    u = u * u * (3.0 - 2.0 * u);
    v = v * v * (3.0 - 2.0 * v);
    w = w * w * (3.0 - 2.0 * w);

    let mut c = [[[0.0_f32; 2]; 2]; 2];

    for (di, x) in c.iter_mut().enumerate() {
        for (dj, y) in x.iter_mut().enumerate() {
            for (dk, z) in y.iter_mut().enumerate() {
                let ix = PERM_X[((i as i32 + di as i32) & 255) as usize];
                let iy = PERM_X[((j as i32 + dj as i32) & 255) as usize];
                let iz = PERM_X[((k as i32 + dk as i32) & 255) as usize];
                *z = RANFLOAT[(ix ^ iy ^ iz) as usize];
            }
        }
    }

    trilinear_interpolate(&c, u, v, w)
}

/// Creates a random texture made of perlin noise.
pub fn perlin_noise(scale: f32) -> Texture {
    Texture(Arc::new(move |_uv_coords, hit_point| {
        Vec3::from(1.0) * noise(&(hit_point * scale))
    }))
}
