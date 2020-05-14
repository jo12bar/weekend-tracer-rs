//! Materials. Allows for easy interchangibility between material types on
//! different objects.

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{
    hittable::{HitRecord, UVCoord},
    pdf::PDF,
    ray::Ray,
    texture::Texture,
    vec3,
    vec3::Vec3,
};
use rand::Rng;

/// The type of scattering. Either a PDF-based scatter or a pre-calculated
/// specular ray.
#[derive(Clone, Debug)]
pub enum ScatterType {
    Specular(Ray),
    PDF(PDF),
}

/// A scattered ray and its attenuation.
#[derive(Clone, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: ScatterType,
}

impl Scatter {
    pub fn new(attenuation: Vec3, scattered: ScatterType) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

/// A material. Each material will scatter incident light (and `Ray`'s) in
/// different ways.
#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
    DiffuseLight(diffuse_light::DiffuseLight),
    Isotropic(isotropic::Isotropic),
}

impl Material {
    /// Create a new lambertian material.
    pub fn lambertian(albedo: Texture) -> Self {
        Self::Lambertian(lambertian::Lambertian::new(albedo))
    }

    /// Create a new metallic material.
    pub fn metal(albedo: Vec3, fuzz: f32) -> Self {
        Self::Metal(metal::Metal::new(albedo, fuzz))
    }

    /// Create a new dielectric material.
    pub fn dielectric(refractive_index: f32, density: f32) -> Self {
        Self::Dielectric(dielectric::Dielectric::new(refractive_index, density))
    }

    /// Create a new dielectric material with a custom albedo.
    pub fn dielectric_with_albedo(albedo: Vec3, refractive_index: f32, density: f32) -> Self {
        Self::Dielectric(dielectric::Dielectric::new_with_albedo(
            albedo,
            refractive_index,
            density,
        ))
    }

    /// Create a new diffuse light.
    pub fn diffuse_light(emit: Texture) -> Self {
        Self::DiffuseLight(diffuse_light::DiffuseLight::new(emit))
    }

    /// Create a new isotropic material. Mainly useful for its isotropic
    /// scattering function.
    pub fn isotropic(albedo: Texture) -> Self {
        Self::Isotropic(isotropic::Isotropic::new(albedo))
    }

    /// Scatter a ray off a material. Will delegate to the material's
    /// implementation of `scatter()`. Returns `Some(Scatter)` if the ray is
    /// scattered, `None` if it isn't.
    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        match rec.material.as_ref() {
            Material::Lambertian(l) => l.scatter(rng, ray, rec),
            Material::Metal(m) => m.scatter(rng, ray, rec),
            Material::Dielectric(d) => d.scatter(rng, ray, rec),
            Material::DiffuseLight(dl) => dl.scatter(rng, ray, rec),
            Material::Isotropic(i) => i.scatter(rng, ray, rec),
        }
    }

    /// Samples a PDF for some ray, its scattered counterpart, and a hit record.
    /// Allows for things like biasing rays towards light sources.
    pub fn scattering_pdf<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        rec: &HitRecord,
        scattered: &Ray,
    ) -> f32 {
        match rec.material.as_ref() {
            Material::Lambertian(l) => l.scattering_pdf(rng, ray_in, rec, scattered),
            _ => 0.0,
        }
    }

    /// Emits light. By default, will just return `Vec3(0.0, 0.0, 0.0)`, as most
    /// materials don't emit light. Can be overridden, however.
    pub fn emitted(&self, rec: &HitRecord, uv_coord: UVCoord, point: &Vec3) -> Vec3 {
        match self {
            Material::DiffuseLight(dl) => dl.emitted(rec, uv_coord, point),
            _ => vec3!(0.0),
        }
    }
}
