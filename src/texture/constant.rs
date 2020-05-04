use crate::hittable::UVCoord;
use crate::texture::TextureTrait;
use crate::vec3::Vec3;

/// A texture that always outputs a constant colour.
#[derive(Debug, Clone, Copy)]
pub struct Constant {
    color: Vec3,
}

impl Constant {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl TextureTrait for Constant {
    fn value(&self, _uv: UVCoord, _point: &Vec3) -> Vec3 {
        self.color
    }
}
