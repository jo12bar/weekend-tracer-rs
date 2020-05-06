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

/// Generate some perlin noise at a point.
#[allow(clippy::many_single_char_names)]
fn noise(p: &Vec3) -> f32 {
    let _u = p[X] - p[X].floor();
    let _v = p[Y] - p[Y].floor();
    let _w = p[Z] - p[Z].floor();

    let i = ((4.0 * p[X]) as usize) & 255;
    let j = ((4.0 * p[Y]) as usize) & 255;
    let k = ((4.0 * p[Z]) as usize) & 255;

    RANFLOAT[(PERM_X[i] ^ PERM_Y[j] ^ PERM_Z[k]) as usize]
}

/// Creates a random texture made of perlin noise.
pub fn perlin_noise() -> Texture {
    Texture(Arc::new(|_uv_coords, hit_point| {
        Vec3::from(1.0) * noise(hit_point)
    }))
}
