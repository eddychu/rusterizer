use crate::mesh::Mesh;

pub struct Model {
    pub mesh: Box<dyn Mesh>,
}
