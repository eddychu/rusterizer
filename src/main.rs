extern crate minifb;
mod framebuffer;
mod math;
mod rasterizer;
mod renderstate;
use framebuffer::FrameBuffer;
use math::{Mat4, Vec2, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use renderstate::RenderState;

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
    // if denom < f32::EPSILON {
    //     println!("{:?}", denom);
    // }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    return Vec3::new(u, v, w);
}

fn render(buffer: &mut FrameBuffer, state: RenderState) {
    let v0 = Vec3::new(0.0, 1.0, 0.0);
    let v1 = Vec3::new(-1.0, -1.0, 0.0);
    let v2 = Vec3::new(1.0, -1.0, 0.0);

    let mut clip_v0 = state.mvp * Vec4::from_vec3(&v0, 1.0);
    let mut clip_v1 = state.mvp * Vec4::from_vec3(&v1, 1.0);
    let mut clip_v2 = state.mvp * Vec4::from_vec3(&v2, 1.0);

    // perspective devide
    clip_v0 = clip_v0 * (1.0 / clip_v0.w);
    clip_v1 = clip_v1 * (1.0 / clip_v1.w);
    clip_v2 = clip_v2 * (1.0 / clip_v2.w);

    // view_port_transform
    clip_v0.x = (clip_v0.x + 1.0) * 0.5 * WIDTH as f32;
    clip_v0.y = (1.0 - clip_v0.y) * 0.5 * HEIGHT as f32;
    clip_v1.x = (clip_v1.x + 1.0) * 0.5 * WIDTH as f32;
    clip_v1.y = (1.0 - clip_v1.y) * 0.5 * HEIGHT as f32;
    clip_v2.x = (clip_v2.x + 1.0) * 0.5 * WIDTH as f32;
    clip_v2.y = (1.0 - clip_v2.y) * 0.5 * HEIGHT as f32;

    let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    let min_x = clip_v0.x.min(clip_v1.x).min(clip_v2.x).max(0.0) as usize;
    let min_y = clip_v0.y.min(clip_v1.y).min(clip_v2.y).max(0.0) as usize;
    let max_x = clip_v0
        .x
        .max(clip_v1.x)
        .max(clip_v2.x)
        .min((WIDTH - 1) as f32) as usize;
    let max_y = clip_v0
        .y
        .max(clip_v1.y)
        .max(clip_v2.y)
        .min((HEIGHT - 1) as f32) as usize;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let a = Vec2::new(clip_v0.x, clip_v0.y);
            let b = Vec2::new(clip_v1.x, clip_v1.y);
            let c = Vec2::new(clip_v2.x, clip_v2.y);
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

    let projection = Mat4::perspective(60.0, (WIDTH as f32) / (HEIGHT as f32), 0.1, 100.0);
    let view = Mat4::lookat(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let state = RenderState {
        projection,
        view,
        mv: view,
        mvp: projection * view,
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

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

        buffer.clear();
        render(&mut buffer, state);
        // break;
        window
            .update_with_buffer(&buffer.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}
