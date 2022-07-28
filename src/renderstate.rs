use crate::math::Mat4;
use crate::texture::Texture;
#[derive(Debug, Clone)]
pub struct RenderState {
    pub projection: Mat4,
    pub view: Mat4,
    pub mv: Mat4,
    pub mvp: Mat4,

    pub albedo: Texture,
}
