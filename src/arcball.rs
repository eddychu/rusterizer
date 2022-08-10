use crate::math::{Mat4, Vec3};

pub struct Arcball {
    pub width: u32,
    pub height: u32,
    pub rot: Mat4,
}

impl Arcball {
    pub fn new(width: u32, height: u32) -> Self {
        Arcball {
            width,
            height,
            rot: Mat4::identity(),
        }
    }

    pub fn get_arcball_vector(&self, x: f32, y: f32) -> Vec3 {
        let mut p = Vec3::new(
            x * 2.0 / self.width as f32 - 1.0,
            1.0 - 2.0 * y as f32 / self.height as f32,
            0.0,
        );
        let op_squared = p.x * p.x + p.y * p.y;
        if op_squared <= 1.0 {
            p.z = (1.0 - op_squared).sqrt();
        } else {
            p = p.normalize();
        }
        p
    }
}
