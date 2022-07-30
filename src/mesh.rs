use crate::math::{Vec2, Vec3};

pub trait Mesh {
    fn position(&self, i: usize) -> Vec3;
    fn indice(&self, i: usize) -> usize;
    fn num_of_indices(&self) -> usize;
    fn num_of_vertices(&self) -> usize;
}

pub struct Triangle {
    pub positions: Vec<Vec3>,
    pub indices: Vec<usize>,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Triangle {
            positions: vec![v0, v1, v2],
            indices: vec![0, 1, 2],
        }
    }
}

impl Mesh for Triangle {
    fn indice(&self, i: usize) -> usize {
        return self.indices[i];
    }

    fn position(&self, i: usize) -> Vec3 {
        return self.positions[i];
    }

    fn num_of_indices(&self) -> usize {
        self.indices.len()
    }

    fn num_of_vertices(&self) -> usize {
        self.positions.len()
    }
}

pub struct Cube {
    pub positions: Vec<Vec3>,
    pub indices: Vec<usize>,
}

impl Cube {
    pub fn new(side: f32) -> Self {
        let side2 = side / 2.0;
        let p = vec![
            // Front
            Vec3::new(-side2, -side2, side2),
            Vec3::new(side2, -side2, side2),
            Vec3::new(side2, side2, side2),
            Vec3::new(-side2, side2, side2),
            // Right
            Vec3::new(side2, -side2, side2),
            Vec3::new(side2, -side2, -side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(side2, side2, side2),
            // Back
            Vec3::new(-side2, -side2, -side2),
            Vec3::new(-side2, side2, -side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(side2, -side2, -side2),
            // Left
            Vec3::new(-side2, -side2, side2),
            Vec3::new(-side2, side2, side2),
            Vec3::new(-side2, side2, -side2),
            Vec3::new(-side2, -side2, -side2),
            // Bottom
            Vec3::new(-side2, -side2, side2),
            Vec3::new(-side2, -side2, -side2),
            Vec3::new(side2, -side2, -side2),
            Vec3::new(side2, -side2, side2),
            // Top
            Vec3::new(-side2, side2, side2),
            Vec3::new(side2, side2, side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(-side2, side2, -side2),
        ];

        let n = vec![
            // Front
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0), // Right
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0), // Back
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0), // Left
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0), // Bottom
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0), // Top
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let tc = vec![
            // Front
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Right
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Back
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Left
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Bottom
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Top
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        let i = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ];

        Cube {
            positions: p,
            indices: i,
        }
    }
}

impl Mesh for Cube {
    fn indice(&self, i: usize) -> usize {
        return self.indices[i];
    }

    fn position(&self, i: usize) -> Vec3 {
        return self.positions[i];
    }

    fn num_of_indices(&self) -> usize {
        self.indices.len()
    }

    fn num_of_vertices(&self) -> usize {
        self.positions.len()
    }
}
