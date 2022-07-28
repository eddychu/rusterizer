use crate::math::Mat4;
#[derive(Debug, Clone, Copy)]
pub struct RenderState {
    pub projection: Mat4,
    pub view: Mat4,
    pub mv: Mat4,
    pub mvp: Mat4,
}
