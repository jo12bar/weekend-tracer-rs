use crate::texture::Texture;
use crate::vec3::Vec3;
use std::sync::Arc;

/// A texture that always outputs a constant colour.
pub fn constant(color: Vec3) -> Texture {
    Texture(Arc::new(move |_uv_coord, _hit_point| color))
}
