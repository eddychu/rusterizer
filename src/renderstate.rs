use crate::camera::Camera;
use crate::framebuffer::FrameBuffer;
use crate::math::Mat4;
use crate::texture::Texture;
#[derive(Debug)]
pub struct RenderState {
    pub camera: Camera,
    pub albedo: Texture,
    pub mvp: Mat4,
    pub target: FrameBuffer,
}
