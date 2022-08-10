use std::f32::consts::PI;

use crate::math::{Mat4, Quat, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub view: Mat4,
    pub projection: Mat4,
}

#[derive(PartialEq)]
pub enum MoveDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

impl Camera {
    pub fn new(
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut camera = Camera {
            eye,
            target,
            up,
            fov,
            aspect,
            near,
            far,
            view: Mat4::identity(),
            projection: Mat4::identity(),
        };
        camera.update_view_matrix();
        camera.update_projection_matrix();
        camera
    }

    pub fn update_view_matrix(&mut self) {
        self.view = Mat4::lookat(self.eye, self.target, self.up);
    }

    pub fn update_projection_matrix(&mut self) {
        self.projection = Mat4::perspective(self.fov, self.aspect, self.near, self.far);
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) -> Mat4 {
        let new_axis = (self.view.inverse() * Vec4::from_vec3(&axis, 0.0)).to_vec3();
        Quat::from_angle_axis(angle, new_axis).to_mat4()
    }
}
