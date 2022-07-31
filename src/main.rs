extern crate minifb;
mod framebuffer;
mod loader;
mod math;
mod mesh;
mod model;
mod renderstate;
mod texture;
use framebuffer::FrameBuffer;
use math::{Mat4, Quat, Vec2, Vec3, Vec4};
use mesh::{Cube, Triangle};
use minifb::{Key, Window, WindowOptions};
use model::Model;
use renderstate::RenderState;
use texture::Texture;
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn is_top_left(v0: &Vec2, v1: &Vec2) -> bool {
    // any edge that goes up must be an right edge.
    return v1.y <= v0.y;
}

fn orient2d(a: &Vec2, b: &Vec2, c: &Vec2) -> f32 {
    return (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
}

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
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    return Vec3::new(u, v, w);
}

fn render(model: &Model, buffer: &mut FrameBuffer, state: &RenderState) {
    let mesh = &model.mesh;
    println!("{:?}", mesh.num_of_indices());
    for i in (0..mesh.num_of_indices() - 2).step_by(3) {
        let i0 = mesh.indice(i);
        let i1 = mesh.indice(i + 1);
        let i2 = mesh.indice(i + 2);
        // println!("{} {} {}", i, i + 1, i + 2);
        let v0 = mesh.position(i0);
        let v1 = mesh.position(i1);
        let v2 = mesh.position(i2);

        println!("{:?} {:?} {:?} ", v0, v1, v2);

        // let t0 = vertex0.tc.unwrap();
        // let t1 = vertex1.tc.unwrap();
        // let t2 = vertex2.tc.unwrap();

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

        // let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        // let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        // let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
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

        let a = Vec2::new(clip_v0.x + 0.5, clip_v0.y + 0.5);
        let b = Vec2::new(clip_v1.x + 0.5, clip_v1.y + 0.5);
        let c = Vec2::new(clip_v2.x + 0.5, clip_v2.y + 0.5);

        let a01 = a.y - b.y;
        let b01 = b.x - a.x;
        let a12 = b.y - c.y;
        let b12 = c.x - b.x;
        let a20 = c.y - a.y;
        let b20 = a.x - c.x;

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                let w0 = orient2d(&b, &c, &p);
                let w1 = orient2d(&c, &a, &p);
                let w2 = orient2d(&a, &b, &p);
                let bary = barycentric(p, a, b, c);
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let z = bary.x * clip_v0.z + bary.y * clip_v1.z + bary.z * clip_v2.z;
                    // let t = t0 * bary.x + t1 * bary.y + t2 * bary.z;
                    if z < buffer.get_depth(x, y) {
                        buffer.set_depth(x, y, z);
                        // let color = state.albedo.sample_repeat(&t);

                        // println!("{:?}", color);
                        buffer.write_pixel_vec3(x, y, Vec3::new(1.0, 0.0, 0.0));
                    }
                }
                // let e0 = b0 * (p.x - a.x) + c0 * (p.y - a.y);
                // let e1 = b1 * (p.x - b.x) * c1 * (p.y - b.y);
                // let bary = barycentric(p, a, b, c);
                // if bary.x > 0.0 && bary.y >= 0.0 && bary.z >= 0.0 {
                //     let z = bary.x * clip_v0.z + bary.y * clip_v1.z + bary.z * clip_v2.z;
                //     // let t = t0 * bary.x + t1 * bary.y + t2 * bary.z;
                //     if z < buffer.get_depth(x, y) {
                //         buffer.set_depth(x, y, z);
                //         // let color = state.albedo.sample_repeat(&t);

                //         // println!("{:?}", color);
                //         buffer.write_pixel_vec3(x, y, Vec3::new(1.0, 0.0, 0.0));
                //     }
                // }
            }
        }
    }
}

fn main() {
    let mut buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let projection = Mat4::perspective(60.0, (WIDTH as f32) / (HEIGHT as f32), 0.1, 100.0);

    let view = Mat4::lookat(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let cube = Cube::new(1.0);
    let triangle = Triangle::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
    );

    let model = Model {
        mesh: Box::new(cube),
    };

    let albedo = Texture::from_file("asset/2.0/BoxTextured/glTF/CesiumLogoFlat.png");

    let mut state = RenderState {
        projection,
        view,
        mv: view,
        mvp: projection * view,
        albedo,
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
    let mut angles = 0.0f32;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

        let rotation_quat = Quat::from_angle_axis(angles, Vec3::new(1.0, 1.0, 1.0));
        state.mvp = projection * view * (rotation_quat.to_mat4());
        buffer.clear();
        render(&model, &mut buffer, &state);
        // break;
        window
            .update_with_buffer(&buffer.pixels, WIDTH, HEIGHT)
            .unwrap();
        angles += 0.02;
    }
}
