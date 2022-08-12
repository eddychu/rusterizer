extern crate minifb;
mod arcball;
mod camera;
mod framebuffer;
mod loader;
mod math;
mod mesh;
mod renderstate;
mod shader;
mod texture;
use arcball::Arcball;
use camera::Camera;
use framebuffer::FrameBuffer;
use math::Vec3;

use mesh::{Cube, Mesh};
use minifb::{Key, Window, WindowOptions};
use renderstate::RenderState;
use texture::Texture;
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let buffer = FrameBuffer::new(WIDTH, HEIGHT);
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

    window.limit_update_rate(Some(std::time::Duration::from_millis(0)));

    let mut started = false;
    let mut mouse_pos_x = 0.0;
    let mut mouse_pos_y = 0.0;
    let mut now = std::time::Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        let elapsed_time = now.elapsed();
        println!("rendering took {} milliseconds.", elapsed_time.as_millis());
        now = std::time::Instant::now();
        window
            .update_with_buffer(&state.target.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}
