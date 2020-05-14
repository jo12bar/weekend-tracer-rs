//! Methods and types and such for working with orthonormal bases.

use crate::{
    vec3,
    vec3::{Axis::*, Vec3},
};

/// An orthonormal basis (in R^3). Represents a set of three vectors which are
/// mutually orthogonal to each other. Any other vector in R^3 may be
/// represented as a linear combination of these three vectors. All three
/// vectors should be of unit length.
#[derive(Debug, Clone, Copy)]
pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    /// Create a new orthonormal basis based off of some vector.
    #[allow(clippy::many_single_char_names)]
    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit_vector();

        let a = if w[X].abs() > 0.9 {
            vec3!(0.0, 1.0, 0.0)
        } else {
            vec3!(1.0, 0.0, 0.0)
        };

        let v = w.cross(&a).unit_vector();
        let u = w.cross(&v);

        Self { u, v, w }
    }

    /// Get a vector in terms of this orthonormal basis.
    pub fn local(&self, a: &Vec3) -> Vec3 {
        a[X] * self.u + a[Y] * self.v + a[Z] * self.w
    }

    /// Get a vector to some coordinates in terms of this orthonormal basis.
    pub fn local_coords(&self, a: f32, b: f32, c: f32) -> Vec3 {
        a * self.u + b * self.v + c * self.w
    }
}

impl Default for ONB {
    fn default() -> Self {
        Self {
            u: vec3!(1.0, 0.0, 0.0),
            v: vec3!(0.0, 1.0, 0.0),
            w: vec3!(0.0, 0.0, 1.0),
        }
    }
}

impl std::ops::Index<usize> for ONB {
    type Output = Vec3;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.u,
            1 => &self.v,
            2 => &self.w,
            i => panic!(
                "Out of bounds index access on Vec3! Tried to access index {}.",
                i
            ),
        }
    }
}

impl std::ops::IndexMut<usize> for ONB {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.u,
            1 => &mut self.v,
            2 => &mut self.w,
            i => panic!(
                "Out of bounds index access on Vec3! Tried to access index {}.",
                i
            ),
        }
    }
}
