use crate::math::{Vec2, Vec3};

pub struct Cube {
    pub positions: Vec<Vec3>,
    pub vertex_coords: Vec<Vec2>,
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
            vertex_coords: tc,
        }
    }

    pub fn indice(&self, i: usize) -> usize {
        return self.indices[i];
    }

    pub fn position(&self, i: usize) -> Vec3 {
        return self.positions[i];
    }

    pub fn num_of_indices(&self) -> usize {
        self.indices.len()
    }

    pub fn num_of_vertices(&self) -> usize {
        self.positions.len()
    }
}
