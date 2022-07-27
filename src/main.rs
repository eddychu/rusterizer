extern crate minifb;
mod framebuffer;
mod math;
mod rasterizer;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use framebuffer::FrameBuffer;
use math::{Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Vec3 {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denom = d00 * d11 - d01 * d01;
    if denom < f32::EPSILON {
        println!("{:?}", denom);
    }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    return Vec3::new(u, v, w);
}

fn render(buffer: &mut FrameBuffer) {
    let v0 = Vec3::new(150.0, 50.0, 0.0);
    let v1: Vec3 = Vec3::new(150.0, 550.0, 0.0);
    let v2: Vec3 = Vec3::new(650.0, 550.0, 0.0);
    let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    let min_x = v0.x.min(v1.x).min(v2.x).max(0.0) as usize;
    let min_y = v0.y.min(v1.y).min(v2.y).max(0.0) as usize;
    let max_x = v0.x.max(v1.x).max(v2.x).min((WIDTH - 1) as f32) as usize;
    let max_y = v0.y.max(v1.y).max(v2.y).min((HEIGHT - 1) as f32) as usize;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let p = Vec2::new(x as f32, y as f32);
            let a = Vec2::new(v0.x, v0.y);
            let b = Vec2::new(v1.x, v1.y);
            let c = Vec2::new(v2.x, v2.y);
            let bar = barycentric(p, a, b, c);

            if bar.x > f32::EPSILON && bar.y > f32::EPSILON && bar.z > f32::EPSILON {
                let color = c0.scale(bar.x) + c1.scale(bar.y) + c2.scale(bar.z);
                // println!("{:?}", color);
                buffer.write_pixel_vec3(x, y, color);
            }
        }
    }
}

fn main() {
    let mut buffer = FrameBuffer::new(WIDTH, HEIGHT);
    buffer.write_pixel_vec3(WIDTH / 2, HEIGHT / 2, Vec3::new(1.0, 0.0, 0.0));

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

        buffer.clear();
        render(&mut buffer);
        window
            .update_with_buffer(&buffer.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}
