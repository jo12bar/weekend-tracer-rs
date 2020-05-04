//! Structs and methods related to operating on 3D vectors.

use rand::prelude::*;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3D vector. Could be utilized for points, colours, actual vectors, etc...
///
/// To access colors, you can do:
///
/// 1. Tuple-style: `v.0`, `v.1`, `v.2`
/// 2. Using the `Axis` enum: `v[X]`. `v[Y]`. `v[Z]`. This requires a `use
///    weekend_tracer_rs::vec3::Axis::*;` statement.
/// 3. Using the `Channel` enum: `v[R]`, `v[G]`, `v[B]`. This requires a `use
///    weekend_tracer_rs::vec3::Channel::*;` statement.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    /// Create a new 3D vector.
    ///
    /// For convenience, the `vec3!` macro is also provided. Use it like this:
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    /// use weekend_tracer_rs::vec3;
    ///
    /// assert_eq!(vec3!(), Vec3(0.0, 0.0, 0.0));
    /// assert_eq!(vec3!(1.0), Vec3(1.0, 0.0, 0.0));
    /// assert_eq!(vec3!(1.0, -3.0), Vec3(1.0, -3.0, 0.0));
    /// assert_eq!(vec3!(1.0, -3.0, 4.3), Vec3(1.0, -3.0, 4.3));
    /// ```
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    /// Create some random vector, where each component ranges from [0, 1).
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let a = Vec3::random(&mut rng);
    ///
    /// assert_eq!(
    ///     a,
    ///     Vec3::new(
    ///         0.33838564,
    ///         0.5598705,
    ///         0.21751523,
    ///     ),
    /// );
    /// ```
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    /// Create some random vector, where each component ranges from [`min`, `max`).
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let a = Vec3::random_range(&mut rng, -5.0, 20.0);
    ///
    /// assert_eq!(
    ///     a,
    ///     Vec3::new(
    ///         3.4596395,
    ///         8.996762,
    ///         0.43788052
    ///     ),
    /// );
    pub fn random_range<R: Rng + ?Sized>(rng: &mut R, min: f32, max: f32) -> Self {
        Vec3(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    /// Generate a random vector within the unit radius sphere.
    ///
    /// Works by first picking a random point in the unit cube, where x, y, and
    /// z all range from -1 to +1. Then, the point is rejected and we try again
    /// if the point is outside the sphere.
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let a = Vec3::random_in_unit_sphere(&mut rng);
    ///
    /// assert!(a.length_squared() < 1.0);
    /// assert_eq!(a.length_squared(), 0.4380054);
    ///
    /// assert_eq!(
    ///     a,
    ///     Vec3::new(
    ///         -0.32322884,
    ///         0.11974096,
    ///         -0.56496954,
    ///     ),
    /// );
    /// ```
    pub fn random_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut vector = Vec3(1.0, 1.0, 1.0);

        while vector.length_squared() >= 1.0 {
            vector = Vec3::random_range(rng, -1.0, 1.0);
        }

        vector
    }

    /// Generate a random unit vector. This is achieved by picking points on the
    /// surface of the unit sphere, which is in turn done by normalizing points
    /// picked in the unit ball.
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let a = Vec3::random_unit_vector(&mut rng);
    ///
    /// assert!(a.length() > 0.999 && a.length() < 1.001);
    ///
    /// assert_eq!(a, Vec3::new(
    ///     -0.5234415,
    ///     0.843606,
    ///     0.11974096,
    /// ));
    /// ```
    pub fn random_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let angle: f32 = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        let z: f32 = rng.gen_range(-1.0, 1.0);
        let radius = (1.0 - z * z).sqrt();

        Vec3(radius * angle.cos(), radius * angle.sin(), z)
    }

    /// Generate a random vector contained within the unit hemisphere
    /// surrounding a given normal vector.
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let normal_vec = Vec3::new(1.0, 2.0, -3.0);
    /// let a = Vec3::random_in_hemisphere(&mut rng, &normal_vec);
    ///
    /// // `a` should be in the same hemisphere as the normal:
    /// assert!(a.dot(&normal_vec) > 0.0);
    ///
    /// assert!(a.length_squared() < 1.0);
    /// assert_eq!(a.length_squared(), 0.4380054);
    ///
    /// assert_eq!(
    ///     a,
    ///     Vec3::new(
    ///         -0.32322884,
    ///         0.11974096,
    ///         -0.56496954,
    ///     ),
    /// );
    /// ```
    pub fn random_in_hemisphere<R: Rng + ?Sized>(rng: &mut R, normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);

        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal!
            in_unit_sphere
        } else {
            // Not in the same hemisphere as the normal! Flip it.
            -in_unit_sphere
        }
    }

    /// Generate a random vector within the unit disk.
    ///
    /// # Usage
    ///
    /// ```
    /// use rand::{Rng, SeedableRng};
    /// use rand_chacha::ChaCha8Rng;
    /// use weekend_tracer_rs::vec3::{Vec3, Axis::*};
    ///
    /// // This is just so we can have a reproducible source of random numbers
    /// // for testing purposes. You should probably use `rand::thread_rng()`
    /// // instead.
    /// let mut rng = ChaCha8Rng::seed_from_u64(10);
    ///
    /// let a = Vec3::random_in_unit_disk(&mut rng);
    ///
    /// assert!(a.length_squared() < 1.0);
    /// assert!(a[X] >= -1.0 && a[X] < 1.0);
    /// assert!(a[Y] >= -1.0 && a[Y] < 1.0);
    /// assert_eq!(a[Z], 0.0);
    ///
    /// assert_eq!(
    ///     a,
    ///     Vec3::new(-0.32322884, 0.11974096, 0.0),
    /// )
    /// ```
    pub fn random_in_unit_disk<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        let mut p = Vec3(1.0, 1.0, 0.0);

        while p.length_squared() >= 1.0 {
            p = Vec3(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        }

        p
    }

    /// Returns the length of the vector, squared.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -1.0, 1.0);
    /// assert_eq!(a.length_squared(), 3.0);
    /// ```
    pub fn length_squared(&self) -> f32 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }

    /// Returns the length of the vector.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(5.0, 10.0, -10.0);
    /// assert_eq!(a.length(), 15.0);
    /// ```
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Computes the [dot product](https://en.wikipedia.org/wiki/Dot_product) of
    /// two vectors.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -2.0, 3.0);
    /// let b = Vec3::new(-5.0, 9.0, 0.1);
    ///
    /// assert_eq!(a.dot(&b), -22.7);
    /// assert_eq!(b.dot(&a), -22.7);
    /// ```
    pub fn dot(&self, other: &Self) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    /// Computes the [cross product](https://en.wikipedia.org/wiki/Cross_product)
    /// of two vectors.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, -2.0, 3.0);
    /// let b = Vec3::new(-5.0, 9.0, 0.1);
    ///
    /// assert_eq!(a.cross(&b), Vec3::new(-27.2, -15.1, -1.0));
    /// assert_eq!(b.cross(&a), Vec3::new(27.2, 15.1, 1.0));
    /// ```
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3(
            (self.1 * rhs.2) - (self.2 * rhs.1),
            -((self.0 * rhs.2) - (self.2 * rhs.0)),
            (self.0 * rhs.1) - (self.1 * rhs.0),
        )
    }

    /// Returns the "unit vector version" of the original vector, which:
    ///
    /// - Is in the same direction as the original vector, and
    /// - Has length 1.
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(5.0, 10.0, -10.0);
    /// let ua = a.unit_vector();
    ///
    /// // Check lengths:
    /// assert_eq!(a.length(), 15.0);
    /// assert_eq!(ua.length(), 1.0);
    ///
    /// // Check directionality by checking that multiplying the unit vector by
    /// // the length (magnitude) of the original vector gives back the original
    /// // vector:
    /// assert_eq!(ua * a.length(), a);
    /// assert_eq!(ua * (-a.length()), -a);
    /// assert_eq!(ua * (-42.0 * a.length()), -42.0 * a);
    /// ```
    pub fn unit_vector(&self) -> Self {
        let inverse_length = 1.0 / self.length();
        Vec3(
            self.0 * inverse_length,
            self.1 * inverse_length,
            self.2 * inverse_length,
        )
    }

    /// Reflect a vector off of a surface, based on the normal vector to that
    /// surface.
    ///
    /// # Usage
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::Vec3;
    ///
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let norm = Vec3::new(-2.0, 3.0, -6.0);
    ///
    /// assert_eq!(a.reflect(&norm), Vec3::new(-55.0, 86.0, -165.0));
    /// ```
    pub fn reflect(&self, normal_vector: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal_vector) * (*normal_vector)
    }

    /// Refract a vector, given the normal vector to the surface where the
    /// vector is being refracted.
    ///
    /// **_Note_** that the last parameter, `etai_over_etat`, refers to the
    /// quantity η/η′. Here, η is the refractive index of the outside material,
    /// and η′ is the refractive index of the material to be refracted into.
    ///
    /// # Usage
    ///
    /// ```
    /// use weekend_tracer_rs::vec3::{Vec3, Axis::*};
    ///
    /// let a = Vec3::new(1.0, 0.0, 0.0);
    /// let norm = Vec3::new(1.0, 0.0, 2.0);
    /// let refracted = a.refract(&norm, 0.4);
    ///
    /// // We have to check each component here due to floating-point rounding
    /// // errors.
    /// assert!(refracted[X] > -0.601 && refracted[X] < -0.599);
    /// assert!(refracted[Y] > -0.001 && refracted[Y] < 0.001);
    /// assert!(refracted[Z] > -2.001 && refracted[Z] < -1.999);
    /// ```
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = normal.dot(&(-(*self)));
        let r_out_parallel = etai_over_etat * ((*self) + cos_theta * (*normal));
        let r_out_perp = -((1.0 - r_out_parallel.length_squared()).sqrt()) * (*normal);

        r_out_parallel + r_out_perp
    }
}

/// Broadcasts a single value to all vector lanes.
impl From<f32> for Vec3 {
    #[inline]
    fn from(v: f32) -> Self {
        Self(v, v, v)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3(self * vec.0, self * vec.1, self * vec.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.0, self.1, self.2)
    }
}

/// Allow accumulation of vectors from an iterator.
impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

/// Allow `Vec3` to be produced by `rand::Rng::gen`.
///
/// The resulting vector has each component in the range [0, 1).
impl Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

/// Names for vector lanes when used as a color.
///
/// `Vec3` has an `Index` impl for `Channel`, so you can use `Channel` values to
/// select components from a `Vec3`:
///
/// ```
/// use weekend_tracer_rs::vec3::{Vec3, Channel::*};
///
/// let v = Vec3(1.0, 2.0, 3.0);
/// assert_eq!(v[R], 1.0);
/// assert_eq!(v[G], 2.0);
/// assert_eq!(v[B], 3.0);
/// ```
#[derive(Copy, Clone, Debug)]
pub enum Channel {
    /// Red.
    R,
    /// Green.
    G,
    /// Blue.
    B,
}

use Channel::*;

impl std::ops::Index<Channel> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: Channel) -> &Self::Output {
        match idx {
            R => &self.0,
            G => &self.1,
            B => &self.2,
        }
    }
}

impl std::ops::IndexMut<Channel> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Channel) -> &mut Self::Output {
        match idx {
            R => &mut self.0,
            G => &mut self.1,
            B => &mut self.2,
        }
    }
}

/// Names for vector lanes when used as a coordinate.
///
/// `Vec3` has an `Index` impl for `Axis`, so you can use `Axis` values to
/// select components from a `Vec3`:
///
/// ```
/// use weekend_tracer_rs::vec3::{Vec3, Axis::*};
///
/// let v = Vec3(1.0, 2.0, 3.0);
/// assert_eq!(v[X], 1.0);
/// assert_eq!(v[Y], 2.0);
/// assert_eq!(v[Z], 3.0);
/// ```
#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

use Axis::*;

impl std::ops::Index<Axis> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: Axis) -> &Self::Output {
        match idx {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl std::ops::IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Axis) -> &mut Self::Output {
        match idx {
            X => &mut self.0,
            Y => &mut self.1,
            Z => &mut self.2,
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            i => panic!(
                "Out of bounds index access on Vec3! Tried to access index {}.",
                i
            ),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            i => panic!(
                "Out of bounds index access on Vec3! Tried to access index {}.",
                i
            ),
        }
    }
}
/// A convenience macro for more easily building `Vec3`'s. Use it like this:
///
/// ```
/// use weekend_tracer_rs::vec3::Vec3;
/// use weekend_tracer_rs::vec3;
///
/// assert_eq!(vec3!(), Vec3::new(0.0, 0.0, 0.0));
/// assert_eq!(vec3!(1.0), Vec3::new(1.0, 0.0, 0.0));
/// assert_eq!(vec3!(1.0, -3.0), Vec3::new(1.0, -3.0, 0.0));
/// assert_eq!(vec3!(1.0, -3.0, 4.3), Vec3::new(1.0, -3.0, 4.3));
/// ```
#[macro_export]
macro_rules! vec3 {
    () => {
        Vec3(0.0, 0.0, 0.0)
    };
    ($x:expr $(,)?) => {
        Vec3($x, 0.0, 0.0)
    };
    ($x:expr, $y:expr $(,)?) => {
        Vec3($x, $y, 0.0)
    };
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        Vec3($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_invocation() {
        assert_eq!(vec3!(4.0, 2.0, 1.0), Vec3(4.0, 2.0, 1.0));
        assert_eq!(vec3!(3.5, 64.2, -13.0), Vec3(3.5, 64.2, -13.0));
        assert_eq!(vec3!(), Vec3(0.0, 0.0, 0.0));
        assert_eq!(vec3!(-1.1), Vec3(-1.1, 0.0, 0.0));
        assert_eq!(vec3!(20.3, -5.6), Vec3(20.3, -5.6, 0.0));
    }

    #[test]
    fn add() {
        assert_eq!(
            vec3!(1.0, 2.0, 0.0) + vec3!(-1.0, 2.1, -4.2),
            vec3!(0.0, 4.1, -4.2)
        );
    }

    #[test]
    fn add_assign() {
        let mut a = vec3!();
        let b = vec3!(1.0, -8.8, 3.8);
        a += b;
        assert_eq!(a, Vec3(1.0, -8.8, 3.8));
    }

    #[test]
    fn sub() {
        assert_eq!(
            vec3!(1.0, -1.9, 3.45) - vec3!(0.0, 8.5, -5.4),
            vec3!(1.0, -10.4, 8.85)
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = vec3!();
        let b = vec3!(1.0, -8.8, 3.8);
        a -= b;
        assert_eq!(a, Vec3(-1.0, 8.8, -3.8));
    }

    #[test]
    fn mul() {
        assert_eq!(3.0 * vec3!(1.0, 2.0, -3.0), vec3!(3.0, 6.0, -9.0));
        assert_eq!(vec3!(1.0, 2.0, -3.0) * 3.0, vec3!(3.0, 6.0, -9.0));
        assert_eq!(
            vec3!(1.0, 2.0, 3.0) * vec3!(-1.0, 3.1),
            vec3!(-1.0, 6.2, 0.0),
        )
    }

    #[test]
    fn mul_assign() {
        let mut a = vec3!(1.0, 2.0, 3.0);
        let b = vec3!(-1.0, 0.0, 2.0);
        a *= 5.0;
        assert_eq!(a, vec3!(5.0, 10.0, 15.0));
        a *= b;
        assert_eq!(a, vec3!(-5.0, 0.0, 30.0));
    }

    #[test]
    fn div() {
        assert_eq!(vec3!(3.0, 6.0, -9.0) / 3.0, vec3!(1.0, 2.0, -3.0));
    }

    #[test]
    fn div_assign() {
        let mut a = vec3!(5.0, -15.0, 30.0);
        a /= 5.0;
        assert_eq!(a, vec3!(1.0, -3.0, 6.0));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn neg() {
        assert_eq!(-vec3!(6.0, -5.5, 3.14159), vec3!(-6.0, 5.5, -3.14159));
    }

    #[test]
    fn display() {
        let a = vec3!(0.0, -6.0, 8.659_834);
        assert_eq!(format!("a = {}", a), "a = <0, -6, 8.659834>");
    }
}
