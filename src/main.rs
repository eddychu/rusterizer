extern crate minifb;
mod arcball;
mod camera;
mod framebuffer;
mod loader;
mod math;
mod mesh;
mod renderstate;
mod texture;
use std::f32::consts::PI;

use arcball::Arcball;
use camera::{Camera, MoveDirection};
use framebuffer::FrameBuffer;
use math::{Mat4, Quat, Vec2, Vec3, Vec4};

use mesh::{Cube, Mesh};
use minifb::{Key, Window, WindowOptions};
use renderstate::RenderState;
use texture::Texture;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// a simple cube for test

fn main() {
    let mut buffer = FrameBuffer::new(WIDTH, HEIGHT);
    let mut arcball = Arcball::new(WIDTH as u32, HEIGHT as u32);

    let aspect = (WIDTH as f32) / (HEIGHT as f32);
    let fov = 60.0;
    let near = 1.0;
    let far = 100.0;

    let eye = Vec3::new(0.0, 0.0, 5.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(eye, target, up, fov, aspect, near, far);

    let cube = Cube::new(1.0);

    let albedo = Texture::from_file("asset/2.0/BoxTextured/glTF/CesiumLogoFlat.png");
    let mvp = camera.projection * camera.view;
    let mut state = RenderState {
        camera,
        albedo,
        mvp,
        target: buffer,
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut started = false;
    let mut mouse_pos_x = 0.0;
    let mut mouse_pos_y = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        // if window.is_key_pressed(Key::W, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Forward);
        // }
        // if window.is_key_pressed(Key::S, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Backward);
        // }

        // if window.is_key_pressed(Key::L, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Left);
        // }

        // if window.is_key_pressed(Key::R, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Right);
        // }

        // if window.is_key_pressed(Key::Up, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Up);
        // }

        // if window.is_key_pressed(Key::Down, minifb::KeyRepeat::Yes) {
        //     state.camera.move_position(MoveDirection::Down);
        // }

        if window.get_mouse_down(minifb::MouseButton::Left) {
            if !started {
                (mouse_pos_x, mouse_pos_y) =
                    window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
            } else {
                let (new_pos_x, new_pos_y) =
                    window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
                if new_pos_x != mouse_pos_x || new_pos_y != mouse_pos_y {
                    let va = arcball.get_arcball_vector(mouse_pos_x, mouse_pos_y);
                    let vb = arcball.get_arcball_vector(new_pos_x, new_pos_y);
                    let angle = va.dot(&vb).min(1.0).acos();
                    let axis = va.cross(&vb);
                    let rot = state.camera.rotate(angle, axis);
                    arcball.rot = rot * arcball.rot;
                    state.mvp = state.camera.projection * state.camera.view * arcball.rot;
                    mouse_pos_x = new_pos_x;
                    mouse_pos_y = new_pos_y;
                }
            }

            started = true;
        } else {
            started = false;
        }

        state.target.clear();

        cube.draw(&mut state);
        // return;
        window
            .update_with_buffer(&state.target.pixels, WIDTH, HEIGHT)
            .unwrap();
        // angles += 0.02;
    }
}
