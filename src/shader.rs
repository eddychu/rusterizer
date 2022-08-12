use crate::{math::Vec4, mesh::Vertex};

pub struct VertexOutput {
    pub clip_vertices: Vec<Vec4>,
}

pub trait Shader {
    fn vertex(vertice: &Vertex) -> VertexOutput;
    fn fragment() -> Vec4;
}

pub struct DiffuseShader {}
